use crate::actions::*;
use crate::alpha_beta::*;
use crate::classes::{get_venoms, AFFLICT_VENOMS};
use crate::curatives::*;
use crate::io::*;
use crate::timeline::*;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

#[cfg(test)]
mod timeline_tests {
    use super::*;

    #[test]
    fn test_dstab() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                Observation::Devenoms("slike".into()),
                Observation::Devenoms("kalmia".into()),
            ],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), false);
        assert_eq!(seur_state.get_flag(FType::Anorexia), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), true);
        assert_eq!(bene_state.get_flag(FType::Anorexia), true);
    }

    #[test]
    fn test_dstab_rebounds() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Assassination".to_string(),
                    skill: "Doublestab".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
                Observation::Devenoms("slike".into()),
                Observation::Rebounds,
                Observation::Devenoms("kalmia".into()),
                Observation::Rebounds,
            ],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::Asthma), true);
        assert_eq!(seur_state.get_flag(FType::Anorexia), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::Asthma), false);
        assert_eq!(bene_state.get_flag(FType::Anorexia), false);
    }

    #[test]
    fn test_bite() {
        let mut timeline = Timeline::new();
        let dstab_slice = TimeSlice {
            observations: vec![Observation::CombatAction(CombatAction {
                caster: "Seurimas".to_string(),
                category: "Assassination".to_string(),
                skill: "Bite".to_string(),
                target: "Benedicto".to_string(),
                annotation: "scytherus".to_string(),
            })],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(dstab_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Balance), false);
        assert_eq!(seur_state.get_flag(FType::ThinBlood), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Balance), true);
        assert_eq!(bene_state.get_flag(FType::ThinBlood), true);
    }

    #[test]
    fn test_suggest() {
        let mut timeline = Timeline::new();
        let suggest_slice = TimeSlice {
            observations: vec![
                Observation::Sent("suggestion Benedicto stupidity".to_string()),
                Observation::CombatAction(CombatAction {
                    caster: "Seurimas".to_string(),
                    category: "Hypnosis".to_string(),
                    skill: "Suggest".to_string(),
                    target: "Benedicto".to_string(),
                    annotation: "".to_string(),
                }),
            ],
            prompt: Prompt::Blackout,
            time: 0,
        };
        timeline.push_time_slice(suggest_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Equil), false);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(
            bene_state.hypnosis_stack.get(0),
            Some(&Hypnosis::Aff(FType::Stupidity))
        );
    }
}

lazy_static! {
    static ref SUGGESTION: Regex = Regex::new(r"suggest (\w+) (.*)").unwrap();
}

lazy_static! {
    static ref ACTION: Regex = Regex::new(r"action (.*)").unwrap();
}

pub fn infer_suggestion(name: &String, agent_states: &mut TimelineState) -> Hypnosis {
    if let Some(suggestion) = agent_states.get_player_hint(name, &"suggestion".into()) {
        if let Some(captures) = ACTION.captures(&suggestion) {
            Hypnosis::Action(captures.get(1).unwrap().as_str().to_string())
        } else {
            if let Some(aff) = FType::from_name(&suggestion) {
                println!("Good {:?}", aff);
                Hypnosis::Aff(aff)
            } else {
                println!("Bad {}", suggestion);
                Hypnosis::Aff(FType::Impatience)
            }
        }
    } else {
        Hypnosis::Aff(FType::Impatience)
    }
}

