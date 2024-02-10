use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

use crate::timeline::*;
use crate::types::*;

use super::AetTarget;
use super::{BehaviorController, BehaviorModel};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum LimbDescriptor {
    Static(LType),
    NotRestoring(Vec<LType>),
    Highest(Vec<LType>),
    Lowest(Vec<LType>),
    HighestOver(Vec<LType>, f32),
    LowestOver(Vec<LType>, f32),
    Breakable(Vec<(LType, CType)>),
    Random(Vec<LType>),
    FromHint(String),
}

impl LimbDescriptor {
    pub fn get_limb(
        &self,
        model: &BehaviorModel,
        controller: &BehaviorController,
        target: &AetTarget,
    ) -> Option<LType> {
        if let Some(me) = target.get_target(model, controller) {
            match self {
                LimbDescriptor::Static(limb) => Some(*limb),
                LimbDescriptor::NotRestoring(limbs) => {
                    let mut found_restoring = false;
                    let mut not_restoring = None;
                    for limb in limbs {
                        let mut limb_state = me.get_limb_state(*limb);
                        if limb_state.is_restoring {
                            found_restoring = true;
                        } else {
                            not_restoring = Some(*limb);
                        }
                    }
                    if found_restoring {
                        not_restoring
                    } else {
                        None
                    }
                }
                LimbDescriptor::Highest(limbs) => {
                    let mut highest = None;
                    let mut highest_damage = 0.0;
                    for limb in limbs {
                        let mut limb_state = me.get_limb_state(*limb);
                        if limb_state.is_restoring {
                            limb_state.assume_restore();
                        }
                        if limb_state.damage > highest_damage {
                            highest = Some(*limb);
                            highest_damage = limb_state.damage;
                        }
                    }
                    highest
                }
                LimbDescriptor::Lowest(limbs) => {
                    let mut lowest = None;
                    let mut lowest_damage = 100.0;
                    for limb in limbs {
                        let mut limb_state = me.get_limb_state(*limb);
                        if limb_state.is_restoring {
                            limb_state.assume_restore();
                        }
                        if limb_state.damage < lowest_damage {
                            lowest = Some(*limb);
                            lowest_damage = limb_state.damage;
                        }
                    }
                    lowest
                }
                LimbDescriptor::HighestOver(limbs, minimum) => {
                    let mut highest = None;
                    let mut highest_damage = 0.0;
                    for limb in limbs {
                        let mut limb_state = me.get_limb_state(*limb);
                        if limb_state.is_restoring {
                            limb_state.assume_restore();
                        }
                        if limb_state.damage > highest_damage && limb_state.damage > *minimum {
                            highest = Some(*limb);
                            highest_damage = limb_state.damage;
                        }
                    }
                    highest
                }
                LimbDescriptor::LowestOver(limbs, minimum) => {
                    let mut lowest = None;
                    let mut lowest_damage = 100.0;
                    for limb in limbs {
                        let mut limb_state = me.get_limb_state(*limb);
                        if limb_state.is_restoring {
                            limb_state.assume_restore();
                        }
                        if limb_state.damage < lowest_damage && limb_state.damage > *minimum {
                            lowest = Some(*limb);
                            lowest_damage = limb_state.damage;
                        }
                    }
                    lowest
                }
                LimbDescriptor::Breakable(limbs) => {
                    for (limb, available_damage) in limbs {
                        let mut limb_state = me.get_limb_state(*limb);
                        if limb_state.is_restoring {
                            limb_state.assume_restore();
                        }
                        if limb_state.hits_to_break(*available_damage as f32 / 100.) == 1 {
                            return Some(*limb);
                        }
                    }
                    None
                }
                LimbDescriptor::Random(limbs) => {
                    let mut rng = rand::thread_rng();
                    let index = rng.gen_range(0, limbs.len());
                    Some(limbs[index])
                }
                LimbDescriptor::FromHint(hint_name) => {
                    if let Some(hint) = controller.get_hint(hint_name) {
                        Some(LType::from_name(hint))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}
