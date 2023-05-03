use crate::timeline::*;
use std::collections::HashMap;

pub fn add_mappings(mut mapping: &mut HashMap<(String, String), (String, String)>) {
    mapping.insert(
        ("Ascendance".to_string(), "Censure".to_string()),
        ("Corpus".to_string(), "Gash".to_string()),
    );
    mapping.insert(
        ("Ascendance".to_string(), "Succour".to_string()),
        ("Corpus".to_string(), "Purify".to_string()),
    );

    mapping.insert(
        ("Dictum".to_string(), "Exhort".to_string()),
        ("Mentis".to_string(), "Mesmerize".to_string()),
    );

    mapping.insert(
        ("Discipline".to_string(), "Light".to_string()),
        ("Sanguis".to_string(), "Trepidation".to_string()),
    );
    mapping.insert(
        ("Discipline".to_string(), "Anathema".to_string()),
        ("Sanguis".to_string(), "Curse".to_string()),
    );
    mapping.insert(
        ("Discipline".to_string(), "Attend".to_string()),
        ("Sanguis".to_string(), "Spew".to_string()),
    );
    mapping.insert(
        ("Discipline".to_string(), "Bane".to_string()),
        ("Sanguis".to_string(), "Poison".to_string()),
    );
}

pub fn map_mentis(combat_action: &CombatAction) -> Option<(String, String)> {
    if combat_action.category == "Dictum" {
        Some(("Mentis".to_string(), combat_action.skill.to_string()))
    } else {
        None
    }
}
