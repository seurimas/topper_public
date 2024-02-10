use crate::timeline::*;
use std::collections::HashMap;

pub fn add_mappings(mut mapping: &mut HashMap<(String, String), (String, String)>) {
    mapping.insert(
        ("Cultivation".to_string(), "Emanation".to_string()),
        ("Geometrics".to_string(), "Shape".to_string()),
    );
    mapping.insert(
        ("Cultivation".to_string(), "Disgust".to_string()),
        ("Geometrics".to_string(), "Circle".to_string()),
    );
    mapping.insert(
        ("Cultivation".to_string(), "Fear".to_string()),
        ("Geometrics".to_string(), "Square".to_string()),
    );
    mapping.insert(
        ("Cultivation".to_string(), "Joy".to_string()),
        ("Geometrics".to_string(), "Triangle".to_string()),
    );
}