pub fn handle_sent(command: &String, agent_states: &mut TimelineState) {
    if let Some(captures) = SUGGESTION.captures(command) {
        agent_states.add_player_hint(
            captures.get(1).unwrap().as_str(),
            &"suggestion",
            captures
                .get(2)
                .unwrap()
                .as_str()
                .to_string()
                .to_ascii_lowercase(),
        );
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut TimelineState,
    before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Doublestab" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_weapon_hits(&mut me, &mut you, after)?;
            apply_or_infer_balance(&mut me, (BType::Balance, 2.8), after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Bite" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            apply_venom(&mut you, &combat_action.annotation)?;
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), after);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Flay" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            me.set_balance(BType::Balance, 1.9);
            apply_or_infer_balance(&mut me, (BType::Balance, 1.9), after);
            apply_weapon_hits(&mut me, &mut you, after)?;
            match combat_action.annotation.as_ref() {
                "rebounding" => {
                    you.set_flag(FType::Rebounding, false);
                }
                "failure-rebounding" => {
                    you.set_flag(FType::Rebounding, false);
                }
                "fangbarrier" => {
                    you.set_flag(FType::HardenedSkin, false);
                }
                "failure-fangbarrier" => {
                    you.set_flag(FType::HardenedSkin, false);
                }
                "shield" => {
                    you.set_flag(FType::Shield, false);
                }
                "failure-shield" => {
                    you.set_flag(FType::Shield, false);
                }
                _ => {}
            }
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Hypnotise" => {
            let mut you = agent_states.get_agent(&combat_action.target);
            you.set_flag(FType::Hypnotized, true);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Seal" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 3.0), after);
            you.set_flag(FType::Hypnotized, false);
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Suggest" => {
            let mut me = agent_states.get_agent(&combat_action.caster);
            let mut you = agent_states.get_agent(&combat_action.target);
            apply_or_infer_balance(&mut me, (BType::Equil, 2.25), after);
            push_suggestion(
                &mut you,
                infer_suggestion(&combat_action.target, agent_states),
            );
            agent_states.set_agent(&combat_action.caster, me);
            agent_states.set_agent(&combat_action.target, you);
        }
        "Snap" => {
            if let Some(target) =
                agent_states.get_player_hint(&combat_action.caster, &"snap".into())
            {
                let mut you = agent_states.get_agent(&target);
                start_hypnosis(&mut you);
                agent_states.set_agent(&target, you);
            }
        }
        _ => {}
    }
    Ok(())
}

lazy_static! {
    static ref COAG_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Vomiting,
        FType::Haemophilia,
        FType::Asthma,
        FType::Paresis,
        FType::Stuttering,
    ];
}

lazy_static! {
    static ref AGGRO_STACK: Vec<FType> = vec![
        FType::Asthma,
        FType::Clumsiness,
        FType::Stupidity,
        FType::Allergies,
        FType::Dizziness,
        FType::Sensitivity,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref SOFT_STACK: Vec<FType> = vec![
        FType::Asthma,
        FType::Paresis,
        FType::Anorexia,
        FType::Slickness,
    ];
}

lazy_static! {
    static ref STACKING_STRATEGIES: HashMap<String, Vec<FType>> = {
        let mut val = HashMap::new();
        val.insert("coag".into(), COAG_STACK.to_vec());
        val.insert("aggro".into(), AGGRO_STACK.to_vec());
        val
    };
}

lazy_static! {
    static ref HARD_HYPNO: Vec<Hypnosis> = vec![
        Hypnosis::Aff(FType::Stupidity),
        Hypnosis::Aff(FType::Stupidity),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Hypersomnia),
        Hypnosis::Aff(FType::Stupidity),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Hypersomnia),
        Hypnosis::Aff(FType::Hypersomnia),
    ];
}

pub fn get_hypno_str(target: &String, hypno: &Hypnosis) -> String {
    match hypno {
        Hypnosis::Aff(affliction) => format!("suggest {} {:?}", target, affliction),
        Hypnosis::Action(act) => format!("suggest {} action {}", target, act),
    }
}

pub fn start_hypnosis(who: &mut AgentState) {
    who.set_flag(FType::Snapped, true);
}

pub fn get_top_hypno(name: &String, target: &AgentState, hypnos: &Vec<Hypnosis>) -> Option<String> {
    let mut hypno_idx = 0;
    let mut hypno = None;
    for i in 0..target.hypnosis_stack.len() {
        if target.hypnosis_stack.get(i) == hypnos.get(hypno_idx) {
            hypno_idx += 1;
        }
    }
    if hypno_idx < hypnos.len() {
        if let Some(next_hypno) = hypnos.get(hypno_idx) {
            hypno = Some(get_hypno_str(name, next_hypno));
        }
    }
    if let Some(suggestion) = hypno {
        if !target.get_flag(FType::Hypnotized) {
            Some(format!("hypnotise {};;{}", name, suggestion))
        } else {
            Some(suggestion)
        }
    } else if target.get_flag(FType::Hypnotized) {
        Some(format!("seal {} 3", name))
    } else {
        None
    }
}

