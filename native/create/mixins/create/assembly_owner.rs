// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
// Private, inert entity-owner foundation. No source block or entity is changed here.

use super::Game;
use horizon_world::{
    attachments::{AttachmentCodec, AttachmentError, AttachmentType, Attachments, ATTACHMENTS_KEY},
    chunk::BlockState,
    nbt::{Compound, Tag},
};
use std::collections::{BTreeSet, HashMap, HashSet};

const MARKER: &str = "create_horizon:assembly_owner_v1";
const MARKER_VALUE: &str = "create_horizon:assembly_owner:1";
const NAMESPACE: &str = "create_horizon";
const RECORD_NAME: &str = "assembly_record_v1";
const VERSION: i32 = 1;
const CELL_LIMIT: usize = 2;
const DISPLAY: &str = "minecraft:block_display";

/// Independent of both the carrier's persistent UUID and its loaded numeric ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct AssemblyId(pub(super) [i32; 4]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Phase {
    Preparing,
    Captured,
    Moving,
    RestorePending,
    Settled,
}

impl Phase {
    fn name(self) -> &'static str {
        match self {
            Self::Preparing => "preparing",
            Self::Captured => "captured",
            Self::Moving => "moving",
            Self::RestorePending => "restore_pending",
            Self::Settled => "settled",
        }
    }
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "preparing" => Self::Preparing,
            "captured" => Self::Captured,
            "moving" => Self::Moving,
            "restore_pending" => Self::RestorePending,
            "settled" => Self::Settled,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Pose {
    pub(super) tick: i64,
    pub(super) angle_degrees: f64,
}

/// Last *verified* identity. Unknown is distinct from observed air.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Observed {
    Unknown,
    Air,
    State(BlockState),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Cell {
    pub(super) local: [i32; 3],
    pub(super) state: BlockState,
    pub(super) source_removed: bool,
    pub(super) target_placed: bool,
    pub(super) last_source: Observed,
    pub(super) last_target: Observed,
}

/// V1 accepts only a bearing with two inert state-only cells. The vector and
/// explicit payload tag keep later cell contracts separate from this limit.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct AssemblyRecord {
    pub(super) id: AssemblyId,
    pub(super) carrier_uuid: [i32; 4],
    pub(super) dimension: String,
    pub(super) controller: [i32; 3],
    pub(super) controller_kind: String,
    pub(super) facing: String,
    pub(super) anchor: [i32; 3],
    pub(super) axis: String,
    pub(super) cells: Vec<Cell>,
    pub(super) previous: Pose,
    pub(super) current: Pose,
    pub(super) phase: Phase,
    pub(super) removal_requested: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ViewCell {
    pub(super) local: [i32; 3],
    pub(super) state: BlockState,
}

/// A projection only; it cannot authorize removal, motion or restoration.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct AssemblyView {
    pub(super) id: AssemblyId,
    pub(super) anchor: [i32; 3],
    pub(super) axis: String,
    pub(super) cells: Vec<ViewCell>,
    pub(super) previous: Pose,
    pub(super) current: Pose,
    pub(super) phase: Phase,
}

impl AssemblyView {
    pub(super) fn from_record(record: &AssemblyRecord) -> Result<Self, AttachmentError> {
        validate_record(record)?;
        Ok(Self {
            id: record.id,
            anchor: record.anchor,
            axis: record.axis.clone(),
            cells: record
                .cells
                .iter()
                .map(|cell| ViewCell {
                    local: cell.local,
                    state: cell.state.clone(),
                })
                .collect(),
            previous: record.previous,
            current: record.current,
            phase: record.phase,
        })
    }
}

fn absent_record() -> AssemblyRecord {
    AssemblyRecord {
        id: AssemblyId([0; 4]),
        carrier_uuid: [0; 4],
        dimension: String::new(),
        controller: [0; 3],
        controller_kind: String::new(),
        facing: String::new(),
        anchor: [0; 3],
        axis: String::new(),
        cells: Vec::new(),
        previous: Pose { tick: 0, angle_degrees: 0.0 },
        current: Pose { tick: 0, angle_degrees: 0.0 },
        phase: Phase::Preparing,
        removal_requested: false,
    }
}

