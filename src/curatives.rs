use crate::io::Topper;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use std::collections::HashMap;

#[cfg(test)]
mod timeline_tests {
    use super::*;
    use crate::timeline::*;

    #[test]
    fn test_pill() {
        let mut timeline = Timeline::new();
        {
            let mut updated_seur = timeline.state.get_agent(&"Seurimas".to_string());
            updated_seur.set_flag(FType::ThinBlood, true);
            timeline.state.set_agent(&"Seurimas".into(), updated_seur);
        }
        {
            let mut updated_bene = timeline.state.get_agent(&"Benedicto".to_string());
            updated_bene.set_flag(FType::ThinBlood, true);
            timeline.state.set_agent(&"Benedicto".into(), updated_bene);
        }
        let coag_slice = TimeSlice {
            observations: vec![Observation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Pill("coagulation".into()),
            })],
            lines: vec![],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Pill), true);
        assert_eq!(seur_state.get_flag(FType::ThinBlood), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Pill), false);
        assert_eq!(bene_state.get_flag(FType::ThinBlood), false);
    }

    #[test]
    fn test_mending() {
        let mut timeline = Timeline::new();
        {
            let mut updated_seur = timeline.state.get_agent(&"Seurimas".to_string());
            updated_seur.set_flag(FType::LeftArmBroken, true);
            timeline.state.set_agent(&"Seurimas".into(), updated_seur);
        }
        {
            let mut updated_bene = timeline.state.get_agent(&"Benedicto".to_string());
            updated_bene.set_flag(FType::LeftLegBroken, true);
            timeline.state.set_agent(&"Benedicto".into(), updated_bene);
        }
        let coag_slice = TimeSlice {
            observations: vec![Observation::SimpleCureAction(SimpleCureAction {
                caster: "Benedicto".into(),
                cure_type: SimpleCure::Salve("mending".into(), "skin".into()),
            })],
            lines: vec![],
            prompt: Prompt::Blackout,
            time: 0,
            me: "Seurimas".into(),
        };
        timeline.push_time_slice(coag_slice);
        let seur_state = timeline.state.get_agent(&"Seurimas".to_string());
        assert_eq!(seur_state.balanced(BType::Salve), true);
        assert_eq!(seur_state.get_flag(FType::LeftArmBroken), true);
        let bene_state = timeline.state.get_agent(&"Benedicto".to_string());
        assert_eq!(bene_state.balanced(BType::Salve), false);
        assert_eq!(bene_state.get_flag(FType::LeftArmBroken), false);
    }
}
fn noop() -> Box<Fn(&mut AgentState)> {
    Box::new(|_me| {})
}

fn revert_flag(flag: FType, val: bool) -> Box<Fn(&mut AgentState)> {
    Box::new(move |me2: &mut AgentState| me2.set_flag(flag, val))
}

pub fn top_aff(who: &AgentState, afflictions: Vec<FType>) -> Option<FType> {
    let mut top = None;
    for affliction in afflictions.iter() {
        if who.is(*affliction) {
            top = Some(*affliction);
        }
    }
    top
}

/*pub fn add_in_order(
    afflictions: Vec<FType>,
) -> Box<Fn(&mut AgentState) -> Box<Fn(&mut AgentState)>> {
    Box::new(move |me| {
        let mut revert = noop();
        for affliction in afflictions.iter() {
            if !me.is(*affliction) {
                revert = revert_flag(*affliction, false);
                me.set_flag(*affliction, true);
                break;
            }
        }
        revert
    })
}*/

pub fn remove_in_order(
    afflictions: Vec<FType>,
) -> Box<Fn(&mut AgentState) -> Box<Fn(&mut AgentState)>> {
    Box::new(move |me| {
        let mut revert = noop();
        for affliction in afflictions.iter() {
            if me.is(*affliction) {
                revert = revert_flag(*affliction, true);
                me.set_flag(*affliction, false);
                break;
            }
        }
        revert
    })
}

