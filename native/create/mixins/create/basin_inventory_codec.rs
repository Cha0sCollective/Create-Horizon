// Independently authored from pinned MIT Create schemas and recorded public-API
// observations. This module has no gameplay dependency: save and client receipt
// share the same canonical item decoding.
use crate::containers::{ModBlockEntity, ModBlockEntityFields};
use horizon_items::items::ItemStack;
use horizon_world::nbt::Tag;
use std::ops::ControlFlow;

type Inventories = &'static [(&'static str, usize, usize)];
fn supported_tag(tag: &horizon_world::nbt::Compound) -> bool {
    ["InputItems", "OutputItems"]
        .into_iter()
        .all(|key| match tag.get(&key.into()) {
            None => true,
            Some(Tag::Compound(value)) => {
                matches!(value.get(&"Size".into()), None | Some(Tag::Int(9)))
                    && matches!(value.get(&"Items".into()), None | Some(Tag::List(_)))
            }
            _ => false,
        })
}
pub fn supported(entity: &ModBlockEntity) -> bool {
    entity.id == "create:basin"
        && supported_tag(&entity.nbt)
        && entity.items.iter().skip(18).all(Option::is_none)
}

/// Derive canonical mirrors at the actual serialization boundary, including a
/// hopper mutation after the Basin's tick. Never dirty or change the live BE.
pub fn snapshot(entity: &ModBlockEntity, tag: &mut horizon_world::nbt::Compound) {
    if !supported(entity) {
        return;
    }
    for (key, start) in [("InputItems", 0), ("OutputItems", 9)] {
        let mut half = match entity.nbt.get(&key.into()) {
            Some(Tag::Compound(value)) => value.clone(),
            _ => horizon_world::nbt::Compound::new(),
        };
        half.insert("Size".into(), Tag::Int(9));
        half.insert(
            "Items".into(),
            Tag::List(
                (0..9)
                    .filter_map(|i| {
                        let stack = entity.items.get(start + i)?.as_ref()?;
                        if stack.count == 0 {
                            return None;
                        }
                        let Tag::Compound(mut value) = horizon_items::item_nbt::item_to_nbt(stack)
                        else {
                            return None;
                        };
                        value.insert("Slot".into(), Tag::Int(i as i32));
                        Some(Tag::Compound(value))
                    })
                    .collect(),
            ),
        );
        tag.insert(key.into(), Tag::Compound(half));
    }
}
fn schema(id: &str) -> Option<(usize, Inventories)> {
    Some(match id {
        "create:basin" => (18, &[("InputItems", 0, 9), ("OutputItems", 9, 9)]),
        _ => return None,
    })
}

// None means an unrelated type or malformed root tag. Canonical inventory
// absence within a valid compound means empty slots, including partial packets.
pub fn parse_slots(id: &str, tag: &Tag) -> Option<Vec<Option<ItemStack>>> {
    let (size, inventories) = schema(id)?;
    if !matches!(tag, Tag::Compound(_)) {
        return None;
    }
    // A different explicitly integer-typed size is retained without exposing
    // inventory to ordinary operations, which cannot represent it.
    let mut items = vec![None; size];
    let unsupported = inventories.iter().any(|&(key, _, count)| {
        matches!(tag.get(key).and_then(|b| b.get("Size")),
            Some(Tag::Int(size)) if *size != count as i32)
    });
    if !unsupported {
        if id == "create:depot" {
            items[0] = tag
                .get("HeldItem")
                .and_then(|held| held.get("Item"))
                .and_then(horizon_items::item_nbt::item_from_nbt);
        }

        // The recorded ordinary-buffer cases read missing/non-numeric slots
        // as zero, narrow long slots to 32 bits, and floor floating slots.
        // Range checking happens after conversion, before indexing our cells.
        fn inventory_slot(value: Option<&Tag>) -> i32 {
            match value {
                Some(Tag::Byte(v)) => i32::from(*v),
                Some(Tag::Short(v)) => i32::from(*v),
                Some(Tag::Int(v)) => *v,
                Some(Tag::Long(v)) => *v as i32,
                Some(Tag::Float(v)) => f32::from_bits(*v).floor() as i32,
                Some(Tag::Double(v)) => f64::from_bits(*v).floor() as i32,
                _ => 0,
            }
        }
        for &(key, start, count) in inventories {
            let Some(Tag::List(entries)) = tag.get(key).and_then(|b| b.get("Items")) else {
                continue;
            };
            for entry in entries {
                if let Ok(slot) = usize::try_from(inventory_slot(entry.get("Slot")))
                    && slot < count
                    && let Some(stack) = horizon_items::item_nbt::item_from_nbt(entry)
                {
                    // A later valid entry replaces a duplicate. An invalid
                    // item entry leaves the previous valid stack untouched.
                    items[start + slot] = Some(stack);
                }
            }
        }
    }

    Some(items)
}