fn absent_view() -> AssemblyView {
    AssemblyView {
        id: AssemblyId([0; 4]),
        anchor: [0; 3],
        axis: String::new(),
        cells: Vec::new(),
        previous: Pose { tick: 0, angle_degrees: 0.0 },
        current: Pose { tick: 0, angle_degrees: 0.0 },
        phase: Phase::Preparing,
    }
}

pub(super) static RECORD: AttachmentType<AssemblyRecord> = AttachmentType {
    namespace: NAMESPACE,
    name: RECORD_NAME,
    default: absent_record,
    codec: Some(AttachmentCodec { encode: encode_record, decode: decode_record }),
    copy_on_death: false,
    sync: false,
};

pub(super) static VIEW: AttachmentType<AssemblyView> = AttachmentType {
    namespace: NAMESPACE,
    name: "assembly_view_v1",
    default: absent_view,
    codec: Some(AttachmentCodec { encode: encode_view, decode: decode_view }),
    copy_on_death: false,
    sync: true,
};

fn bad() -> AttachmentError { AttachmentError::Decode }
fn key(c: &Compound, name: &str) -> Result<&Tag, AttachmentError> {
    c.get(&name.into()).ok_or_else(bad)
}
fn fields(c: &Compound, expected: &[&str]) -> Result<(), AttachmentError> {
    let actual: BTreeSet<_> = c.keys().map(|key| key.to_string().map_err(|_| bad())).collect::<Result<_, _>>()?;
    let expected: BTreeSet<_> = expected.iter().map(|s| (*s).to_owned()).collect();
    (actual == expected).then_some(()).ok_or_else(bad)
}
fn compound(tag: &Tag) -> Result<&Compound, AttachmentError> {
    match tag { Tag::Compound(c) => Ok(c), _ => Err(bad()) }
}
fn string(tag: &Tag) -> Result<String, AttachmentError> {
    match tag { Tag::String(s) => s.to_string().map_err(|_| bad()), _ => Err(bad()) }
}
fn int(tag: &Tag) -> Result<i32, AttachmentError> {
    match tag { Tag::Int(n) => Ok(*n), _ => Err(bad()) }
}
fn bool_tag(tag: &Tag) -> Result<bool, AttachmentError> {
    match tag { Tag::Byte(0) => Ok(false), Tag::Byte(1) => Ok(true), _ => Err(bad()) }
}
fn position(tag: &Tag) -> Result<[i32; 3], AttachmentError> {
    match tag { Tag::IntArray(v) if v.len() == 3 => Ok([v[0],v[1],v[2]]), _ => Err(bad()) }
}
fn identity(tag: &Tag) -> Result<[i32; 4], AttachmentError> {
    match tag { Tag::IntArray(v) if v.len() == 4 && v.iter().any(|n| *n != 0) => Ok([v[0],v[1],v[2],v[3]]), _ => Err(bad()) }
}
fn pose_tag(p: Pose) -> Tag {
    Tag::Compound([("tick".into(),Tag::Long(p.tick)),("angle".into(),Tag::Double(p.angle_degrees.to_bits()))].into())
}
fn pose(tag: &Tag) -> Result<Pose, AttachmentError> {
    let c = compound(tag)?;
    fields(c, &["tick", "angle"])?;
    let Tag::Long(tick) = key(c,"tick")? else { return Err(bad()) };
    let Tag::Double(bits) = key(c,"angle")? else { return Err(bad()) };
    let result = Pose { tick: *tick, angle_degrees: f64::from_bits(*bits) };
    (result.tick >= 0 && result.angle_degrees.is_finite()).then_some(result).ok_or_else(bad)
}
fn observed_tag(value: &Observed) -> Tag {
    let mut c = Compound::new();
    match value {
        Observed::Unknown => { c.insert("kind".into(),Tag::String("unknown".into())); }
        Observed::Air => { c.insert("kind".into(),Tag::String("air".into())); }
        Observed::State(state) => {
            c.insert("kind".into(),Tag::String("state".into()));
            c.insert("state".into(),state.to_nbt());
        }
    }
    Tag::Compound(c)
}
fn observed(tag: &Tag) -> Result<Observed, AttachmentError> {
    let c = compound(tag)?;
    match string(key(c,"kind")?)?.as_str() {
        "unknown" => { fields(c,&["kind"])?; Ok(Observed::Unknown) }
        "air" => { fields(c,&["kind"])?; Ok(Observed::Air) }
        "state" => { fields(c,&["kind","state"])?; Ok(Observed::State(state(key(c,"state")?)?)) }
        _ => Err(bad()),
    }
}
fn state(tag: &Tag) -> Result<BlockState, AttachmentError> {
    let c = compound(tag)?;
    if c.contains_key(&"properties".into()) { fields(c,&["id","properties"])?; }
    else { fields(c,&["id"])?; }
    let state = BlockState::from_nbt(tag).map_err(|_| bad())?;
    valid_state(&state).then_some(state).ok_or_else(bad)
}
fn valid_state(state: &BlockState) -> bool {
    match state.name.as_str() {
        "minecraft:slime_block" => state.properties.is_empty(),
        "minecraft:oak_log" => state.properties.len() == 1 && matches!(state.properties.get("axis").map(String::as_str),Some("x"|"y"|"z")),
        _ => false,
    }
}
fn cell_tag(cell: &Cell) -> Tag {
    Tag::Compound([
        ("local".into(),Tag::IntArray(cell.local.to_vec())),
        ("state".into(),cell.state.to_nbt()),
        ("payload".into(),Tag::String("state_only".into())),
        ("source_removed".into(),Tag::Byte(if cell.source_removed { 1 } else { 0 })),
        ("target_placed".into(),Tag::Byte(if cell.target_placed { 1 } else { 0 })),
        ("last_source".into(),observed_tag(&cell.last_source)),
        ("last_target".into(),observed_tag(&cell.last_target)),
    ].into())
}
fn cell(tag: &Tag) -> Result<Cell, AttachmentError> {
    let c = compound(tag)?;
    fields(c,&["local","state","payload","source_removed","target_placed","last_source","last_target"])?;
    if string(key(c,"payload")?)? != "state_only" { return Err(bad()); }
    Ok(Cell {
        local: position(key(c,"local")?)?, state: state(key(c,"state")?)?,
        source_removed: bool_tag(key(c,"source_removed")?)?,
        target_placed: bool_tag(key(c,"target_placed")?)?,
        last_source: observed(key(c,"last_source")?)?,
        last_target: observed(key(c,"last_target")?)?,
    })
}
fn view_cell_tag(cell: &ViewCell) -> Tag {
    Tag::Compound([("local".into(),Tag::IntArray(cell.local.to_vec())),("state".into(),cell.state.to_nbt())].into())
}
fn view_cell(tag: &Tag) -> Result<ViewCell, AttachmentError> {
    let c=compound(tag)?; fields(c,&["local","state"])?;
    Ok(ViewCell { local: position(key(c,"local")?)?, state: state(key(c,"state")?)? })
}
fn cells(tag: &Tag) -> Result<&[Tag], AttachmentError> {
    match tag { Tag::List(v) if v.len() == CELL_LIMIT => Ok(v), _ => Err(bad()) }
}
fn phase(tag: &Tag) -> Result<Phase, AttachmentError> {
    Phase::parse(&string(tag)?).ok_or_else(bad)
}
fn valid_cells<'a>(cells: impl IntoIterator<Item=([i32;3], &'a BlockState)>) -> bool {
    let cells: Vec<_> = cells.into_iter().collect();
    cells.len() == CELL_LIMIT
        && cells.iter().map(|(local,_)| *local).collect::<HashSet<_>>().len() == CELL_LIMIT
        && cells.iter().any(|(local,state)| *local == [0,0,0] && state.name == "minecraft:slime_block" && valid_state(state))
        && cells.iter().any(|(local,state)| matches!(*local, [1,0,0]|[-1,0,0]|[0,1,0]|[0,-1,0]|[0,0,1]|[0,0,-1]) && state.name == "minecraft:oak_log" && valid_state(state))
}
fn valid_pose(previous: Pose, current: Pose) -> bool {
    previous.tick >= 0 && current.tick >= previous.tick && previous.angle_degrees.is_finite() && current.angle_degrees.is_finite()
}
fn valid_axis(axis: &str) -> bool { matches!(axis,"x"|"y"|"z") }
fn facing_axis(facing: &str) -> Option<&'static str> {
    match facing { "up"|"down" => Some("y"), "north"|"south" => Some("z"), "east"|"west" => Some("x"), _ => None }
}
fn facing_step(facing: &str) -> Option<[i32;3]> {
    Some(match facing { "up"=>[0,1,0], "down"=>[0,-1,0], "north"=>[0,0,-1], "south"=>[0,0,1], "east"=>[1,0,0], "west"=>[-1,0,0], _=>return None })
}
fn validate_record(r: &AssemblyRecord) -> Result<(), AttachmentError> {
    if r.id.0 == [0;4] || r.carrier_uuid == [0;4] || r.dimension.is_empty()
        || r.controller_kind != "create:mechanical_bearing" || facing_axis(&r.facing) != Some(r.axis.as_str())
        || !valid_pose(r.previous,r.current) || !valid_cells(r.cells.iter().map(|c|(c.local,&c.state)))
    { return Err(bad()); }
    let step=facing_step(&r.facing).ok_or_else(bad)?;
    if r.controller.iter().zip(step).zip(r.anchor).any(|((&a,b),target)| a.checked_add(b) != Some(target)) { return Err(bad()); }
    if r.cells.iter().any(|c| c.source_removed && c.last_source != Observed::Air) { return Err(bad()); }
    if r.cells.iter().any(|c| c.target_placed && !matches!(&c.last_target, Observed::State(state) if state.name == c.state.name)) { return Err(bad()); }
    if matches!(r.phase,Phase::Captured|Phase::Moving|Phase::RestorePending|Phase::Settled)
        && r.cells.iter().any(|c| !c.source_removed) { return Err(bad()); }
    if r.phase == Phase::Settled && (r.cells.iter().any(|c| !c.target_placed) || r.removal_requested) { return Err(bad()); }
    Ok(())
}
fn validate_view(v: &AssemblyView) -> Result<(), AttachmentError> {
    (v.id.0 != [0;4] && valid_axis(&v.axis) && valid_pose(v.previous,v.current)
        && valid_cells(v.cells.iter().map(|c|(c.local,&c.state))))
        .then_some(()).ok_or_else(bad)
}
fn encode_record(r: &AssemblyRecord) -> Result<Compound, AttachmentError> {
    validate_record(r).map_err(|_| AttachmentError::Encode)?;
    Ok([
        ("version".into(),Tag::Int(VERSION)),
        ("assembly_id".into(),Tag::IntArray(r.id.0.to_vec())),
        ("carrier_uuid".into(),Tag::IntArray(r.carrier_uuid.to_vec())),
        ("dimension".into(),Tag::String(r.dimension.as_str().into())),
        ("controller".into(),Tag::IntArray(r.controller.to_vec())),
        ("controller_kind".into(),Tag::String(r.controller_kind.as_str().into())),
        ("facing".into(),Tag::String(r.facing.as_str().into())),
        ("drive".into(),Tag::String("bearing".into())),
        ("anchor".into(),Tag::IntArray(r.anchor.to_vec())),
        ("axis".into(),Tag::String(r.axis.as_str().into())),
        ("cells".into(),Tag::List(r.cells.iter().map(cell_tag).collect())),
        ("previous".into(),pose_tag(r.previous)),
        ("current".into(),pose_tag(r.current)),
        ("phase".into(),Tag::String(r.phase.name().into())),
        ("removal_requested".into(),Tag::Byte(if r.removal_requested { 1 } else { 0 })),
    ].into())
}
fn decode_record(c: &Compound) -> Result<AssemblyRecord, AttachmentError> {
    fields(c,&["version","assembly_id","carrier_uuid","dimension","controller","controller_kind","facing","drive","anchor","axis","cells","previous","current","phase","removal_requested"])?;
    if int(key(c,"version")?)? != VERSION || string(key(c,"drive")?)? != "bearing" { return Err(bad()); }
    let r=AssemblyRecord {
        id: AssemblyId(identity(key(c,"assembly_id")?)?),
        carrier_uuid: identity(key(c,"carrier_uuid")?)?,
        dimension: string(key(c,"dimension")?)?,
        controller: position(key(c,"controller")?)?,
        controller_kind: string(key(c,"controller_kind")?)?,
        facing: string(key(c,"facing")?)?,
        anchor: position(key(c,"anchor")?)?,
        axis: string(key(c,"axis")?)?,
        cells: cells(key(c,"cells")?)?.iter().map(cell).collect::<Result<_,_>>()?,
        previous: pose(key(c,"previous")?)?,
        current: pose(key(c,"current")?)?,
        phase: phase(key(c,"phase")?)?,
        removal_requested: bool_tag(key(c,"removal_requested")?)?,
    };
    validate_record(&r)?; Ok(r)
}
fn encode_view(v: &AssemblyView) -> Result<Compound, AttachmentError> {
    validate_view(v).map_err(|_| AttachmentError::Encode)?;
    Ok([
        ("version".into(),Tag::Int(VERSION)),
        ("assembly_id".into(),Tag::IntArray(v.id.0.to_vec())),
        ("anchor".into(),Tag::IntArray(v.anchor.to_vec())),
        ("axis".into(),Tag::String(v.axis.as_str().into())),
        ("cells".into(),Tag::List(v.cells.iter().map(view_cell_tag).collect())),
        ("previous".into(),pose_tag(v.previous)),
        ("current".into(),pose_tag(v.current)),
        ("phase".into(),Tag::String(v.phase.name().into())),
    ].into())
}
fn decode_view(c: &Compound) -> Result<AssemblyView, AttachmentError> {
    fields(c,&["version","assembly_id","anchor","axis","cells","previous","current","phase"])?;
    if int(key(c,"version")?)? != VERSION { return Err(bad()); }
    let v=AssemblyView {
        id: AssemblyId(identity(key(c,"assembly_id")?)?),
        anchor: position(key(c,"anchor")?)?,
        axis: string(key(c,"axis")?)?,
        cells: cells(key(c,"cells")?)?.iter().map(view_cell).collect::<Result<_,_>>()?,
        previous: pose(key(c,"previous")?)?,
        current: pose(key(c,"current")?)?,
        phase: phase(key(c,"phase")?)?,
    };
    validate_view(&v)?; Ok(v)
}

