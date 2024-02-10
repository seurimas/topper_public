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
}
