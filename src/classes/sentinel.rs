use crate::alpha_beta::ActionPlanner;
#[macro_use(affliction_stacker, affliction_plan_stacker)]
use crate::{affliction_stacker, affliction_plan_stacker};
use crate::classes::*;
use crate::observables::*;
use crate::timeline::*;
use crate::topper::*;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod sentinel_tests {
    use super::*;

    #[test]
    fn test_salve_attacks() {
        let mut timeline = Timeline::new();
        let breath_flourish_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    annotation: "".to_string(),
                    caster: "Rinata".to_string(),
                    category: "Woodlore".to_string(),
                    skill: "Icebreath".to_string(),
                    target: "Illidan".to_string(),
                }),
                Observation::CombatAction(CombatAction {
                    annotation: "".to_string(),
                    caster: "Rinata".to_string(),
                    category: "Dhuriv".to_string(),
                    skill: "Flourish".to_string(),
                    target: "Illidan".to_string(),
                }),
                Observation::Devenoms("epseth".to_string()),
            ],
            lines: vec![],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(breath_flourish_slice);
        let me_state = timeline.state.get_agent(&"Rinata".to_string());
        assert_eq!(me_state.balanced(BType::Balance), false);
        assert_eq!(me_state.balanced(BType::Equil), false);
        assert_eq!(me_state.is(FType::Insulation), true);
        assert_eq!(me_state.is(FType::LeftLegBroken), true);
        let you_state = timeline.state.get_agent(&"Illidan".to_string());
        assert_eq!(you_state.balanced(BType::Balance), true);
        assert_eq!(you_state.balanced(BType::Equil), true);
        assert_eq!(you_state.is(FType::Insulation), false);
        assert_eq!(you_state.is(FType::LeftLegBroken), false);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum FirstStrike {
    Slash(&'static str),
    Ambush(&'static str),
    Blind,
    Twirl,
    Strike,
    Crosscut,
    WeakenArms,
    WeakenLegs,
    Reave,
    Trip,
    Slam,
    Daunt(&'static str),
    Icebreath,
}

impl FirstStrike {
    pub fn combo_str(&self) -> String {
        match self {
            FirstStrike::Slash(venom) => "slash".to_string(),
            FirstStrike::Ambush(venom) => "ambush".to_string(),
            FirstStrike::Blind => "blind".to_string(),
            FirstStrike::Twirl => "twirl".to_string(),
            FirstStrike::Strike => "strike".to_string(),
            FirstStrike::Crosscut => "crosscut".to_string(),
            FirstStrike::WeakenArms => "weaken arms".to_string(),
            FirstStrike::WeakenLegs => "weaken legs".to_string(),
            FirstStrike::Reave => "reave".to_string(),
            FirstStrike::Trip => "trip".to_string(),
            FirstStrike::Slam => "slam".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn full_str(&self, target: &String) -> String {
        match self {
            FirstStrike::Daunt(animal) => format!("order {} daunt {}", animal, target),
            FirstStrike::Icebreath => format!("order icewyrm icebreath {}", target),
            _ => "".to_string(), // TODO
        }
    }

    pub fn venom(&self) -> &'static str {
        match self {
            FirstStrike::Slash(venom) | FirstStrike::Ambush(venom) => venom,
            _ => "",
        }
    }

    pub fn flourish(&self) -> bool {
        match self {
            FirstStrike::Daunt(_) | FirstStrike::Icebreath => true,
            _ => false,
        }
    }

    pub fn ignores_rebounding(&self) -> bool {
        match self {
            FirstStrike::Twirl => false, // TODO: We need to handle for second strike rebounding if we try this.
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum SecondStrike {
    Stab(&'static str),
    Slice(&'static str),
    Thrust(&'static str),
    Flourish(&'static str),
    Disarm,
    Gouge,
    Heartbreaker,
    Slit,
}

impl SecondStrike {
    pub fn combo_str(&self) -> String {
        match self {
            SecondStrike::Stab(venom) => "stab".to_string(),
            SecondStrike::Slice(venom) => "slice".to_string(),
            SecondStrike::Thrust(venom) => "thrust".to_string(),
            SecondStrike::Disarm => "disarm".to_string(),
            SecondStrike::Gouge => "gouge".to_string(),
            SecondStrike::Heartbreaker => "heartbreaker".to_string(),
            SecondStrike::Slit => "slit".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn full_str(&self, target: &String) -> String {
        match self {
            SecondStrike::Flourish(venom) => format!("dhuriv flourish {} {}", target, venom),
            _ => "".to_string(), // TODO
        }
    }

    pub fn venom(&self) -> &'static str {
        match self {
            SecondStrike::Stab(venom)
            | SecondStrike::Slice(venom)
            | SecondStrike::Thrust(venom) => venom,
            _ => "",
        }
    }
}

lazy_static! {
    static ref FIRST_STRIKES: HashMap<FType, FirstStrike> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(*aff, FirstStrike::Slash(venom));
        }
        val.insert(FType::Frozen, FirstStrike::Icebreath);
        val.insert(FType::Shivering, FirstStrike::Icebreath);
        val.insert(FType::Confusion, FirstStrike::Twirl);
        val.insert(FType::Impairment, FirstStrike::Crosscut);
        val.insert(FType::Addiction, FirstStrike::Crosscut);
        val.insert(FType::Lethargy, FirstStrike::WeakenLegs);
        val.insert(FType::Epilepsy, FirstStrike::Slam);
        val.insert(FType::Laxity, FirstStrike::Slam);
        val
    };
}

lazy_static! {
    static ref FIRST_STRIKE_AFFS: HashMap<FirstStrike, Vec<FType>> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(FirstStrike::Slash(venom), vec![*aff]);
        }
        val.insert(FirstStrike::Slash("epseth"), vec![FType::LeftLegBroken, FType::RightLegBroken]);
        val.insert(FirstStrike::Slash("epteth"), vec![FType::LeftArmBroken, FType::RightArmBroken]);
        val.insert(FirstStrike::Twirl, vec![FType::Confusion]);
        // Wrong, only one actually applies
        val.insert(FirstStrike::Crosscut, vec![FType::Impairment, FType::Addiction]);
        val.insert(FirstStrike::WeakenLegs, vec![FType::Lethargy]);
        val.insert(FirstStrike::Slam, vec![FType::Epilepsy, FType::Laxity]);
        val
    };
}

affliction_plan_stacker!(
    add_first_strike_from_plan,
    get_first_strike_from_plan,
    FIRST_STRIKES,
    FirstStrike
);

fn assume_hit(who: &mut AgentState, strike: &FirstStrike) {
    if let Some(affs) = FIRST_STRIKE_AFFS.get(strike) {
        for aff in affs {
            who.set_flag(*aff, true);
        }
    }
}

lazy_static! {
    static ref SECOND_STRIKES: HashMap<FType, SecondStrike> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(*aff, SecondStrike::Stab(venom));
        }
        val.insert(FType::Impatience, SecondStrike::Gouge);
        val.insert(FType::Heartflutter, SecondStrike::Heartbreaker);
        val.insert(FType::CrippledThroat, SecondStrike::Slit);
        val
    };
}

affliction_plan_stacker!(
    add_second_strike_from_plan,
    get_second_strike_from_plan,
    SECOND_STRIKES,
    SecondStrike
);

/**
 *
 * ActiveTransitions!
 *
**/

pub struct ComboAction {
    pub caster: String,
    pub target: String,
    pub first_strike: FirstStrike,
    pub second_strike: SecondStrike,
}

impl ComboAction {
    pub fn new(
        caster: String,
        target: String,
        first_strike: FirstStrike,
        second_strike: SecondStrike,
    ) -> Self {
        ComboAction {
            caster,
            target,
            first_strike,
            second_strike,
        }
    }
}

impl ActiveTransition for ComboAction {
    fn simulate(&self, _timeline: &Timeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
        Ok(get_combo_action(
            &timeline,
            &self.target,
            &self.first_strike,
            &self.second_strike,
        ))
    }
}

fn get_combo_action(
    timeline: &Timeline,
    target: &String,
    first_strike: &FirstStrike,
    second_strike: &SecondStrike,
) -> String {
    let attack = if first_strike.flourish() {
        format!(
            "{};;{}",
            first_strike.full_str(target),
            second_strike.full_str(target)
        )
    } else {
        format!(
            "dhuriv combo {} {} {} {} {}",
            target,
            first_strike.combo_str(),
            second_strike.combo_str(),
            first_strike.venom(),
            second_strike.venom(),
        )
    };
    format!("order loyal attack {};;stand;;stand;;{}", target, attack)
}

pub struct PierceAction {
    pub caster: String,
    pub target: String,
    pub side: String,
}

impl PierceAction {
    pub fn new(caster: String, target: String, side: String) -> Self {
        PierceAction {
            caster,
            target,
            side,
        }
    }
}

impl ActiveTransition for PierceAction {
    fn simulate(&self, _timeline: &Timeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, _timeline: &Timeline) -> ActivateResult {
        Ok(format!(
            "stand;;stand;;dhuriv pierce {} {}",
            self.target, self.side
        ))
    }
}

pub struct SeverAction {
    pub caster: String,
    pub target: String,
    pub side: String,
}

impl SeverAction {
    pub fn new(caster: String, target: String, side: String) -> Self {
        SeverAction {
            caster,
            target,
            side,
        }
    }
}

impl ActiveTransition for SeverAction {
    fn simulate(&self, _timeline: &Timeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, _timeline: &Timeline) -> ActivateResult {
        Ok(format!(
            "stand;;stand;;dhuriv sever {} {}",
            self.target, self.side
        ))
    }
}

pub struct MightAction {
    pub caster: String,
}

impl MightAction {
    pub fn new(caster: String) -> Self {
        MightAction { caster }
    }
}

impl ActiveTransition for MightAction {
    fn simulate(&self, _timeline: &Timeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, _timeline: &Timeline) -> ActivateResult {
        Ok("might".to_string())
    }
}

pub struct DualrazeAction {
    pub caster: String,
    pub target: String,
}

impl DualrazeAction {
    pub fn new(caster: String, target: String) -> Self {
        DualrazeAction { caster, target }
    }
}

impl ActiveTransition for DualrazeAction {
    fn simulate(&self, _timeline: &Timeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, _timeline: &Timeline) -> ActivateResult {
        Ok(format!("dhuriv dualraze {}", self.target))
    }
}

pub struct FitnessAction {
    pub caster: String,
}

impl FitnessAction {
    pub fn new(caster: String) -> Self {
        FitnessAction { caster }
    }
}

impl ActiveTransition for FitnessAction {
    fn simulate(&self, _timeline: &Timeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, _timeline: &Timeline) -> ActivateResult {
        Ok("fitness".to_string())
    }
}

/**
 * Observations
 **/

lazy_static! {
    static ref DUALRAZE_ORDER: Vec<FType> = vec![FType::Shielded, FType::Rebounding, FType::Speed,];
}

lazy_static! {
    static ref REAVE_ORDER: Vec<FType> = vec![FType::Shielded, FType::Rebounding];
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    _before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Might" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::ClassCure1, 20.0);
            });
        }
        "Slash" | "Stab" | "Slice" | "Thrust" | "Ambush" | "Flourish" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_weapon_hits(
                &mut me,
                &mut you,
                after,
                combat_action.caster.eq(&agent_states.me),
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string()),
            )?;
            apply_or_infer_balance(&mut me, (BType::Balance, 2.65), after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Pierce" | "Sever" => {
            let mut target = &combat_action.target;
            let mut limb_hit = None;
            let mut limb_damaged = false;
            for observation in after {
                match observation {
                    Observation::Damaged(_who, limb) => {
                        limb_hit = get_limb_damage(limb).ok();
                        limb_damaged = true;
                    }
                    Observation::Connects(limb) => {
                        limb_hit = get_limb_damage(limb).ok();
                        limb_damaged = false;
                    }
                    Observation::Rebounds => {
                        target = &combat_action.caster;
                    }
                    Observation::CombatAction(action) => {
                        if action != combat_action {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(limb_hit) = limb_hit {
                for_agent_closure(
                    agent_states,
                    target,
                    Box::new(move |you: &mut AgentState| {
                        if limb_damaged {
                            you.set_limb_damage(limb_hit, DAMAGED_VALUE);
                            you.limb_damage.set_limb_damaged(limb_hit, true);
                        } else {
                            you.set_flag(limb_hit.broken().unwrap(), true);
                        }
                    }),
                );
            } else {
                println!("No limb hit...");
            }
        }
        "Dualraze" => {
            let razed = combat_action.annotation.clone();
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |mut you: &mut AgentState| {
                    remove_through(
                        you,
                        match razed.as_ref() {
                            "rebounding" => FType::Rebounding,
                            "shield" => FType::Shielded,
                            _ => FType::Speed,
                        },
                        &DUALRAZE_ORDER.to_vec(),
                    );
                }),
            );
        }
        "Reave" => {
            let razed = combat_action.annotation.clone();
            for_agent_closure(
                agent_states,
                &combat_action.target,
                Box::new(move |mut you: &mut AgentState| {
                    remove_through(
                        you,
                        match razed.as_ref() {
                            "shielded" => FType::Shielded,
                            _ => FType::Rebounding,
                        },
                        &REAVE_ORDER.to_vec(),
                    );
                }),
            );
            if let Some(def_flag) = FType::from_name(&combat_action.annotation) {
                attack_strip(agent_states, &combat_action.caster, vec![def_flag], after);
            }
        }
        "Twirl" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Confusion],
                after,
            );
        }
        "Throatcrush" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::DestroyedThroat],
                after,
            );
        }
        "Crosscut" => {
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::Impairment)
            {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Impairment],
                    after,
                );
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Addiction],
                    after,
                );
            }
        }
        "Weaken" => {
            // TODO: Parse out which limb was hit and its effect
        }
        "Trip" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Balance, 2.25);
            });
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
        }
        "Slam" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Balance, 2.25);
            });
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Epilepsy, FType::Laxity],
                after,
            );
        }
        "Gouge" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Balance, 2.25);
            });
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Impatience],
                after,
            );
        }
        "Heartbreaker" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Balance, 2.25);
            });
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Heartflutter],
                after,
            );
        }
        "Slit" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Balance, 2.25);
            });
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::CrippledThroat],
                after,
            );
        }
        // Passive actions
        "Gyrfalcon" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![FType::Disfigurement],
                after,
            );
        }
        "Elk" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![FType::Fallen],
                after,
            );
        }
        "Weasel" => {
            if let Some(def_flag) = FType::from_name(&combat_action.annotation) {
                attack_strip(agent_states, &combat_action.caster, vec![def_flag], after);
            }
        }
        "Cockatrice" | "Crocodile" | "Raloth" => {
            if let Some(aff_flag) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.caster, vec![aff_flag], after);
            }
        }
        "Daunt" => match combat_action.annotation.as_ref() {
            "direwolf" => {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.set_balance(BType::Equil, 2.25);
                });
                attack_afflictions(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Claustrophobia],
                    after,
                );
            }
            "raloth" => {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.set_balance(BType::Equil, 2.25);
                });
                attack_afflictions(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Agoraphobia],
                    after,
                );
            }
            "crocodile" => {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.set_balance(BType::Equil, 2.25);
                });
                attack_afflictions(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Loneliness],
                    after,
                );
            }
            "cockatrice" => {
                for_agent(agent_states, &combat_action.caster, |me| {
                    me.set_balance(BType::Equil, 2.25);
                });
                attack_afflictions(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Berserking],
                    after,
                );
            }
            _ => {}
        },
        "Icebreath" => {
            for_agent(agent_states, &combat_action.caster, |me| {
                me.set_balance(BType::Equil, 2.25);
            });
            attack_strip_or_afflict(
                agent_states,
                &combat_action.target,
                vec![FType::Insulation, FType::Shivering, FType::Frozen],
                after,
            );
        }
        "Icewyrm" => {
            attack_strip_or_afflict(
                agent_states,
                &combat_action.caster,
                vec![FType::Insulation, FType::Shivering, FType::Frozen],
                after,
            );
        }
        _ => {}
    }
    Ok(())
}

