use std::f32::consts::E;

use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::*,
    classes::{get_venoms_from_plan, VenomType},
    items::WieldAction,
    types::*,
    with_defense_db,
};

use super::{actions::*, add_delphs, get_top_suggestion, BEDAZZLE_AFFS};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum FlayType {
    None,
    Shield,
    Rebounding,
    Cloak,
    Speed,
    Fangbarrier,
}

impl FlayType {
    pub fn get_annotation(&self) -> String {
        match self {
            FlayType::None => "".to_string(),
            FlayType::Shield => "shield".to_string(),
            FlayType::Rebounding => "rebounding".to_string(),
            FlayType::Cloak => "cloak".to_string(),
            FlayType::Speed => "speed".to_string(),
            FlayType::Fangbarrier => "fangbarrier".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum BiteType {
    Scytherus,
    Camus,
    Loki,
    Stack,
}

impl BiteType {
    pub fn get_venom(&self) -> VenomType {
        match self {
            BiteType::Scytherus => "scytherus",
            BiteType::Camus => "camus",
            BiteType::Loki => "loki",
            BiteType::Stack => panic!("Cannot get venom for stack"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SleightType {
    Dissipate,
    Abrasion,
    Invasion,
    Blank,
    Void,
    Pall,
}

impl SleightType {
    pub fn get_annotation(&self) -> String {
        match self {
            SleightType::Dissipate => "dissipate".to_string(),
            SleightType::Abrasion => "abrasion".to_string(),
            SleightType::Invasion => "invasion".to_string(),
            SleightType::Blank => "blank".to_string(),
            SleightType::Void => "void".to_string(),
            SleightType::Pall => "pall".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum InfiltratorBehavior {
    AspDoublestab(AetTarget),
    DelphDoublestab(AetTarget),
    Doublestab(AetTarget),
    Flay(AetTarget, FlayType),
    Slit(AetTarget),
    Bedazzle(AetTarget),
    StackSuggest(AetTarget),
    Seal(AetTarget),
    ShrugVenom,
    Bind(AetTarget),
    Bite(AetTarget, BiteType),
    Garrote(AetTarget),
    Sleight(AetTarget, SleightType),
}

impl UnpoweredFunction for InfiltratorBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            InfiltratorBehavior::Bedazzle(target) => {
                if let (me, Some(you)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                ) {
                    if !me.arm_free() {
                        return UnpoweredFunctionState::Failed;
                    } else if you.affs_count(&BEDAZZLE_AFFS.to_vec()) >= 5 {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(BedazzleAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::Bind(target) => {
                if let (me, Some(you)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                ) {
                    if !me.arm_free() {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::WritheBind) || !you.is(FType::Asleep) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(BindAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::Bite(target, bite_type) => {
                if let (me, Some(you), Some(aff_stack)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                    controller.aff_priorities.clone(),
                ) {
                    if !me.arm_free() {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Fangbarrier) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if bite_type == &BiteType::Stack {
                        let venoms = get_venoms_from_plan(&aff_stack.to_vec(), 1, &you);
                        if venoms.len() < 1 {
                            return UnpoweredFunctionState::Failed;
                        }
                        controller.plan.add_to_qeb(Box::new(BiteAction::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                            venoms[0],
                        )));
                    } else {
                        controller.plan.add_to_qeb(Box::new(BiteAction::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                            bite_type.get_venom(),
                        )));
                    }
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::AspDoublestab(aet_target) => {
                if let (me, Some(you), Some(venom_plan)) = (
                    model.state.borrow_me(),
                    aet_target.get_target(model, controller),
                    controller.aff_priorities.clone(),
                ) {
                    if !assure_wielded(&me, model, controller, "dirk", false) {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Rebounding) || you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venoms = get_venoms_from_plan(&venom_plan.to_vec(), 1, &you);
                    if venoms.len() < 1 {
                        return UnpoweredFunctionState::Failed;
                    }
                    let v1 = venoms[0];
                    controller
                        .plan
                        .add_to_qeb(Box::new(DoublestabAction::new_asp(
                            model.who_am_i(),
                            aet_target.get_name(model, controller),
                            v1,
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::DelphDoublestab(aet_target) => {
                if let (me, Some(you), Some(venom_plan)) = (
                    model.state.borrow_me(),
                    aet_target.get_target(model, controller),
                    controller.aff_priorities.clone(),
                ) {
                    if !assure_wielded(&me, model, controller, "dirk", false) {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Rebounding) || you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let mut venoms = get_venoms_from_plan(&venom_plan.to_vec(), 2, &you);
                    if !add_delphs(you, &mut venoms, 2) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if venoms.len() < 2 {
                        return UnpoweredFunctionState::Failed;
                    }
                    let v1 = venoms[1];
                    let v2 = venoms[0];
                    controller.plan.add_to_qeb(Box::new(DoublestabAction::new(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                        v1,
                        v2,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::Doublestab(aet_target) => {
                if let (me, Some(you), Some(venom_plan)) = (
                    model.state.borrow_me(),
                    aet_target.get_target(model, controller),
                    controller.aff_priorities.clone(),
                ) {
                    if !assure_wielded(&me, model, controller, "dirk", false) {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Rebounding) || you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venoms = get_venoms_from_plan(&venom_plan.to_vec(), 2, &you);
                    if venoms.len() < 2 {
                        return UnpoweredFunctionState::Failed;
                    }
                    let v1 = venoms[1];
                    let v2 = venoms[0];
                    controller.plan.add_to_qeb(Box::new(DoublestabAction::new(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                        v1,
                        v2,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::Flay(target, flay_type) => {
                if let (me, Some(you), Some(venom_plan)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                    controller.aff_priorities.clone(),
                ) {
                    if !assure_wielded(&me, model, controller, "whip", true) {
                        unsafe {
                            if DEBUG_TREES {
                                println!("Failed to wield whip");
                            }
                        }
                        return UnpoweredFunctionState::Failed;
                    }
                    let venoms = get_venoms_from_plan(&venom_plan.to_vec(), 1, &you);
                    if venoms.len() < 1 {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(FlayAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        flay_type.get_annotation(),
                        venoms[0],
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::Garrote(target) => {
                if let (me, Some(you)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                ) {
                    if !assure_wielded(&me, model, controller, "whip", true) {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Rebounding) || you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(GarroteAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::Seal(target) => {
                if let (me, Some(you)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                ) {
                    if you.hypno_state.is_sealed() || !you.hypno_state.is_hypnotized() {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(SealAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        3,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::ShrugVenom => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::ClassCure1) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(ShruggingAction::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            InfiltratorBehavior::Sleight(target, sleight) => {
                if let (me, Some(you)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                ) {
                    if !me.balanced(BType::Secondary) {
                        return UnpoweredFunctionState::Failed;
                    } else if *sleight == SleightType::Void
                        && (you.is(FType::Weakvoid) || you.is(FType::Void))
                    {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Asthma) && *sleight == SleightType::Invasion {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Slickness) && *sleight == SleightType::Abrasion {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Anorexia) && *sleight == SleightType::Dissipate {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(SleightAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        sleight.get_annotation(),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::Slit(target) => {
                if let (me, Some(you), Some(venom_plan)) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                    controller.aff_priorities.clone(),
                ) {
                    if !assure_wielded(&me, model, controller, "dirk", false) {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Rebounding)
                        || you.is(FType::Shielded)
                        || !you.is_prone()
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venoms = get_venoms_from_plan(&venom_plan.to_vec(), 1, &you);
                    if venoms.len() < 1 {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(SlitAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        venoms[0],
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            InfiltratorBehavior::StackSuggest(target) => {
                if let (me, Some(you), hypno_stack) = (
                    model.state.borrow_me(),
                    target.get_target(model, controller),
                    controller.hypno_stack().clone(),
                ) {
                    if !me.arm_free() {
                        return UnpoweredFunctionState::Failed;
                    }
                    let suggestion = get_top_suggestion(you, &hypno_stack);
                    if suggestion.is_none() {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !you.hypno_state.is_hypnotized() {
                        controller.plan.add_to_qeb(Box::new(HypnotiseAction::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                        )));
                    }
                    controller.plan.add_to_qeb(Box::new(SuggestAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        suggestion.unwrap(),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {}
}

fn assure_wielded(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    wielded: &str,
    prefer_left: bool,
) -> bool {
    if !me.wield_state.is_wielding(wielded) {
        if me.can_wield(prefer_left, !prefer_left) {
            controller
                .plan
                .add_to_qeb(Box::new(WieldAction::quick_wield(
                    model.who_am_i(),
                    wielded.to_string(),
                    prefer_left,
                )));
        } else if me.can_wield(!prefer_left, prefer_left) {
            controller
                .plan
                .add_to_qeb(Box::new(WieldAction::quick_wield(
                    model.who_am_i(),
                    wielded.to_string(),
                    !prefer_left,
                )));
        } else {
            return false;
        }
    } else if me.wield_state.is_wielding_left(wielded) && !me.arm_free_left() {
        return false;
    } else if me.wield_state.is_wielding_right(wielded) && !me.arm_free_right() {
        return false;
    }
    true
}
