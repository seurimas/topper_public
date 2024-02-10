use std::ops::DerefMut;

use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::db::DummyDatabaseModule;

use crate::{
    bt::*,
    classes::{Action, Class, FitnessAction, ParryAction},
    db::AetDatabaseModule,
    types::*,
};

#[macro_use]
use crate::with_defense_db;
use super::{
    get_needed_parry, get_needed_refills, get_wanted_dodge, DodgeAction, DEFENSE_DATABASE,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum DefenseBehavior {
    Parry,
    ClassParry(String),
    Repipe,
    Fitness,
    Dodge,
}

impl UnpoweredFunction for DefenseBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            DefenseBehavior::Parry => match DEFENSE_DATABASE.as_ref().try_lock() {
                Ok(outer_guard) => {
                    let option = outer_guard.as_ref();
                    if let Some(inner_mutex) = option {
                        match inner_mutex.as_ref().read() {
                            Ok(db) => {
                                if let Some(limb) = get_needed_parry(
                                    model,
                                    &model.who_am_i(),
                                    &controller.target.clone().unwrap_or_default(),
                                    &"".to_string(),
                                    Some(&*db),
                                ) {
                                    controller.plan.add_to_qeb(Box::new(ParryAction::new(
                                        model.who_am_i(),
                                        limb,
                                    )));
                                };
                            }
                            Err(err) => println!("Could not parry, inner: {:?}", err),
                        }
                    }
                }
                Err(err) => println!("Could not parry: {:?}", err),
            },
            DefenseBehavior::ClassParry(verb) => match DEFENSE_DATABASE.as_ref().try_lock() {
                Ok(outer_guard) => {
                    let option = outer_guard.as_ref();
                    if let Some(inner_mutex) = option {
                        match inner_mutex.as_ref().read() {
                            Ok(db) => {
                                if let Some(limb) = get_needed_parry(
                                    model,
                                    &model.who_am_i(),
                                    &controller.target.clone().unwrap_or_default(),
                                    &"".to_string(),
                                    Some(&*db),
                                ) {
                                    controller.plan.add_to_qeb(Box::new(Action::new(format!(
                                        "{} {}",
                                        verb,
                                        limb.to_string()
                                    ))));
                                };
                            }
                            Err(err) => println!("Could not parry, inner: {:?}", err),
                        }
                    }
                }
                Err(err) => println!("Could not parry: {:?}", err),
            },
            DefenseBehavior::Repipe => {
                let refill_actions = get_needed_refills(&model.state.borrow_me());
                for action in refill_actions {
                    controller.plan.add_to_qeb(Box::new(action));
                }
            }
            DefenseBehavior::Fitness => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::Fitness) {
                    return UnpoweredFunctionState::Complete;
                }
                if me.lock_duration().is_some() {
                    controller
                        .plan
                        .add_to_qeb(Box::new(FitnessAction::new(model.who_am_i())));
                } else {
                    for aggressor in me.aggro.get_aggro_attackers() {
                        let aggressor = model.state.borrow_agent(&aggressor);
                        match aggressor.class_state.get_normalized_class() {
                            Some(Class::Sentinel) => {
                                if me.is(FType::Asthma) && me.is(FType::Slickness) {
                                    controller
                                        .plan
                                        .add_to_qeb(Box::new(FitnessAction::new(model.who_am_i())));
                                    return UnpoweredFunctionState::Complete;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            DefenseBehavior::Dodge => {
                let me = model.state.borrow_me();
                with_defense_db!(db, {
                    let wanted_dodge = get_wanted_dodge(model, Some(&*db));
                    if me.dodge_state.dodge_type != wanted_dodge {
                        controller
                            .plan
                            .add_to_front_of_qeb(Box::new(DodgeAction::new(
                                model.who_am_i(),
                                wanted_dodge,
                            )));
                    }
                })
            }
        }
        UnpoweredFunctionState::Complete
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Do nothing...
    }
}