/**
 * Planning
 **/

lazy_static! {
    static ref DEFAULT_STACK: Vec<FType> = vec![
        FType::Impatience,
        FType::Epilepsy,
        FType::Paresis,
        FType::Asthma,
        FType::Clumsiness,
        FType::Impairment,
        FType::Stupidity,
        FType::Vomiting,
        FType::Slickness,
        FType::Anorexia,
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Dizziness,
        FType::Confusion,
        FType::Impatience,
        FType::Epilepsy,
        FType::Sensitivity,
        FType::Recklessness,
    ];
}

fn get_stack<'s>(
    timeline: &Timeline,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> Vec<VenomPlan> {
    let mut vec = db
        .and_then(|db| db.get_venom_plan(&format!("sentinel_{}", strategy)))
        .unwrap_or(get_simple_plan(DEFAULT_STACK.to_vec()));
    let you = timeline.state.borrow_agent(target);
    vec.retain(move |aff| match aff.affliction() {
        FType::Impatience
        | FType::Epilepsy
        | FType::Laxity
        | FType::Heartflutter
        | FType::Impairment => !you.can_parry(),
        _ => true,
    });
    vec
}

fn want_fitness(me: &AgentState) -> bool {
    me.balanced(BType::ClassCure2)
        && me.is(FType::Asthma)
        && (me.is(FType::Hellsight) || me.is(FType::Slickness))
}

