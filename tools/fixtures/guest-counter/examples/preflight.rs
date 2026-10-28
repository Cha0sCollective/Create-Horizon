//! Native proof consumer for one explicitly prepared paired fixture folder.
use horizon_mods::{
    CounterEvaluator, CounterProposal, CounterRequest, Mods, Runs, Side, Status,
    paired::PreparedBundle,
};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

fn main() -> Result<(), String> {
    let mut args = std::env::args_os().skip(1);
    let path = args.next().map(PathBuf::from)
        .ok_or("usage: preflight <prepared-paired-folder>")?;
    if path.as_os_str().is_empty() || args.next().is_some() {
        return Err("usage: preflight <prepared-paired-folder> (exactly one nonempty argument)".into());
    }

    let prepared = Arc::new(PreparedBundle::read_folder(&path)?);
    // Refuse imports and the wrong diagnostic ABI before any ordinary init runs.
    let mut checker = Mods::default();
    let checked = checker.preflight_pair(&prepared)?;
    for side in [Side::Client, Side::Server] {
        let component = checked.component(side)?;
        let count = component.component_type().imports(component.engine()).count();
        if count != 0 {
            return Err(format!("{side:?} component retains {count} imports"));
        }
    }
    let evaluator = CounterEvaluator::from_pair(&checked)?;
    if evaluator.identity() != prepared.identity() {
        return Err("evaluator identity differs from the captured pair".into());
    }

    let mut mods = Mods::load_for_with_prepared(
        Runs::Both,
        std::slice::from_ref(&path),
        &BTreeMap::from([(path.clone(), prepared.clone())]),
        |_| true,
        |_| false,
    );
    if mods.entries().len() != 1 {
        return Err("expected exactly one paired runtime entry".into());
    }
    if mods.entries()[0].status != Status::Running || !mods.runs_server() {
        return Err(format!("paired activation did not start both halves: {:?}", mods.entries()[0].status));
    }
    if !mods.drain().is_empty() {
        return Err("quiet paired initialization unexpectedly queued output".into());
    }

    let proposal = evaluator.evaluate(&CounterRequest {
        world_instance: 17,
        dimension: "minecraft:overworld".into(),
        type_id: "test:counter".into(),
        schema: 1,
        x: -5,
        y: 80,
        z: 12,
        instance: 23,
        counter: 9,
        game_time: -22,
    })?;
    let expected = CounterProposal {
        world_instance: 17,
        instance: 23,
        game_time: -22,
        expected_counter: 9,
        next_counter: 10,
    };
    if proposal != expected {
        return Err(format!("unexpected full counter proposal: {proposal:?}"));
    }
    if !mods.drain().is_empty() {
        return Err("isolated evaluation unexpectedly queued ordinary mod output".into());
    }
    let identity: String = evaluator.identity().iter().map(|byte| format!("{byte:02x}")).collect();
    println!("paired activation/preflight and scalar evaluation passed: both imports=0, counter=9->10, identity={identity}");
    Ok(())
}