pub fn decode(id: &str, tag: &Tag) -> ControlFlow<Option<ModBlockEntity>> {
    if schema(id).is_none() {
        return ControlFlow::Continue(());
    }
    let Tag::Compound(fields) = tag else {
        return ControlFlow::Break(None);
    };
    if !supported_tag(fields) {
        // Let the host retain its generic items and all opaque canonical NBT;
        // the Basin view/processing/interaction guards keep it closed.
        return ControlFlow::Continue(());
    }
    let Some(items) = parse_slots(id, tag) else {
        return ControlFlow::Break(None);
    };
    // Generic Items is a redundant snapshot and cannot revive canonical empties.
    let mut nbt = fields.clone();
    for key in ["id", "x", "y", "z", "Items"] {
        nbt.remove(&key.into());
    }
    ControlFlow::Break(Some(
        ModBlockEntityFields {
            id: id.to_owned(),
            nbt,
            items,
            data: Vec::new(),
        }
        .into(),
    ))
}

// Called before Horizon's ordinary merge_update_tag. Only canonical inventory
// fields are reset on absence; unowned NBT and runtime-only data keep the same
// live entity identity. Slot parsing borrows the packet without cloning its NBT.
pub fn receive(entity: &mut ModBlockEntity, packet_type: &str, block: &str, tag: &Tag) -> bool {
    if matches!(tag, Tag::Compound(fields) if !supported_tag(fields)) {
        return false;
    }
    // Horizon remote_block_entity_data treats an empty update tag as a no-op.
    // This transport rule differs from decoding an empty saved inventory.
    if !matches!(tag, Tag::Compound(fields) if !fields.is_empty()) {
        return false;
    }
    if packet_type != block || entity.id != packet_type {
        return false;
    }
    let Some((_, inventories)) = schema(packet_type) else {
        return false;
    };
    let Some(items) = parse_slots(packet_type, tag) else {
        return false;
    };
    if entity.items != items {
        entity.items = items;
    }
    for key in inventories
        .iter()
        .map(|&(key, _, _)| key)
        .chain((packet_type == "create:depot").then_some("HeldItem"))
    {
        if tag.get(key).is_none() && entity.nbt.contains_key(&key.into()) {
            entity.nbt.remove(&key.into());
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::containers::BlockEntity;
    use horizon_world::nbt::Compound;

    #[test]
    fn immediate_save_uses_live_slots_without_mutating_mirrors_or_opaque_data() {
        let half = Tag::Compound(Compound::from([
            ("Size".into(), Tag::Int(9)),
            ("Items".into(), Tag::List(Vec::new())),
            ("FutureField".into(), Tag::Int(71)),
        ]));
        let mut entity: ModBlockEntity = ModBlockEntityFields {
            id: "create:basin".into(),
            nbt: Compound::from([
                ("InputItems".into(), half.clone()),
                ("OutputItems".into(), half),
                ("Unowned".into(), Tag::Int(99)),
            ]),
            items: vec![None; 18],
            data: Vec::new(),
        }
        .into();
        let identity = entity.revision();
        entity.items[0] = Some(ItemStack::new("minecraft:andesite", 3));
        assert!(identity.same_entity(entity.revision()));
        assert!(!identity.same_entity(entity.clone().revision()));
        let mut block = BlockEntity::Mod(Box::new(entity));
        for occupied in [true, false] {
            let BlockEntity::Mod(entity) = &mut block else {
                unreachable!()
            };
            if !occupied {
                entity.items[0] = None;
            }
            let before = entity.nbt.clone();
            let slots = entity.items.clone();
            let revision = entity.revision();
            let encoded = crate::save::encode_block_entity((6, 65, 5), &block, "create:basin");
            assert_eq!(parse_slots("create:basin", &encoded), Some(slots));
            assert_eq!(encoded.get("Unowned"), Some(&Tag::Int(99)));
            assert_eq!(
                encoded.get("InputItems").and_then(|v| v.get("FutureField")),
                Some(&Tag::Int(71))
            );
            let BlockEntity::Mod(entity) = &block else {
                unreachable!()
            };
            assert_eq!(entity.nbt, before);
            assert_eq!(entity.revision(), revision);
        }
        let BlockEntity::Mod(entity) = &mut block else {
            unreachable!()
        };
        let Some(Tag::Compound(half)) = entity.nbt.get_mut(&"InputItems".into()) else {
            unreachable!()
        };
        half.insert("Size".into(), Tag::Int(17));
        let opaque = entity.nbt[&"InputItems".into()].clone();
        let encoded = crate::save::encode_block_entity((6, 65, 5), &block, "create:basin");
        assert_eq!(encoded.get("InputItems"), Some(&opaque));
        assert!(matches!(
            decode("create:basin", &encoded),
            ControlFlow::Continue(())
        ));
    }
}