lazy_static! {
    pub static ref MENTAL_AFFLICTIONS: Vec<FType> = vec![
        FType::Egocentric,
        FType::Stupidity,
        FType::Anorexia,
        FType::Epilepsy,
        FType::Mirroring,
        FType::MentalDisruption,
        FType::Peace,
        FType::Paranoia,
        FType::Hallucinations,
        FType::Dizziness,
        FType::Indifference,
        FType::Berserking,
        FType::Pacifism,
        FType::LoversEffect,
        FType::Laxity,
        FType::Hatred,
        FType::Generosity,
        FType::Claustrophobia,
        FType::Vertigo,
        FType::Faintness,
        FType::Loneliness,
        FType::Agoraphobia,
        FType::Masochism,
        FType::Recklessness,
        FType::Weariness,
        FType::Impatience,
        FType::Confusion,
        FType::Dementia,
        FType::Nyctophobia,
        // Premonition
    ];
}

lazy_static! {
    static ref AFFLICTIONS: Vec<FType> = vec![];
}

lazy_static! {
    static ref ANTIPSYCHOTIC_ORDER: Vec<FType> = vec![
        FType::Sadness,
        FType::Confusion,
        FType::Dementia,
        FType::Hallucinations,
        FType::Hallucinations,
        FType::Paranoia,
        FType::Hatred,
        FType::Addiction,
        FType::Hypersomnia,
        FType::BloodCurse,
        FType::Blighted,
    ];
}

lazy_static! {
    static ref EUPHORIANT_ORDER: Vec<FType> = vec![
        FType::SelfPity,
        FType::Stupidity,
        FType::Dizziness,
        FType::Faintness,
        FType::Shyness,
        FType::Epilepsy,
        FType::Impatience,
        FType::Dissonance,
        FType::Infested,
    ];
}

lazy_static! {
    static ref DECONGESTANT_ORDER: Vec<FType> = vec![
        FType::Baldness,
        FType::Clumsiness,
        FType::Hypochondria,
        FType::Weariness,
        FType::Asthma,
        FType::Sensitivity,
        FType::RingingEars,
        FType::Impairment,
        FType::BloodPoison,
    ];
}

lazy_static! {
    static ref DEPRESSANT_ORDER: Vec<FType> = vec![
        FType::CommitmentFear,
        FType::Merciful,
        FType::Recklessness,
        FType::Egocentric,
        FType::Masochism,
        FType::Agoraphobia,
        FType::Loneliness,
        FType::Berserking,
        FType::Vertigo,
        FType::Claustrophobia,
        FType::Nyctophobia,
    ];
}

lazy_static! {
    static ref COAGULATION_ORDER: Vec<FType> = vec![
        FType::BodyOdor,
        FType::Lethargy,
        FType::Allergies,
        FType::MentalDisruption,
        FType::PhysicalDisruption,
        FType::Vomiting,
        FType::Exhausted,
        FType::ThinBlood,
        FType::Rend,
        FType::Haemophilia,
    ];
}

lazy_static! {
    static ref STEROID_ORDER: Vec<FType> = vec![
        FType::Hubris,
        FType::Pacifism,
        FType::Peace,
        FType::Soulburn,
        FType::LimpVeins,
        FType::LoversEffect,
        FType::Laxity,
        FType::Superstition,
        FType::Generosity,
        FType::Justice,
        FType::Magnanimity,
    ];
}

lazy_static! {
    static ref OPIATE_ORDER: Vec<FType> = vec![
        FType::Paralysis,
        FType::Paresis,
        FType::Mirroring,
        FType::CrippledBody,
        FType::Crippled,
        FType::Blisters,
        FType::Slickness,
        FType::Heartflutter,
        FType::Sandrot,
    ];
}

lazy_static! {
    pub static ref PILL_CURE_ORDERS: HashMap<String, Vec<FType>> = {
        let mut val = HashMap::new();
        val.insert("antipsychotic".into(), ANTIPSYCHOTIC_ORDER.to_vec());
        val.insert("euphoriant".into(), EUPHORIANT_ORDER.to_vec());
        val.insert("decongestant".into(), DECONGESTANT_ORDER.to_vec());
        val.insert("depressant".into(), DEPRESSANT_ORDER.to_vec());
        val.insert("coagulation".into(), COAGULATION_ORDER.to_vec());
        val.insert("steroid".into(), STEROID_ORDER.to_vec());
        val.insert("opiate".into(), OPIATE_ORDER.to_vec());
        val
    };
}

