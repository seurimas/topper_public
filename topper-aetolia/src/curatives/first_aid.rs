use super::statics::*;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::{Regex, RegexSet};
use std::collections::HashMap;
use topper_core::observations::strip_ansi;
use topper_core::timeline::BaseAgentState;

impl ActiveTransition for SimpleCureAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![AetObservation::SimpleCureAction(self.clone())])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        match &self.cure_type {
            SimpleCure::Pill(pill) => Ok(format!("eat {}", pill)),
            SimpleCure::Salve(salve, location) => Ok(format!("apply {} to {}", salve, location)),
            SimpleCure::Smoke(herb) => Ok(format!("smoke {}", herb)),
        }
    }
}

pub struct FocusAction {
    caster: String,
}

impl FocusAction {
    pub fn new(caster: &str) -> Self {
        FocusAction {
            caster: caster.to_string(),
        }
    }
}

impl ActiveTransition for FocusAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"Survival",
            &"Focus",
            &"",
            &"",
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok("focus".to_string())
    }
}

pub struct TreeAction {
    caster: String,
}

impl TreeAction {
    pub fn new(caster: &str) -> Self {
        TreeAction {
            caster: caster.to_string(),
        }
    }
}

impl ActiveTransition for TreeAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"Tattoos",
            &"Tree",
            &"",
            &"",
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok("touch tree".to_string())
    }
}

pub enum FirstAidAction {
    Simple(SimpleCureAction),
    Focus(FocusAction),
    Tree(TreeAction),
    Wait,
}

impl FirstAidAction {
    pub fn is_tree(&self) -> bool {
        match self {
            FirstAidAction::Tree(_) => true,
            _ => false,
        }
    }
    pub fn is_focus(&self) -> bool {
        match self {
            FirstAidAction::Focus(_) => true,
            _ => false,
        }
    }
}

impl ActiveTransition for FirstAidAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        match self {
            FirstAidAction::Simple(action) => action.simulate(&timeline),
            FirstAidAction::Focus(action) => action.simulate(&timeline),
            FirstAidAction::Tree(action) => action.simulate(&timeline),
            FirstAidAction::Wait => vec![],
        }
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        match self {
            FirstAidAction::Simple(action) => action.act(&timeline),
            FirstAidAction::Focus(action) => action.act(&timeline),
            FirstAidAction::Tree(action) => action.act(&timeline),
            FirstAidAction::Wait => Ok("".to_string()),
        }
    }
}

static FIRST_AID_BLOCK: &'static str = "\x1b[48;5;232mYour affliction curing priorities:
1)  pipe:     [aeon]
    poultice: [anorexia, indifference, destroyed_throat]
    pill:     [paralysis, crippled_body, paresis]
    special:  [asleep, voyria, writhe_gunk, writhe_grappled, writhe_stasis,
               writhe_web, writhe_vines, writhe_bind, writhe_transfix,
               writhe_ropes, writhe_impaled, writhe_thighlock,
               writhe_armpitlock, writhe_necklock, dazed, writhe_hoist,
               writhe_lure, itchy]

2)  pipe:     [slickness, hellsight]
    poultice: [head_mangled, crushed_chest, burnt_skin, head_bruised_critical]
    pill:     [asthma, limp_veins, ringing_ears]
    special:  [disrupted]

3)  pipe:     [withering]
    poultice: [left_arm_amputated, right_arm_amputated, left_leg_amputated,
               right_leg_amputated, left_leg_damaged, right_leg_damaged,
               right_leg_mangled, left_leg_mangled, right_arm_mangled,
               left_arm_mangled, left_leg_bruised_critical,
               right_leg_bruised_critical, right_arm_bruised_critical,
               left_arm_bruised_critical, torso_bruised_critical, voidgaze]
    pill:     [slough, clumsiness, thin_blood]
    special:  [vinethorns]

