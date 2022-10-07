use std::collections::HashMap;

use crate::timeline::*;
use crate::types::*;

mod ravager;
mod revenant;

lazy_static! {
    static ref ABILITY_MAPPING: HashMap<(String, String), (String, String)> = {
        let mut mapping = HashMap::new();
        mapping.insert(
            ("Ancestry".to_string(), "Intercept".to_string()),
            ("Deathlore".to_string(), "Shield".to_string()),
        );
        mapping.insert(
            ("Ancestry".to_string(), "Shield".to_string()),
            ("Deathlore".to_string(), "Shield".to_string()),
        );
        mapping.insert(
            ("Hyalincuru".to_string(), "Sphere".to_string()),
            ("Tarot".to_string(), "Sun".to_string()),
        );
        mapping.insert(
            ("Hyalincuru".to_string(), "Hypercube".to_string()),
            ("Tarot".to_string(), "Moon".to_string()),
        );
        mapping.insert(
            ("Subjugation".to_string(), "Subdue".to_string()),
            ("Spirituality".to_string(), "Chasten".to_string()),
        );
        mapping.insert(
            ("Subjugation".to_string(), "Ribcage".to_string()),
            ("Spirituality".to_string(), "Aura".to_string()),
        );
        revenant::add_mappings(&mut mapping);
        ravager::add_mappings(&mut mapping);
        mapping
    };
}

pub fn normalize_combat_action(combat_action: &CombatAction) -> CombatAction {
    if let Some((category, skill)) =
        ABILITY_MAPPING.get(&(combat_action.category.clone(), combat_action.skill.clone()))
    {
        CombatAction {
            caster: combat_action.caster.clone(),
            target: combat_action.target.clone(),
            annotation: combat_action.annotation.clone(),
            skill: skill.clone(),
            category: category.clone(),
        }
    } else {
        CombatAction {
            caster: combat_action.caster.clone(),
            target: combat_action.target.clone(),
            annotation: combat_action.annotation.clone(),
            skill: "".to_string(),
            category: "".to_string(),
        }
    }
}