lazy_static! {
    pub static ref PILL_DEFENCES: HashMap<String, FType> = {
        let mut val = HashMap::new();
        val.insert("thanatonin".into(), FType::Deathsight);
        val.insert("stimulant".into(), FType::Instawake);
        val.insert("kawhe".into(), FType::Insomnia);
        val.insert("ototoxin".into(), FType::Deafness);
        val.insert("amaurosis".into(), FType::Blindness);
        val.insert("acuity".into(), FType::Thirdeye);
        val.insert("waterbreathing".into(), FType::Waterbreathing);
        val
    };
}

lazy_static! {
    static ref AFFLICTION_PILLS: HashMap<FType, &'static str> = {
        let mut val = HashMap::new();
        for aff in ANTIPSYCHOTIC_ORDER.to_vec() {
            val.insert(aff, "antipsychotic");
        }
        for aff in EUPHORIANT_ORDER.to_vec() {
            val.insert(aff, "euphoriant");
        }
        for aff in DECONGESTANT_ORDER.to_vec() {
            val.insert(aff, "decogestant");
        }
        for aff in DEPRESSANT_ORDER.to_vec() {
            val.insert(aff, "depressant");
        }
        for aff in COAGULATION_ORDER.to_vec() {
            val.insert(aff, "coagulation");
        }
        for aff in STEROID_ORDER.to_vec() {
            val.insert(aff, "steroid");
        }
        for aff in OPIATE_ORDER.to_vec() {
            val.insert(aff, "opiate");
        }
        val
    };
}

lazy_static! {
    static ref EPIDERMAL_HEAD_ORDER: Vec<FType> = vec![
        FType::Indifference,
        FType::Stuttering,
        FType::BlurryVision,
        FType::BurntEyes,
        FType::Gloom,
    ];
}

lazy_static! {
    static ref EPIDERMAL_TORSO_ORDER: Vec<FType> = vec![
        FType::Anorexia,
        FType::Gorged,
        FType::EffusedBlood,
        FType::Hypothermia,
    ];
}

lazy_static! {
    static ref EPIDERMAL_SKIN_ORDER: Vec<FType> = vec![
        FType::Indifference,
        FType::Stuttering,
        FType::BlurryVision,
        FType::BurntEyes,
        FType::Gloom,
        FType::Anorexia,
        FType::Gorged,
        FType::EffusedBlood,
        FType::Hypothermia,
    ];
}

lazy_static! {
    pub static ref CALORIC_TORSO_ORDER: Vec<FType> = vec![FType::Frozen, FType::Shivering,];
}