4)  pipe:     [disfigurement, migraine]
    poultice: [left_leg_crippled, right_leg_crippled, firstaid_predict_arms,
               firstaid_predict_legs, firstaid_predict_any_limb]
    pill:     [impatience, recklessness, baldness, hypochondria, weariness,
               pacifism, mirroring, infested, patterns]

5)  pipe:     [deadening]
    poultice: [spinal_rip, head_damaged, torso_damaged, left_arm_damaged,
               right_arm_damaged, torso_mangled, left_arm_bruised,
               right_arm_bruised, right_leg_bruised, left_leg_bruised,
               head_bruised, torso_bruised, left_leg_bruised_moderate,
               right_leg_bruised_moderate, right_arm_bruised_moderate,
               left_arm_bruised_moderate, torso_bruised_moderate,
               head_bruised_moderate, gloom]
    pill:     [physical_disruption, mental_disruption, confusion, blood_curse,
               blood_poison, plodding, idiocy, blighted, merciful, accursed,
               agony]

6)  pipe:     [squelched]
    poultice: [shivering, frozen, gorged, effused_blood, blurry_vision,
               smashed_throat, right_arm_crippled, left_arm_crippled, cracked_ribs,
               whiplash, backstrain, collapsed_lung, left_arm_dislocated,
               left_leg_dislocated, right_arm_dislocated, right_leg_dislocated,
               sore_wrist, sore_ankle, muscle_spasms, heatspear]
    pill:     [sensitivity, rend, epilepsy, masochism, loneliness, haemophilia,
               lethargy, vomiting, impairment, crippled, allergies,
               rot_body, rot_benign, rot_spirit, rot_heat,
               rot_wither]

7)  poultice: [ablaze, hypothermia, stuttering, crippled_throat, mauled_face,
               deepwound, stiffness, weak_grip]
    pill:     [stupidity, heartflutter, hallucinations, hypersomnia, hatred,
               peace, berserking, justice, lovers_effect, laxity, egocentric,
               exhausted]
    special:  [premonition]

8)  poultice: [burnt_eyes, lightwound]
    pill:     [dementia, paranoia, dizziness, shyness, dissonance, agoraphobia,
               vertigo, claustrophobia, faintness]
    special:  [fear]

9)  pill:     [sadness, addiction, self-pity, commitment_fear, hubris,
               body_odor, magnanimity]

10) poultice: [pre-restore right arm (20%), pre-restore right leg (20%)]
    pill:     [generosity, superstition, blisters]
    special:  [oiled]

11) poultice: [void, weakvoid]

12) poultice: [pre-restore head (15%), pre-restore left leg (15%)]

13)
14)
15) poultice: [pre-restore left arm (20%)]

16)
17)
18)
19)
20)
21)
22)
23)
24)
25) poultice: [pre-restore torso (5%)]

26)";

lazy_static! {
    static ref UNNAMED_HEADER: Regex = Regex::new(r"Your affliction curing priorities:").unwrap();
    static ref NAMED_HEADER: Regex =
        Regex::new(r"Your affliction curing priorities for the priority set (\w+):").unwrap();
    static ref PRIORITY_NUM_LINE: Regex =
        Regex::new(r"^(\d+)\)\s+(pipe|poultice|pill|special):\s+\[([a-z_, ]+)\]?$").unwrap();
    static ref PRIORITY_TYPE_LINE: Regex =
        Regex::new(r"^\s+(pipe|poultice|pill|special):\s+\[([a-z_, ]+)\]?$").unwrap();
    static ref PRIORITY_CONTINUITY_LINE: Regex = Regex::new(r"^\s+([a-z_, ]+)\]?$").unwrap();
}

pub type FirstAidPriorities = HashMap<FType, u32>;

fn add_priorities(priorities: &mut FirstAidPriorities, priority: u32, aff_list: &str) {
    for mut aff_str in aff_list.split(", ") {
        aff_str = aff_str.trim_end_matches(&[',', ' '][..]);
        if let Some(aff) = FType::from_name(&aff_str.to_string()) {
            priorities.insert(aff, priority);
        }
    }
}