fn want_might(me: &AgentState) -> bool {
    me.balanced(BType::ClassCure1)
        && me.affs_count(&vec![FType::Anorexia, FType::Asthma, FType::Slickness]) >= 2
}

fn want_pierce(you: &AgentState) -> Option<String> {
    if you.can_parry() || you.is(FType::Rebounding) || you.is(FType::Shielded) {
        return None;
    } else if you.limb_damage.broken(LType::LeftLegDamage)
        && !you.limb_damage.damaged(LType::LeftLegDamage)
        && you.limb_damage.restoring != Some(LType::LeftLegDamage)
    {
        return Some("left".to_string());
    } else if you.limb_damage.broken(LType::RightLegDamage)
        && !you.limb_damage.damaged(LType::RightLegDamage)
        && you.limb_damage.restoring != Some(LType::RightLegDamage)
    {
        return Some("right".to_string());
    } else {
        return None;
    }
}

fn want_sever(you: &AgentState) -> Option<String> {
    if you.can_parry() || you.is(FType::Rebounding) || you.is(FType::Shielded) {
        return None;
    } else if you.limb_damage.broken(LType::LeftArmDamage)
        && !you.limb_damage.damaged(LType::LeftArmDamage)
        && you.limb_damage.restoring != Some(LType::LeftArmDamage)
    {
        return Some("left".to_string());
    } else if you.limb_damage.broken(LType::RightArmDamage)
        && !you.limb_damage.damaged(LType::RightArmDamage)
        && you.limb_damage.restoring != Some(LType::RightArmDamage)
    {
        return Some("right".to_string());
    } else {
        return None;
    }
}