lazy_static! {
    static ref MENDING_SKIN_ORDER: Vec<FType> = vec![
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    static ref MENDING_ARMS_ORDER: Vec<FType> = vec![FType::LeftArmBroken, FType::RightArmBroken,];
}

lazy_static! {
    static ref MENDING_LEGS_ORDER: Vec<FType> = vec![FType::LeftLegBroken, FType::RightLegBroken,];
}

lazy_static! {
    static ref MENDING_HEAD_ORDER: Vec<FType> = vec![
        FType::HeadBruisedCritical,
        FType::DestroyedThroat,
        FType::CrippledThroat,
        FType::HeadBruisedModerate,
        FType::HeadBruised,
    ];
}

lazy_static! {
    static ref MENDING_TORSO_ORDER: Vec<FType> = vec![
        FType::TorsoBruisedCritical,
        FType::Lightwound,
        FType::Ablaze,
        FType::CrackedRibs,
        FType::TorsoBruisedModerate,
        FType::TorsoBruised,
    ];
}

lazy_static! {
    static ref MENDING_LEFT_ARM_ORDER: Vec<FType> = vec![
        FType::LeftArmBruisedCritical,
        FType::LeftArmBroken,
        FType::LeftArmBruisedModerate,
        FType::LeftArmBruised,
        FType::LeftArmDislocated,
    ];
}

lazy_static! {
    static ref MENDING_RIGHT_ARM_ORDER: Vec<FType> = vec![
        FType::RightArmBruisedCritical,
        FType::RightArmBroken,
        FType::RightArmBruisedModerate,
        FType::RightArmBruised,
        FType::RightArmDislocated,
    ];
}

lazy_static! {
    static ref MENDING_LEFT_LEG_ORDER: Vec<FType> = vec![
        FType::LeftLegBruisedCritical,
        FType::LeftLegBroken,
        FType::LeftLegBruisedModerate,
        FType::LeftLegBruised,
        FType::LeftLegDislocated,
    ];
}

lazy_static! {
    static ref MENDING_RIGHT_LEG_ORDER: Vec<FType> = vec![
        FType::RightLegBruisedCritical,
        FType::RightLegBroken,
        FType::RightLegBruisedModerate,
        FType::RightLegBruised,
        FType::RightLegDislocated,
    ];
}

lazy_static! {
    static ref SOOTHING_HEAD_ORDER: Vec<FType> = vec![FType::Whiplash];
}

lazy_static! {
    static ref SOOTHING_TORSO_ORDER: Vec<FType> =
        vec![FType::Backstrain, FType::MuscleSpasms, FType::Stiffness];
}

lazy_static! {
    static ref SOOTHING_LEGS_ORDER: Vec<FType> = vec![FType::SoreAnkle];
}

lazy_static! {
    static ref SOOTHING_ARMS_ORDER: Vec<FType> = vec![FType::SoreWrist, FType::WeakGrip];
}

lazy_static! {
    static ref SOOTHING_SKIN_ORDER: Vec<FType> = vec![
        FType::Whiplash,
        FType::Backstrain,
        FType::MuscleSpasms,
        FType::Stiffness,
        FType::SoreAnkle,
        FType::SoreWrist,
        FType::WeakGrip
    ];
}

lazy_static! {
    pub static ref SALVE_CURE_ORDERS: HashMap<(String, String), Vec<FType>> = {
        let mut val = HashMap::new();
        val.insert(
            ("mending".into(), "skin".into()),
            MENDING_SKIN_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "head".into()),
            MENDING_HEAD_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "torso".into()),
            MENDING_TORSO_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "legs".into()),
            MENDING_LEGS_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "arms".into()),
            MENDING_ARMS_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "left leg".into()),
            MENDING_LEFT_LEG_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "right leg".into()),
            MENDING_RIGHT_LEG_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "left arm".into()),
            MENDING_LEFT_ARM_ORDER.to_vec(),
        );
        val.insert(
            ("mending".into(), "right arm".into()),
            MENDING_RIGHT_ARM_ORDER.to_vec(),
        );

        val.insert(
            ("epidermal".into(), "torso".into()),
            EPIDERMAL_TORSO_ORDER.to_vec(),
        );
        val.insert(
            ("epidermal".into(), "head".into()),
            EPIDERMAL_HEAD_ORDER.to_vec(),
        );
        val.insert(
            ("epidermal".into(), "skin".into()),
            EPIDERMAL_SKIN_ORDER.to_vec(),
        );

        val.insert(
            ("caloric".into(), "torso".into()),
            CALORIC_TORSO_ORDER.to_vec(),
        );
        val.insert(
            ("caloric".into(), "skin".into()),
            CALORIC_TORSO_ORDER.to_vec(),
        );

        val.insert(
            ("soothing".into(), "skin".into()),
            SOOTHING_SKIN_ORDER.to_vec(),
        );
        val.insert(
            ("soothing".into(), "head".into()),
            SOOTHING_HEAD_ORDER.to_vec(),
        );
        val.insert(
            ("soothing".into(), "torso".into()),
            SOOTHING_TORSO_ORDER.to_vec(),
        );
        val.insert(
            ("soothing".into(), "legs".into()),
            SOOTHING_LEGS_ORDER.to_vec(),
        );
        val.insert(
            ("soothing".into(), "arms".into()),
            SOOTHING_ARMS_ORDER.to_vec(),
        );
        val
    };
}

lazy_static! {
    static ref AFFLICTION_SALVES: HashMap<FType, (String, String)> = {
        let mut val = HashMap::new();
        for (key, affs) in SALVE_CURE_ORDERS.iter() {
            for aff in affs {
                val.insert(*aff, key.clone());
            }
        }
        val
    };
}

lazy_static! {
    static ref WILLOW_ORDER: Vec<FType> = vec![FType::Aeon, FType::Hellsight, FType::Deadening,];
}