pub fn get_balance_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    if let Some(stack) = STACKING_STRATEGIES.get(strategy) {
        let you = topper.timeline.state.borrow_agent(target);
        if get_equil_attack(topper, target, strategy).starts_with("seal") {
            "".into()
        } else if you.is(FType::Shield) || you.is(FType::Rebounding) {
            let defense = if you.is(FType::Shield) {
                "shield"
            } else {
                "rebounding"
            };
            if let Some(venom) = get_venoms(stack.to_vec(), 1, &you).pop() {
                return format!("flay {} {} {}", target, defense, venom);
            } else {
                return format!("flay {} {}", target, defense);
            }
        } else {
            let mut venoms = get_venoms(SOFT_STACK.to_vec(), 3, &you);
            if venoms.len() > 2 {
                venoms = get_venoms(stack.to_vec(), 2, &you);
            }
            let v1 = venoms.pop();
            let v2 = venoms.pop();
            if let (Some(v1), Some(v2)) = (v1, v2) {
                return format!("dstab {} {} {}", target, v1, v2);
            } else if let Some(v1) = v1 {
                return format!("dstab {} {} {}", target, v1, "delphinium");
            } else {
                return format!("dstab {} {} {}", target, "delphinium", "delphinium");
            }
        }
    } else if strategy == "damage" {
        let you = topper.timeline.state.borrow_agent(target);
        if you.is(FType::HardenedSkin) {
            return format!("flay {} hardenedskin", target);
        } else {
            return format!("bite {} camus", target);
        }
    } else {
        "".into()
    }
}

pub fn get_equil_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    let you = topper.timeline.state.borrow_agent(target);
    let hypno_action = get_top_hypno(target, &you, &HARD_HYPNO.to_vec());
    hypno_action.unwrap_or("".into())
}

pub fn get_shadow_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    let you = topper.timeline.state.borrow_agent(target);
    if (you.get_flag(FType::Void) || you.get_flag(FType::WeakVoid)) && !you.get_flag(FType::Snapped)
    {
        format!("shadow sleight void {}", target)
    } else {
        format!("shadow sleight dissipate {}", target)
    }
}

pub fn get_snap(topper: &Topper, target: &String, strategy: &String) -> bool {
    let you = topper.timeline.state.borrow_agent(target);
    if get_top_hypno(target, &you, &HARD_HYPNO.to_vec()) == None
        && !you.get_flag(FType::Snapped)
        && !you.get_flag(FType::Hypnotized)
        && !you.balanced(BType::Tree)
    {
        return true;
    } else {
        return false;
    }
}

pub fn get_attack(topper: &Topper, target: &String, strategy: &String) -> String {
    let balance = get_balance_attack(topper, target, strategy);
    let equil = get_equil_attack(topper, target, strategy);
    let shadow = get_shadow_attack(topper, target, strategy);
    let should_snap = get_snap(topper, target, strategy);
    let mut attack: String = if should_snap {
        format!("snap {}", target)
    } else {
        "".to_string()
    };
    if balance != "" {
        attack = format!("qeb {}", balance);
    }
    if equil != "" {
        attack = format!("{};;{}", attack, equil);
    }
    if shadow != "" {
        attack = format!("{}%%qs {}", attack, shadow);
    }
    attack
}

pub fn get_offensive_actions() -> Vec<StateAction> {
    let mut actions = vec![];
    // Aggro Stack
    actions.push(dstab_stack(vec![
        FType::Paresis,
        FType::Asthma,
        FType::ThinBlood,
        FType::Stupidity,
        FType::Vomiting,
        FType::Allergies,
        FType::Anorexia,
        FType::Slickness,
    ]));
    // Coag Stack
    actions.push(dstab_stack(COAG_STACK.to_vec()));
    // Salve Stack
    actions.push(dstab_stack(vec![
        FType::LeftLegBroken,
        FType::RightLegBroken,
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::Anorexia,
        FType::Slickness,
        FType::Asthma,
    ]));
    actions
}

