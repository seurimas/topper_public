use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::*,
    classes::get_venoms_from_plan,
    classes::group::*,
    items::{UnwieldAction, WieldAction},
    non_agent::AetTimelineRoomExt,
    observables::PlainAction,
    types::*,
};

use super::actions::*;

pub const INSTRUMENT_HINT: &str = "INSTRUMENT";
pub const WEAPON_HINT: &str = "WEAPON";
pub const IMPETUS_WEAPON_HINT: &str = "IMPETUS_WEAPON";
pub const FAST_WEAPON_HINT: &str = "FAST_WEAPON";

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
    PatchAggroedAlly,
    AudienceTarget,
    AudienceAggroedAlly,
    SingSong(Song),
    PlaySong(Song),
    SingOrPlaySong(Song),
    Induce(Emotion),
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
                } else if !me.arms_free() || me.stuck_fallen() {
                    return UnpoweredFunctionState::Failed;
                } else if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                } else if *weavable == Weavable::Boundary {
                    if let Some(room) = model.state.get_my_room() {
                        if room.has_tag("boundary") {
                            return UnpoweredFunctionState::Failed;
                        }
                    } else {
                        println!("No valid room!");
                        return UnpoweredFunctionState::Failed;
                    }
                } else if *weavable == Weavable::Impetus {
                    if !me.can_wield(true, true) {
                        return UnpoweredFunctionState::Failed;
                    } else if me
                        .check_if_bard(&|bard: &BardClassState| bard.instrument_song.is_some())
                        .unwrap_or(false)
                    {
                        return UnpoweredFunctionState::Failed;
                    } else if !assure_weapon_wielded_only(&me, model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if *weavable != Weavable::Impetus
                    && !assure_unwielded(&me, model, controller, false)
                {
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
                } else if !me.arms_free() || me.stuck_fallen() {
                    return UnpoweredFunctionState::Failed;
                } else if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                }
                if let Some(target) = controller.target.clone() {
                    let you = model.state.borrow_agent(&target);
                    if *weave_attack == WeavingAttack::Heartcage {
                        if !you.is(FType::Fallen) {
                            return UnpoweredFunctionState::Failed;
                        } else if !you.bard_board.iron_collar_state.is_active() {
                            return UnpoweredFunctionState::Failed;
                        } else if you.bard_board.emotion_state.primary.is_none() {
                            return UnpoweredFunctionState::Failed;
                        } else if you
                            .bard_board
                            .emotion_state
                            .get_emotion_level(you.bard_board.emotion_state.primary.unwrap())
                            < 50
                        {
                            return UnpoweredFunctionState::Failed;
                        } else if let Some(room) = model.state.get_my_room() {
                            if !room.has_tag("boundary") {
                                return UnpoweredFunctionState::Failed;
                            }
                        }
                    }
                    if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !assure_unwielded(&me, model, controller, false) {
                        return UnpoweredFunctionState::Failed;
                    }
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
                }
                if *venom_attack == BardVenomAttack::Bravado {
                    let me = model.state.borrow_me();
                    if !me.balanced(BType::ClassCure1) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if let (Some(target), Some(venom_plan)) =
                    (controller.target.clone(), controller.aff_priorities.clone())
                {
                    let me = model.state.borrow_me();
                    let you = model.state.borrow_agent(&target);
                    if *venom_attack != BardVenomAttack::Needle && you.is(FType::Rebounding) {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    } else if me.stuck_fallen() {
                        return UnpoweredFunctionState::Failed;
                    } else if *venom_attack != BardVenomAttack::Needle
                        && me.check_if_bard(&|me| me.is_on_tempo()).unwrap_or_default()
                    {
                        return UnpoweredFunctionState::Failed;
                    } else if *venom_attack != BardVenomAttack::Needle {
                        let me = model.state.borrow_me();
                        if !assure_weapon_wielded(&me, model, controller, true) {
                            return UnpoweredFunctionState::Failed;
                        }
                    }
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
                }
                if let Some(target) = &controller.target.clone() {
                    let me = model.state.borrow_me();
                    let you = model.state.borrow_agent(target);
                    if performance_attack.gets_rebounded()
                        && (you.is(FType::Rebounding) || you.is(FType::Shielded))
                    {
                        return UnpoweredFunctionState::Failed;
                    } else if performance_attack.must_stand() && me.stuck_fallen() {
                        return UnpoweredFunctionState::Failed;
                    } else if performance_attack.needs_arm() && !me.arm_free() {
                        return UnpoweredFunctionState::Failed;
                    } else if performance_attack.needs_weapon()
                        && me.check_if_bard(&|me| me.is_on_tempo()).unwrap_or_default()
                    {
                        return UnpoweredFunctionState::Failed;
                    } else if performance_attack.needs_voice() && me.is(FType::CrippledThroat) {
                        return UnpoweredFunctionState::Failed;
                    } else if performance_attack.needs_weapon() {
                        if me
                            .check_if_bard(&|me: &BardClassState| me.is_on_tempo())
                            .unwrap_or(false)
                        {
                            // If it's a weapon attack, we cannot use while in rhythm.
                            return UnpoweredFunctionState::Failed;
                        }
                        if !assure_weapon_wielded(&me, model, controller, true) {
                            return UnpoweredFunctionState::Failed;
                        }
                    } else if performance_attack.needs_free_hand() {
                        if !assure_unwielded(&me, model, controller, true) {
                            return UnpoweredFunctionState::Failed;
                        }
                    }
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
                    } else if me.stuck_fallen() {
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
            BardBehavior::PatchAggroedAlly => {
                let me = model.state.borrow_me();
                if me
                    .check_if_bard(&|bard| bard.dithering > 0)
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if !me.arms_free() || me.stuck_fallen() {
                    return UnpoweredFunctionState::Failed;
                } else if !controller.has_qeb() {
                    return UnpoweredFunctionState::Failed;
                }
                let best_ally = get_aggroed_ally(controller);
                if let Some((ally, _aggro)) = &best_ally {
                    if !assure_unwielded(&me, model, controller, false) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(WeavingAttackAction::patchwork(
                            model.who_am_i(),
                            ally.clone(),
                        )));
                } else {
                    // println!("NO ALLIES");
                    return UnpoweredFunctionState::Failed;
                }
            }
            BardBehavior::AudienceTarget => {
                if let Some(target) = &controller.target {
                    controller
                        .plan
                        .add_to_front_of_qeb(Box::new(PlainAction::new(format!(
                            "audience {}",
                            target
                        ))));
                }
            }
            BardBehavior::AudienceAggroedAlly => {
                let best_ally = get_aggroed_ally(controller);
                if let Some((ally, _aggro)) = &best_ally {
                    controller
                        .plan
                        .add_to_front_of_qeb(Box::new(PlainAction::new(format!(
                            "audience {}",
                            ally
                        ))));
                } else {
                    // println!("NO ALLIES");
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
                let me = model.state.borrow_me();
                if me
                    .check_if_bard(&|bard| bard.voice_song.is_some())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if me.stuck_fallen() || !me.arm_free() {
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
                } else if me.stuck_fallen() || !me.arms_free() {
                    return UnpoweredFunctionState::Failed;
                } else if !assure_instrument_wielded(&me, model, controller, false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(SongAction::play(model.who_am_i(), *play_song)));
                controller.used_balance = true;
                controller.used_equilibrium = true;
            }
            BardBehavior::SingOrPlaySong(play_song) => {
                let me = model.state.borrow_me();
                let mut playing = true;
                if me.stuck_fallen() {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_bard(&|bard| {
                        bard.instrument_song == Some(*play_song)
                            || bard.voice_song == Some(*play_song)
                    })
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                if me
                    .check_if_bard(&|bard| bard.instrument_song.is_some())
                    .unwrap_or(false)
                {
                    playing = false;
                } else if !me.arms_free() {
                    playing = false;
                } else if !assure_instrument_wielded(&me, model, controller, false) {
                    playing = false;
                }
                if !playing
                    && model
                        .state
                        .borrow_me()
                        .check_if_bard(&|bard| bard.voice_song.is_some())
                        .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if !me.arm_free() {
                    return UnpoweredFunctionState::Failed;
                }
                if playing {
                    controller
                        .plan
                        .add_to_qeb(Box::new(SongAction::play(model.who_am_i(), *play_song)));
                } else {
                    controller
                        .plan
                        .add_to_qeb(Box::new(SongAction::sing(model.who_am_i(), *play_song)));
                }
                controller.used_balance = true;
                controller.used_equilibrium = true;
            }
            BardBehavior::Induce(emotion) => {
                let me = model.state.borrow_me();
                if !me
                    .check_if_bard(&|bard| {
                        bard.instrument_song.is_some() || bard.voice_song.is_some()
                    })
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                if let Some(target) = &controller.target {
                    controller.plan.add_to_qeb(Box::new(InduceAction::new(
                        model.who_am_i(),
                        target.clone(),
                        *emotion,
                    )));
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
        }
        UnpoweredFunctionState::Complete
    }

    fn reset(self: &mut Self, parameter: &Self::Model) {
        // Nothing to do.
    }
}

fn get_aggroed_ally(controller: &mut BehaviorController) -> Option<(String, i32)> {
    let mut best_ally: Option<(String, i32)> = None;
    for (ally, aggro) in controller.allies.iter() {
        if *aggro
            > best_ally
                .as_ref()
                .map(|(_, ally_aggro)| *ally_aggro)
                .unwrap_or(-1)
        {
            best_ally = Some((ally.clone(), *aggro));
        }
    }
    best_ally.filter(|(_ally, aggro)| *aggro > 0)
}

fn assure_unwielded(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    prefer_left: bool,
) -> bool {
    if !me.wield_state.empty_hand() {
        if me
            .check_if_bard(&|bard: &BardClassState| bard.instrument_song.is_some())
            .unwrap_or(false)
        {
            if let (Some(left), Some(right)) =
                (me.wield_state.get_left(), me.wield_state.get_right())
            {
                let instrument = get_instrument(controller);
                if prefer_left && left.contains(&instrument) && !right.contains(&instrument) {
                    return assure_unwielded(me, model, controller, false);
                } else if !prefer_left && right.contains(&instrument) && !left.contains(&instrument)
                {
                    return assure_unwielded(me, model, controller, true);
                }
            }
        } else if me
            .check_if_bard(&|bard: &BardClassState| bard.is_on_tempo())
            .unwrap_or(false)
        {
            if let (Some(left), Some(right)) =
                (me.wield_state.get_left(), me.wield_state.get_right())
            {
                if prefer_left && left.contains("falchion") {
                    return assure_unwielded(me, model, controller, false);
                } else if !prefer_left && right.contains("falchion") {
                    return assure_unwielded(me, model, controller, true);
                }
            }
        }
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

fn assure_weapon_wielded(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    prefer_left: bool,
) -> bool {
    if let Some(alias) = controller.get_hint(WEAPON_HINT).cloned() {
        assure_wielded(&me, model, controller, alias.as_str(), prefer_left)
    } else {
        assure_wielded(&me, model, controller, "falchion", prefer_left)
    }
}

fn get_instrument(controller: &mut BehaviorController) -> String {
    if let Some(alias) = controller.get_hint(INSTRUMENT_HINT).cloned() {
        alias
    } else {
        "lute".to_string()
    }
}

fn assure_instrument_wielded(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    prefer_left: bool,
) -> bool {
    if let Some(alias) = controller.get_hint(INSTRUMENT_HINT).cloned() {
        assure_wielded(&me, model, controller, alias.as_str(), prefer_left)
    } else {
        assure_wielded(&me, model, controller, "lute", prefer_left)
    }
}

fn assure_wielded(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    wielded: &str,
    prefer_left: bool,
) -> bool {
    if me
        .check_if_bard(&|bard: &BardClassState| bard.instrument_song.is_some())
        .unwrap_or(false)
    {
        if let (Some(left), Some(right)) = (me.wield_state.get_left(), me.wield_state.get_right()) {
            let instrument = get_instrument(controller);
            if prefer_left && left.contains(&instrument) && !right.contains(&instrument) {
                return assure_wielded(me, model, controller, wielded, false);
            } else if !prefer_left && right.contains(&instrument) && !left.contains(&instrument) {
                return assure_wielded(me, model, controller, wielded, true);
            }
        }
    }
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

fn assure_weapon_wielded_only(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
) -> bool {
    let wielded = if let Some(alias) = controller.get_hint(WEAPON_HINT) {
        alias.as_str()
    } else {
        "falchion"
    };
    if !me.can_wield(true, true) {
        return false;
    } else if !me.wield_state.is_wielding(wielded) {
        controller
            .plan
            .add_to_qeb(Box::new(WieldAction::quick_wield(
                model.who_am_i(),
                wielded.to_string(),
                true,
            )));
        controller
            .plan
            .add_to_qeb(Box::new(UnwieldAction::unwield(model.who_am_i(), false)));
    } else if me.wield_state.is_wielding_left(wielded) {
        controller
            .plan
            .add_to_qeb(Box::new(UnwieldAction::unwield(model.who_am_i(), false)));
    } else if me.wield_state.is_wielding_right(wielded) {
        controller
            .plan
            .add_to_qeb(Box::new(UnwieldAction::unwield(model.who_am_i(), true)));
    }
    true
}