lazy_static! {
    static ref YARROW_ORDER: Vec<FType> = vec![
        FType::Slickness,
        FType::Withering,
        FType::Disfigurement,
        FType::Migraine,
        FType::Squelched,
    ];
}

lazy_static! {
    pub static ref SMOKE_CURE_ORDERS: HashMap<String, Vec<FType>> = {
        let mut val = HashMap::new();
        val.insert("yarrow".into(), YARROW_ORDER.to_vec());
        val.insert("willow".into(), WILLOW_ORDER.to_vec());
        val
    };
}

lazy_static! {
    static ref AFFLICTION_SMOKES: HashMap<FType, &'static str> = {
        let mut val = HashMap::new();
        for aff in YARROW_ORDER.to_vec() {
            val.insert(aff, "yarrow");
        }
        for aff in WILLOW_ORDER.to_vec() {
            val.insert(aff, "willow");
        }
        val
    };
}

impl ActiveTransition for SimpleCureAction {
    fn simulate(&self, timeline: &Timeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![Observation::SimpleCureAction(self.clone())])
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
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
    fn simulate(&self, timeline: &Timeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"",
            &"Survival",
            &"Focus",
            &"",
        )])
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
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
    fn simulate(&self, timeline: &Timeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"",
            &"Tattoos",
            &"Tree",
            &"",
        )])
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
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
    fn simulate(&self, timeline: &Timeline) -> Vec<ProbableEvent> {
        match self {
            FirstAidAction::Simple(action) => action.simulate(&timeline),
            FirstAidAction::Focus(action) => action.simulate(&timeline),
            FirstAidAction::Tree(action) => action.simulate(&timeline),
            FirstAidAction::Wait => vec![],
        }
    }
    fn act(&self, timeline: &Timeline) -> ActivateResult {
        match self {
            FirstAidAction::Simple(action) => action.act(&timeline),
            FirstAidAction::Focus(action) => action.act(&timeline),
            FirstAidAction::Tree(action) => action.act(&timeline),
            FirstAidAction::Wait => Ok("".to_string()),
        }
    }
}

static FIRST_AID_BLOCK: &'static str = "Your affliction curing priorities:
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
    pill:     [sandrot, clumsiness, thin_blood]
    special:  [vinethorns]

4)  pipe:     [disfigurement, migraine]
    poultice: [left_leg_broken, right_leg_broken, firstaid_predict_arms,
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
               blood_poison, plodding, idiocy, blighted, merciful, soulfire,
               soulburn]

6)  pipe:     [squelched]
    poultice: [shivering, frozen, gorged, effused_blood, blurry_vision,
               smashed_throat, right_arm_broken, left_arm_broken, cracked_ribs,
               whiplash, backstrain, collapsed_lung, left_arm_dislocated,
               left_leg_dislocated, right_arm_dislocated, right_leg_dislocated,
               sore_wrist, sore_ankle, muscle_spasms, heatspear]
    pill:     [sensitivity, rend, epilepsy, masochism, loneliness, haemophilia,
               lethargy, vomiting, impairment, crippled, allergies,
               shaderot_body, shaderot_benign, shaderot_spirit, shaderot_heat,
               shaderot_wither]

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
        simple_priorities.insert(FType::RightArmBroken, 4);
        simple_priorities.insert(FType::RightLegBroken, 4);
        simple_priorities.insert(FType::LeftLegBroken, 4);
        simple_priorities.insert(FType::LeftArmBroken, 4);
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

pub fn handle_simple_cure_action(
    simple_cure: &SimpleCureAction,
    agent_states: &mut TimelineState,
    _before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    let mut me = agent_states.get_agent(&simple_cure.caster);
    let results = match &simple_cure.cure_type {
        SimpleCure::Pill(_) => {
            apply_or_infer_balance(&mut me, (BType::Pill, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        SimpleCure::Salve(_salve_name, _salve_loc) => {
            apply_or_infer_balance(&mut me, (BType::Salve, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        SimpleCure::Smoke(_) => {
            apply_or_infer_balance(&mut me, (BType::Smoke, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        // _ => Ok(()),
    };
    agent_states.set_agent(&simple_cure.caster, me);
    results
}
