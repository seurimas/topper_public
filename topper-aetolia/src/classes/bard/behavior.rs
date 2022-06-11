use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::*,
    classes::get_venoms_from_plan,
    items::{UnwieldAction, WieldAction},
    types::*,
};

use super::{actions::*, ANELACE};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BardVenomAttack {
    Tempo,
    Needle,
    Harry,
    Bravado,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BardBehavior {
    Weave(Weavable),
    WeaveAttack(WeavingAttack),
    PerformanceAttack(PerformanceAttack),
    VenomAttack(BardVenomAttack),
    Anelace,
    ColdRead,
    SingSong(Song),
    PlaySong(Song),
}

impl UnpoweredFunction for BardBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            BardBehavior::Weave(weavable) => {
                let me = model.state.borrow_me();
                if me
                    .check_if_bard(&|bard| bard.dithering > 0)
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                } else if !assure_unwielded(&me, model, controller, false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(WeavingAction::new(model.who_am_i(), *weavable)));
                controller.used_equilibrium = true;
            }
            BardBehavior::WeaveAttack(weave_attack) => {
                let me = model.state.borrow_me();
                if me
                    .check_if_bard(&|bard| bard.dithering > 0)
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                } else if !assure_unwielded(&me, model, controller, false) {
                    return UnpoweredFunctionState::Failed;
                }
                if let Some(target) = &controller.target {
                    controller
                        .plan
                        .add_to_qeb(Box::new(WeavingAttackAction::new(
                            model.who_am_i(),
                            target.to_string(),
                            *weave_attack,
                        )));
                    controller.used_equilibrium = true;
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            BardBehavior::VenomAttack(venom_attack) => {
                if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                } else if *venom_attack != BardVenomAttack::Needle {
                    let me = model.state.borrow_me();
                    if !assure_wielded(&me, model, controller, "falchion", true) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if *venom_attack == BardVenomAttack::Bravado {
                    let me = model.state.borrow_me();
                    if !me.balanced(BType::ClassCure1) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if let (Some(target), Some(venom_plan)) = (
                    controller.target.as_ref(),
                    controller.aff_priorities.as_ref(),
                ) {
                    let you = model.state.borrow_agent(target);
                    let venom_count = match venom_attack {
                        BardVenomAttack::Tempo => 3,
                        _ => 1,
                    };
                    let venoms = get_venoms_from_plan(&venom_plan.to_vec(), venom_count, &you);
                    controller
                        .plan
                        .add_to_qeb(Box::new(PerformanceAttackAction::new(
                            model.who_am_i(),
                            target.to_string(),
                            match venom_attack {
                                BardVenomAttack::Tempo => PerformanceAttack::TempoThree(
                                    venoms.get(2).unwrap_or(&"kalmia").to_string(),
                                    venoms.get(1).unwrap_or(&"digitalis").to_string(),
                                    venoms.get(0).unwrap_or(&"curare").to_string(),
                                ),
                                BardVenomAttack::Needle => PerformanceAttack::Needle(
                                    venoms.get(0).unwrap_or(&"curare").to_string(),
                                ),
                                BardVenomAttack::Harry => PerformanceAttack::Harry(
                                    venoms.get(0).unwrap_or(&"curare").to_string(),
                                ),
                                BardVenomAttack::Bravado => PerformanceAttack::Bravado(
                                    venoms.get(0).unwrap_or(&"curare").to_string(),
                                ),
                            },
                        )));
                }
            }
            BardBehavior::PerformanceAttack(performance_attack) => {
                if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                } else if performance_attack.needs_weapon() {
                    let me = model.state.borrow_me();
                    if !assure_wielded(&me, model, controller, "falchion", true) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if let Some(target) = &controller.target {
                    controller
                        .plan
                        .add_to_qeb(Box::new(PerformanceAttackAction::new(
                            model.who_am_i(),
                            target.to_string(),
                            performance_attack.clone(),
                        )));
                    controller.used_balance = true;
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            BardBehavior::Anelace => {
                if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                } else {
                    let me = model.state.borrow_me();
                    if me
                        .check_if_bard(&|bard| bard.anelaces == 0)
                        .unwrap_or_default()
                    {
                        // No anelaces ready
                        return UnpoweredFunctionState::Failed;
                    } else if !assure_wielded(&me, model, controller, "anelace", false) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if let Some(target) = &controller.target {
                    controller.plan.add_to_qeb(Box::new(AnelaceAction::new(
                        model.who_am_i(),
                        target.to_string(),
                    )));
                    controller.used_balance = true;
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            BardBehavior::ColdRead => {
                if let Some(target) = &controller.target {
                    controller.plan.add_to_qeb(Box::new(ColdReadAction::new(
                        model.who_am_i(),
                        target.to_string(),
                    )));
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            BardBehavior::SingSong(sing_song) => {
                if model
                    .state
                    .borrow_me()
                    .check_if_bard(&|bard| bard.voice_song.is_some())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(SongAction::sing(model.who_am_i(), *sing_song)));
                controller.used_equilibrium = true;
            }
            BardBehavior::PlaySong(play_song) => {
                let me = model.state.borrow_me();
                if me
                    .check_if_bard(&|bard| bard.instrument_song.is_some())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if !assure_wielded(&me, model, controller, "fife", false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(SongAction::play(model.who_am_i(), *play_song)));
                controller.used_balance = true;
                controller.used_equilibrium = true;
            }
        }
        UnpoweredFunctionState::Complete
    }

    fn reset(self: &mut Self, parameter: &Self::Model) {
        // Nothing to do.
    }
}

fn assure_unwielded(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    prefer_left: bool,
) -> bool {
    if !me.wield_state.empty_hand() {
        if me.can_wield(prefer_left, !prefer_left) {
            controller.plan.add_to_qeb(Box::new(UnwieldAction::unwield(
                model.who_am_i(),
                prefer_left,
            )));
        } else if me.can_wield(!prefer_left, prefer_left) {
            controller.plan.add_to_qeb(Box::new(UnwieldAction::unwield(
                model.who_am_i(),
                !prefer_left,
            )));
        } else {
            return false;
        }
    }
    true
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
    }
    true
}
