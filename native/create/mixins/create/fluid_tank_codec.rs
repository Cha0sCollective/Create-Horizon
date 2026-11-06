// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Source-informed one-controller FluidTankBlockEntity TankContent adaptation.
use horizon_world::nbt::{Compound, Tag};

pub(super) const CAPACITY: u32 = 8_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Tank {
    pub amount: u32,
    pub window: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Unsupported(pub &'static str);

pub(super) fn initial() -> Compound {
    Compound::from([
        ("Window".into(), Tag::Byte(1)),
        ("Size".into(), Tag::Int(1)),
        ("Height".into(), Tag::Int(1)),
        ("TankContent".into(), Tag::Compound(Compound::new())),
    ])
}

pub(super) fn read(nbt: &Compound) -> Result<Tank, Unsupported> {
    // The first packet is a single, ordinary controller. Never reinterpret a
    // connected/boiler/opaque tank as a plain empty one.
    if nbt.keys().any(|key| {
        !matches!(
            key.to_string_lossy().as_str(),
            "Window" | "Size" | "Height" | "TankContent" | "Boiler" | "Luminosity" | "components"
        )
    }) {
        return Err(Unsupported("unsupported tank controller or foreign field"));
    }
    let window = match nbt.get(&"Window".into()) {
        Some(Tag::Byte(0)) => false,
        Some(Tag::Byte(1)) => true,
        _ => return Err(Unsupported("invalid tank window")),
    };
    if !matches!(nbt.get(&"Size".into()), Some(Tag::Int(1)))
        || !matches!(nbt.get(&"Height".into()), Some(Tag::Int(1)))
    {
        return Err(Unsupported("tank is not one isolated controller block"));
    }
    if nbt
        .get(&"Boiler".into())
        .is_some_and(|tag| !matches!(tag, Tag::Compound(c) if c.is_empty()))
        || nbt
            .get(&"Luminosity".into())
            .is_some_and(|tag| !matches!(tag, Tag::Int(0)))
    {
        return Err(Unsupported("unsupported tank boiler or luminosity"));
    }
    // A normally placed item supplies an empty native block-entity component
    // envelope. Keep it, but never reinterpret a patched item as a plain tank.
    if nbt
        .get(&"components".into())
        .is_some_and(|tag| !matches!(tag, Tag::Compound(c) if c.is_empty()))
    {
        return Err(Unsupported("unsupported block entity components"));
    }
    let Some(Tag::Compound(content)) = nbt.get(&"TankContent".into()) else {
        return Err(Unsupported("missing tank content"));
    };
    let amount = if content.is_empty() {
        0
    } else {
        if content.len() != 1 {
            return Err(Unsupported("unknown tank content fields"));
        }
        let Some(Tag::Compound(fluid)) = content.get(&"Fluid".into()) else {
            return Err(Unsupported("missing tank fluid"));
        };
        if fluid.keys().any(|key| {
            !matches!(
                key.to_string_lossy().as_str(),
                "id" | "amount" | "components"
            )
        }) || !matches!(fluid.get(&"id".into()), Some(Tag::String(id)) if id.to_string_lossy() == "minecraft:water")
            || !fluid
                .get(&"components".into())
                .is_none_or(|tag| matches!(tag, Tag::Compound(c) if c.is_empty()))
        {
            return Err(Unsupported("unsupported tank fluid or components"));
        }
        match fluid.get(&"amount".into()) {
            Some(Tag::Int(value)) if (1..=CAPACITY as i32).contains(value) => *value as u32,
            _ => return Err(Unsupported("invalid tank water amount")),
        }
    };
    Ok(Tank { amount, window })
}

pub(super) fn set_amount(nbt: &mut Compound, amount: u32) -> Result<bool, Unsupported> {
    let current = read(nbt)?;
    if amount > CAPACITY {
        return Err(Unsupported("tank capacity exceeded"));
    }
    if current.amount == amount {
        return Ok(false);
    }
    let content = if amount == 0 {
        Compound::new() // Explicitly clear client-merged TankContent.
    } else {
        Compound::from([(
            "Fluid".into(),
            Tag::Compound(Compound::from([
                ("id".into(), Tag::String("minecraft:water".into())),
                ("amount".into(), Tag::Int(amount as i32)),
            ])),
        )])
    };
    nbt.insert("TankContent".into(), Tag::Compound(content));
    Ok(true)
}

pub(super) fn toggle_window(nbt: &mut Compound) -> Result<bool, Unsupported> {
    let current = read(nbt)?;
    nbt.insert("Window".into(), Tag::Byte(i8::from(!current.window)));
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_block_controller_roundtrip_and_explicit_empty() {
        let mut nbt = initial();
        assert_eq!(read(&nbt).unwrap().amount, 0);
        assert_eq!(set_amount(&mut nbt, CAPACITY).unwrap(), true);
        assert_eq!(read(&nbt).unwrap().amount, CAPACITY);
        assert_eq!(set_amount(&mut nbt, 0).unwrap(), true);
        assert_eq!(read(&nbt).unwrap().amount, 0);
        assert!(matches!(nbt.get(&"TankContent".into()), Some(Tag::Compound(c)) if c.is_empty()));
    }

    #[test]
    fn foreign_state_is_preserved_without_mutation() {
        let mut nbt = initial();
        nbt.insert("Controller".into(), Tag::Compound(Compound::new()));
        let before = nbt.clone();
        assert!(set_amount(&mut nbt, 1_000).is_err());
        assert_eq!(nbt, before);
    }

    #[test]
    fn ordinary_placed_item_envelope_keeps_empty_components_but_refuses_payloads() {
        // Horizon's BlockItem.applyComponentsFromItemStack inserts this key
        // even when the placed tank item carries no component patch.
        let mut nbt = initial();
        nbt.insert("components".into(), Tag::Compound(Compound::new()));
        assert_eq!(read(&nbt).unwrap().amount, 0);
        assert!(set_amount(&mut nbt, 1_000).unwrap());
        assert!(toggle_window(&mut nbt).unwrap());
        assert_eq!(
            read(&nbt).unwrap(),
            Tank {
                amount: 1_000,
                window: false
            }
        );
        assert!(matches!(nbt.get(&"components".into()), Some(Tag::Compound(c)) if c.is_empty()));
        assert!(set_amount(&mut nbt, 0).unwrap());
        assert!(matches!(nbt.get(&"TankContent".into()), Some(Tag::Compound(c)) if c.is_empty()));
        assert!(set_amount(&mut nbt, 1_000).unwrap());

        nbt.insert(
            "components".into(),
            Tag::Compound(Compound::from([(
                "minecraft:custom_name".into(),
                Tag::String("foreign".into()),
            )])),
        );
        let before = nbt.clone();
        assert!(read(&nbt).is_err());
        assert!(set_amount(&mut nbt, 2_000).is_err());
        assert!(toggle_window(&mut nbt).is_err());
        assert_eq!(nbt, before);

        nbt.insert("components".into(), Tag::Byte(0));
        assert!(read(&nbt).is_err());
    }

    #[test]
    fn filled_window_toggle_preserves_fluid_and_rejects_bad_geometry() {
        let mut nbt = initial();
        set_amount(&mut nbt, 3_500).unwrap();
        toggle_window(&mut nbt).unwrap();
        assert_eq!(
            read(&nbt).unwrap(),
            Tank {
                amount: 3_500,
                window: false
            }
        );
        nbt.insert("Height".into(), Tag::Int(2));
        let before = nbt.clone();
        assert!(set_amount(&mut nbt, 2_000).is_err());
        assert_eq!(nbt, before);
    }
}