pub fn get_balance_attack<'s>(
    timeline: &Timeline,
    who_am_i: &String,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> Box<dyn ActiveTransition> {
    if strategy.eq("damage") {
        return Box::new(Inactivity);
    } else {
        let stack = get_stack(timeline, target, strategy, db);
        let me = timeline.state.borrow_agent(who_am_i);
        let mut you = timeline.state.borrow_agent(target);
        if want_fitness(&me) {
            return Box::new(FitnessAction::new(who_am_i.to_string()));
        } else if want_might(&me) {
            return Box::new(MightAction::new(who_am_i.to_string()));
        } else if you.is(FType::Shielded) && you.is(FType::Rebounding) {
            return Box::new(DualrazeAction::new(who_am_i.to_string(), target.clone()));
        } else if let Some(side) = want_pierce(&you) {
            return Box::new(PierceAction::new(
                who_am_i.to_string(),
                target.clone(),
                side.clone(),
            ));
        } else if let Some(side) = want_sever(&you) {
            return Box::new(SeverAction::new(
                who_am_i.to_string(),
                target.clone(),
                side.clone(),
            ));
        } else {
            let first_strike = get_first_strike_from_plan(&stack, 1, &you).pop();
            if let Some(mut first_strike) = first_strike {
                if you.is(FType::Rebounding) && !first_strike.ignores_rebounding() {
                    first_strike = FirstStrike::Reave;
                }
                assume_hit(&mut you, &first_strike);
                let second_strike = if first_strike.flourish() {
                    get_venoms_from_plan(&stack, 1, &you)
                        .pop()
                        .map(|venom| SecondStrike::Flourish(venom))
                } else {
                    get_second_strike_from_plan(&stack, 1, &you).pop()
                };
                if let Some(second_strike) = second_strike {
                    return Box::new(ComboAction::new(
                        who_am_i.to_string(),
                        target.clone(),
                        first_strike,
                        second_strike,
                    ));
                }
            }
        }
        return Box::new(Inactivity);
    }
}

pub fn get_action_plan(
    timeline: &Timeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> ActionPlan {
    let mut action_plan = ActionPlan::new(me);
    let mut balance = get_balance_attack(timeline, me, target, strategy, db);
    if let Some(parry) = get_needed_parry(timeline, me, target, strategy, db) {
        balance = Box::new(SeparatorAction::pair(
            Box::new(ParryAction::new(me.to_string(), parry)),
            balance,
        ));
    }
    if let Ok(_activation) = balance.act(&timeline) {
        action_plan.add_to_qeb(balance);
    }
    action_plan
}

pub fn get_attack(
    timeline: &Timeline,
    target: &String,
    strategy: &String,
    db: Option<&DatabaseModule>,
) -> String {
    let action_plan = get_action_plan(&timeline, &timeline.who_am_i(), &target, &strategy, db);
    action_plan.get_inputs(&timeline)
}