fn parse_priorities(priority_lines: &Vec<String>) -> FirstAidPriorities {
    let mut priorities = HashMap::new();
    let mut priority = 0;
    for line in priority_lines.iter() {
        if let Some(captures) = PRIORITY_NUM_LINE.captures(&line) {
            priority = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            add_priorities(&mut priorities, priority, captures.get(3).unwrap().as_str());
        } else if let Some(captures) = PRIORITY_TYPE_LINE.captures(&line) {
            add_priorities(&mut priorities, priority, captures.get(2).unwrap().as_str());
        } else if let Some(captures) = PRIORITY_CONTINUITY_LINE.captures(&line) {
            add_priorities(&mut priorities, priority, captures.get(1).unwrap().as_str());
        }
    }
    priorities
}

pub fn parse_priority_set(lines: &Vec<(String, u32)>) -> Option<(String, FirstAidPriorities)> {
    let mut priority_lines = Vec::new();
    let mut priority_name = None;
    for (line, _num) in lines.iter() {
        if let Some(captures) = NAMED_HEADER.captures(&line) {
            priority_name = Some(captures.get(1).unwrap().as_str().to_string());
        } else if let Some(captures) = UNNAMED_HEADER.find(&line) {
            priority_name = Some("".to_string());
        } else if priority_name.is_some() {
            priority_lines.push(strip_ansi(line));
        }
    }
    priority_name.map(|name| (name, parse_priorities(&priority_lines)))
}

#[derive(Debug, Default)]
pub struct FirstAid {
    simple_priorities: HashMap<FType, u32>,
    use_tree: bool,
    use_focus: bool,
}

impl FirstAid {
    pub fn new() -> Self {
        let mut simple_priorities = HashMap::new();
        simple_priorities.insert(FType::Anorexia, 1);
        simple_priorities.insert(FType::Indifference, 1);
        simple_priorities.insert(FType::Paralysis, 1);
        simple_priorities.insert(FType::Paresis, 1);
        simple_priorities.insert(FType::Aeon, 1);

        simple_priorities.insert(FType::Slickness, 2);
        simple_priorities.insert(FType::Asthma, 2);
        simple_priorities.insert(FType::LimpVeins, 2);
        simple_priorities.insert(FType::Hellsight, 2);

        simple_priorities.insert(FType::Clumsiness, 3);
        simple_priorities.insert(FType::ThinBlood, 3);

        simple_priorities.insert(FType::Disfigurement, 4);
        simple_priorities.insert(FType::RightArmCrippled, 4);
        simple_priorities.insert(FType::RightLegCrippled, 4);
        simple_priorities.insert(FType::LeftLegCrippled, 4);
        simple_priorities.insert(FType::LeftArmCrippled, 4);
        simple_priorities.insert(FType::Impatience, 4);
        simple_priorities.insert(FType::Recklessness, 4);
        simple_priorities.insert(FType::Hypochondria, 4);
        simple_priorities.insert(FType::Weariness, 4);
        simple_priorities.insert(FType::Pacifism, 4);

        simple_priorities.insert(FType::Confusion, 5);
        simple_priorities.insert(FType::Merciful, 5);

        simple_priorities.insert(FType::Sensitivity, 6);
        simple_priorities.insert(FType::Epilepsy, 6);
        simple_priorities.insert(FType::Masochism, 6);
        simple_priorities.insert(FType::Loneliness, 6);
        simple_priorities.insert(FType::Haemophilia, 6);
        simple_priorities.insert(FType::Lethargy, 6);
        simple_priorities.insert(FType::Vomiting, 6);
        simple_priorities.insert(FType::Allergies, 6);

        simple_priorities.insert(FType::Stuttering, 7);
        simple_priorities.insert(FType::Stupidity, 7);
        simple_priorities.insert(FType::Hallucinations, 7);
        simple_priorities.insert(FType::Hypersomnia, 7);
        simple_priorities.insert(FType::Berserking, 7);

        simple_priorities.insert(FType::Dementia, 8);
        simple_priorities.insert(FType::Paranoia, 8);
        FirstAid {
            simple_priorities,
            use_tree: true,
            use_focus: true,
        }
    }