#[derive(Clone, Debug, PartialEq)]
enum Owner {
    None,
    Active(AssemblyRecord),
    Retired,
    Corrupt,
}

fn raw_record_present(raw: Option<&Tag>) -> bool {
    raw.and_then(|tag| tag.get(ATTACHMENTS_KEY))
        .and_then(|tag| tag.get(NAMESPACE))
        .and_then(|tag| tag.get(RECORD_NAME)).is_some()
}

/// Does not create an attachment map or UUID. Cloning permits lazy decode
/// without mutating the authoritative map or queuing a sync.
fn inspect_owner(kind: &str, raw: Option<&Tag>, uuid: Option<[i32;4]>, map: Option<&Attachments>) -> Owner {
    let marker=raw.and_then(|tag|tag.get(MARKER));
    let has_record=map.is_some_and(|m|m.has(&RECORD)) || raw_record_present(raw);
    if marker.is_none() && !has_record { return Owner::None; }
    if kind != DISPLAY || !matches!(marker,Some(Tag::String(s)) if s.to_string().ok().as_deref()==Some(MARKER_VALUE)) {
        return Owner::Corrupt;
    }
    let Some(uuid)=uuid else { return Owner::Corrupt; };
    let Some(map)=map else { return Owner::Corrupt; };
    let Ok(Some(record))=map.clone().get_existing(&RECORD) else { return Owner::Corrupt; };
    if record.carrier_uuid != uuid { return Owner::Corrupt; }
    if record.phase == Phase::Settled { Owner::Retired } else { Owner::Active(record) }
}

