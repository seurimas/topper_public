use std::collections::HashMap;

pub fn add_mappings(mut mapping: &mut HashMap<(String, String), (String, String)>) {
    // Riving/Battlefury
    mapping.insert(
        ("Riving".to_string(), "Duplicity".to_string()),
        ("Battlefury".to_string(), "Duality".to_string()),
    );
    mapping.insert(
        ("Chirography".to_string(), "Atdum".to_string()),
        ("Battlefury".to_string(), "Vorpal".to_string()),
    );
}