    fn best_cure(&self, who_am_i: &str, state: &AgentState, aff: &FType) -> FirstAidAction {
        if let Some(herb) = AFFLICTION_SMOKES.get(aff) {
            if state.can_smoke(false) {
                return FirstAidAction::Simple(SimpleCureAction::smoke(&who_am_i, &herb));
            }
        }
        if let Some(pill) = AFFLICTION_PILLS.get(aff) {
            if state.can_pill(false) {
                return FirstAidAction::Simple(SimpleCureAction::pill(&who_am_i, &pill));
            }
        }
        if let Some((salve, location)) = AFFLICTION_SALVES.get(aff) {
            if state.can_salve(false) {
                return FirstAidAction::Simple(SimpleCureAction::salve(
                    &who_am_i, &salve, &location,
                ));
            }
        }
        // if let Some(elixir) = AFFLICTION_ELIXIRS.get(aff) {
        //     format!("sip {}", elixir)
        // }
        if self.use_focus && MENTAL_AFFLICTIONS.to_vec().contains(aff) && state.can_focus(false) {
            return FirstAidAction::Focus(FocusAction::new(&who_am_i));
        } else if self.use_tree && state.can_tree(false) {
            return FirstAidAction::Tree(TreeAction::new(&who_am_i));
        } else {
            return FirstAidAction::Wait;
        }
    }

    pub fn get_cure(&self, who_am_i: &str, state: &AgentState) -> Option<(FType, FirstAidAction)> {
        let mut top_priority: Option<(FType, u32, FirstAidAction)> = None;
        for aff in state.flags.aff_iter() {
            if let Some(priority) = self.simple_priorities.get(&aff) {
                match top_priority {
                    Some((aff, top, _)) => {
                        if *priority < top {
                            match self.best_cure(&who_am_i, state, &aff) {
                                FirstAidAction::Wait => {}
                                cure => {
                                    top_priority = Some((aff, *priority, cure));
                                }
                            }
                        }
                    }
                    None => match self.best_cure(&who_am_i, state, &aff) {
                        FirstAidAction::Wait => {}
                        cure => {
                            top_priority = Some((aff, *priority, cure));
                        }
                    },
                }
            }
        }
        top_priority.map(|(aff, _, cure)| (aff, cure))
    }

    pub fn get_next_cure(
        &self,
        who_am_i: &str,
        state: &AgentState,
    ) -> Option<(FType, FirstAidAction)> {
        if let Some(cure) = self.get_cure(&who_am_i, &state) {
            return Some(cure);
        }
        let mut viable_balances = vec![];
        if state.can_pill(true) && !state.balanced(BType::Pill) {
            viable_balances.push(BType::Pill);
        }
        if state.can_salve(true) && !state.balanced(BType::Salve) {
            viable_balances.push(BType::Salve);
        }
        if state.can_smoke(true) && !state.balanced(BType::Smoke) {
            viable_balances.push(BType::Smoke);
        }
        if state.can_tree(true) && !state.balanced(BType::Tree) {
            viable_balances.push(BType::Tree);
        }
        if state.can_focus(true) && !state.balanced(BType::Focus) {
            viable_balances.push(BType::Focus);
        }
        if let Some(balance) = state.next_balance(viable_balances.iter()) {
            let mut state = state.clone();
            state.wait(state.get_raw_balance(balance));
            return self.get_next_cure(&who_am_i, &state);
        } else {
            None
        }
    }
}

#[cfg(test)]
#[path = "./tests/first_aid_tests.rs"]
mod first_aid_tests;