#[cfg(test)]
mod simulation_tests {
    use super::*;

    #[test]
    fn test_dstab_stack() {
        let salve_stack = dstab_stack(vec![
            FType::LeftLegBroken,
            FType::RightLegBroken,
            FType::LeftArmBroken,
            FType::RightArmBroken,
            FType::Anorexia,
        ]);
        let mut simulation = SimulationState::new(&vec![BASE_STATE.clone(), BASE_STATE.clone()]);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), true);
        assert_eq!(simulation.states[1].is(FType::Anorexia), true);
    }

    #[test]
    fn test_flay_stack() {
        let salve_stack = flay_stack(vec![
            FType::LeftLegBroken,
            FType::RightLegBroken,
            FType::LeftArmBroken,
            FType::RightArmBroken,
            FType::Anorexia,
        ]);
        let mut simulation = SimulationState::new(&vec![BASE_STATE.clone(), BASE_STATE.clone()]);
        simulation.states[1].set_flag(FType::Shield, true);
        simulation.states[1].set_flag(FType::Rebounding, true);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), false);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        assert_eq!(simulation.states[1].is(FType::Shield), false);
        assert_eq!(simulation.states[1].is(FType::Rebounding), true);
        simulation.apply_action(&salve_stack, 0, 1);
        assert_eq!(simulation.states[1].is(FType::LeftLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::RightLegBroken), true);
        assert_eq!(simulation.states[1].is(FType::LeftArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::RightArmBroken), false);
        assert_eq!(simulation.states[1].is(FType::Anorexia), false);
        assert_eq!(simulation.states[1].is(FType::Shield), false);
        assert_eq!(simulation.states[1].is(FType::Rebounding), false);
    }
}

pub fn dstab_stack(afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("dstab {:?}", afflictions),
        changes: vec![
            balance_change(BType::Balance, 2.8),
            afflict_in_order(afflictions.clone()),
            afflict_in_order(afflictions.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Rebounding)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks_some(afflictions)),
        ],
    }
}

pub fn flay_stack(afflictions: Vec<FType>) -> StateAction {
    let flayable = vec![FType::Shield, FType::Rebounding];
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(flayable.clone()),
            afflict_in_order(afflictions.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(flayable)),
        ],
    }
}

pub fn dstab_action(
    (venom1, affliction1): (String, FType),
    (venom2, affliction2): (String, FType),
) -> StateAction {
    StateAction {
        name: format!("dstab {} {}", venom1, venom2),
        changes: vec![
            balance_change(BType::Balance, 2.8),
            flag_me(FType::Shield, false),
            afflict(affliction1),
            afflict(affliction2),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Rebounding)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction1)),
            target(lacks(affliction2)),
        ],
    }
}

pub fn bite_one(affliction: FType) -> StateAction {
    StateAction {
        name: "bite".into(),
        changes: vec![
            balance_change(BType::Balance, 1.9),
            flag_me(FType::Shield, false),
            afflict(affliction),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::HardenedSkin)),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction)),
        ],
    }
}

pub fn flay_one(defense: FType) -> StateAction {
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(vec![defense]),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(vec![defense])),
        ],
    }
}

pub fn flay_action() -> StateAction {
    let flayable = vec![FType::Shield, FType::Rebounding, FType::HardenedSkin];
    StateAction {
        name: "flay".into(),
        changes: vec![
            balance_change(BType::Balance, 2.5),
            flag_me(FType::Shield, false),
            strip_in_order(flayable.clone()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(some(flayable)),
        ],
    }
}

pub fn snipe_action(affliction: FType) -> StateAction {
    StateAction {
        name: "snipe".into(),
        changes: vec![
            attack_change(900),
            balance_change(BType::Balance, 3.25),
            afflict(affliction),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Balance),
            has(BType::Equil),
            target(lacks(affliction)),
        ],
    }
}