/// `true` means an attempted explicit removal or external relocation must be
/// refused. Ordinary displays and a fully settled owner return `false`.
pub(super) fn guard_active_carrier(game: &Game<'_>, id: u32) -> bool {
    let Some(entity)=game.others.iter().find(|other|other.id==id) else { return false; };
    let uuid=game.persist.uuids.get(&id).copied();
    !matches!(inspect_owner(&entity.kind,game.persist.raw.get(&id),uuid,uuid.and_then(|u|game.entity_attachments.get(&u))),Owner::None|Owner::Retired)
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(super) struct Scan {
    pub(super) active: Vec<(AssemblyId,u32)>,
    pub(super) retired: Vec<u32>,
    pub(super) refused: Vec<u32>,
}

/// Read-only loaded-level discovery. The caller may report `refused`; it must
/// never repair or advance a record based solely on this scan.
pub(super) fn scan_loaded_owners(game: &Game<'_>, dimension: &str) -> Scan {
    let mut result=Scan::default();
    if !game.native_authoritative() || dimension != game.dim.id() { return result; }
    let mut candidates=Vec::new();
    for entity in &game.others {
        let uuid=game.persist.uuids.get(&entity.id).copied();
        let owner=inspect_owner(&entity.kind,game.persist.raw.get(&entity.id),uuid,uuid.and_then(|u|game.entity_attachments.get(&u)));
        match owner {
            Owner::None => (),
            Owner::Retired => result.retired.push(entity.id),
            Owner::Corrupt => result.refused.push(entity.id),
            Owner::Active(record) => {
                let p=entity.position;
                let loaded=p.x.is_finite() && p.z.is_finite() && p.x >= f64::from(i32::MIN) && p.x < f64::from(i32::MAX)
                    && p.z >= f64::from(i32::MIN) && p.z < f64::from(i32::MAX)
                    && game.world.loaded(p.x.floor() as i32,p.z.floor() as i32);
                let positions: Option<Vec<_>>=record.cells.iter().map(|c| {
                    Some([record.anchor[0].checked_add(c.local[0])?,record.anchor[1].checked_add(c.local[1])?,record.anchor[2].checked_add(c.local[2])?])
                }).collect();
                let valid=loaded && record.dimension == dimension && positions.as_ref().is_some_and(|positions| {
                    positions.iter().all(|pos| game.world.loaded(pos[0],pos[2]) && (pos[0]>>4,pos[2]>>4)==((p.x.floor() as i32)>>4,(p.z.floor() as i32)>>4))
                });
                if !valid {
                    result.refused.push(entity.id);
                } else {
                    candidates.push((record.id,entity.id,positions.unwrap()));
                }
            }
        }
    }
    let mut id_counts=HashMap::new();
    let mut source_counts=HashMap::new();
    for (assembly,_,positions) in &candidates {
        *id_counts.entry(*assembly).or_insert(0usize)+=1;
        for pos in positions { *source_counts.entry(*pos).or_insert(0usize)+=1; }
    }
    for (assembly,entity,positions) in candidates {
        if id_counts[&assembly] != 1 || positions.iter().any(|pos|source_counts[pos] != 1) {
            result.refused.push(entity);
        } else {
            result.active.push((assembly,entity));
        }
    }
    result
}

/// Hook body for `level_tick_post`: `create_assembly_owner::level_tick_post(game, dimension)`.
/// It deliberately performs no world, entity, attachment or sync mutation.
pub(super) fn level_tick_post(game: &Game<'_>, dimension: &str) {
    let _ = scan_loaded_owners(game, dimension);
}

#[cfg(test)]
mod tests {
    use super::*;
    use horizon_world::chunk::BlockProperties;

    fn state(name: &str, axis: Option<&str>) -> BlockState {
        let mut properties=BlockProperties::new();
        if let Some(axis)=axis { properties.insert("axis".into(),axis.into()); }
        BlockState { name:name.into(), properties }
    }
    fn fixture() -> AssemblyRecord {
        let cell=|local,state| Cell { local,state,source_removed:false,target_placed:false,last_source:Observed::Unknown,last_target:Observed::Unknown };
        AssemblyRecord {
            id:AssemblyId([1,2,3,4]),carrier_uuid:[5,6,7,8],dimension:"minecraft:overworld".into(),
            controller:[8,63,8],controller_kind:"create:mechanical_bearing".into(),facing:"up".into(),
            anchor:[8,64,8],axis:"y".into(),
            cells:vec![cell([0,0,0],state("minecraft:slime_block",None)),cell([0,0,1],state("minecraft:oak_log",Some("z")))],
            previous:Pose{tick:8,angle_degrees:0.0},current:Pose{tick:9,angle_degrees:86.4},
            phase:Phase::Preparing,removal_requested:false,
        }
    }
    fn raw_marker() -> Tag {
        Tag::Compound([(MARKER.into(),Tag::String(MARKER_VALUE.into()))].into())
    }
    #[test]
    fn strict_record_and_projection_round_trip_through_attachment_storage() {
        let record=fixture();
        let view=AssemblyView::from_record(&record).unwrap();
        let mut map=Attachments::default();
        map.set(&RECORD,record.clone()).unwrap();
        map.set(&VIEW,view.clone()).unwrap();
        assert!(map.take_sync_changes().iter().all(|change|change.name != RECORD_NAME.into()));
        let mut loaded=Attachments::from_nbt(map.to_nbt());
        assert_eq!(loaded.get_existing(&RECORD),Ok(Some(record)));
        assert_eq!(loaded.get_existing(&VIEW),Ok(Some(view)));
    }
    #[test]
    fn schema_rejects_unknown_version_payload_duplicate_and_invalid_pose() {
        let mut encoded=encode_record(&fixture()).unwrap();
        encoded.insert("version".into(),Tag::Int(2));
        assert_eq!(decode_record(&encoded),Err(AttachmentError::Decode));
        encoded.insert("version".into(),Tag::Int(1));
        if let Some(Tag::List(cells))=encoded.get_mut(&"cells".into()) {
            if let Tag::Compound(cell)=&mut cells[0] { cell.insert("payload".into(),Tag::String("inventory".into())); }
        }
        assert_eq!(decode_record(&encoded),Err(AttachmentError::Decode));
        let mut record=fixture();
        record.cells[1].local=[0,0,0];
        assert_eq!(encode_record(&record),Err(AttachmentError::Encode));
        record=fixture(); record.current.angle_degrees=f64::NAN;
        assert_eq!(encode_record(&record),Err(AttachmentError::Encode));
    }
    #[test]
    fn exact_marker_uuid_and_retirement_control_read_only_guard_decision() {
        let record=fixture();
        let mut map=Attachments::default(); map.set(&RECORD,record.clone()).unwrap();
        let raw=raw_marker();
        assert!(matches!(inspect_owner(DISPLAY,Some(&raw),Some(record.carrier_uuid),Some(&map)),Owner::Active(_)));
        assert_eq!(inspect_owner(DISPLAY,None,Some(record.carrier_uuid),None),Owner::None);
        assert_eq!(inspect_owner(DISPLAY,Some(&raw),Some([9,9,9,9]),Some(&map)),Owner::Corrupt);
        assert_eq!(inspect_owner(DISPLAY,Some(&raw),None,Some(&map)),Owner::Corrupt);
        let mut retired=record;
        retired.phase=Phase::Settled;
        for cell in &mut retired.cells { cell.source_removed=true;cell.last_source=Observed::Air;cell.target_placed=true;cell.last_target=Observed::State(cell.state.clone()); }
        map.set(&RECORD,retired).unwrap();
        assert_eq!(inspect_owner(DISPLAY,Some(&raw),Some([5,6,7,8]),Some(&map)),Owner::Retired);
    }
}
