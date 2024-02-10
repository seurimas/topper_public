use topper_aetolia::timeline::{
    aet_observation_creator, AetObservation, AetPrompt, AetTimeSlice, AetTimeline, AetTimelineTrait,
};
use topper_core::{
    observations::ObservationParser,
    timeline::{
        db::{DatabaseModule, DummyDatabaseModule},
        BaseTimeline,
    },
};

lazy_static! {
    static ref OBSERVATIONS: Vec<String> = {
        let mut results = vec![];
        results.push(include_str!("../../../triggers/Attack Observations.json").to_string());
        results.push(include_str!("../../../triggers/Bard Spoofs.json").to_string());
        results.push(include_str!("../../../triggers/CombatActions.json").to_string());
        results.push(include_str!("../../../triggers/Cures.json").to_string());
        results.push(include_str!("../../../triggers/Hypnosis Spoofs.json").to_string());
        results.push(include_str!("../../../triggers/Indorani Spoof.json").to_string());
        results.push(include_str!("../../../triggers/Luminary Spoof.json").to_string());
        results.push(include_str!("../../../triggers/Observations.json").to_string());
        results.push(include_str!("../../../triggers/Sentinel Spoof.json").to_string());
        results.push(include_str!("../../../triggers/Simple Aff Messages.json").to_string());
        results.push(include_str!("../../../triggers/Subterfuge Spoofs.json").to_string());
        results.push(include_str!("../../../triggers/Predator Spoof.json").to_string());
        results.push(include_str!("../../../triggers/Praekkari Spoofs.json").to_string());
        results.push(include_str!("../../../triggers/Titan Lord Spoofs.json").to_string());
        results.push(include_str!("../../../triggers/Wielding.json").to_string());
        results.push(include_str!("../../../triggers/Writhes.json").to_string());
        results.push(include_str!("../../../triggers/Zealot Spoof.json").to_string());
        results.push(include_str!("../../../triggers/Lists/Diagnose.json").to_string());
        results.push(include_str!("../../../triggers/Lists/Wounds.json").to_string());
        results.push(include_str!("../../../triggers/Lists/AlliesEnemies.json").to_string());
        results.push(include_str!("../../../triggers/Lists/ColdRead.json").to_string());
        results.push(include_str!("../../../triggers/Lists/Pipes.json").to_string());
        results
    };
    pub static ref OBSERVER: ObservationParser<AetObservation> = {
        ObservationParser::<AetObservation>::new_from_strings(
            OBSERVATIONS.to_vec(),
            aet_observation_creator,
        )
        .unwrap()
    };
}
