use crate::actions::*;
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
pub fn heal_action(name: String, heal: CType) -> StateAction {
    StateAction {
        name,
        changes: vec![
            heal_change(heal),
            balance_change(BType::Elixir, 6.0),
            tick(SType::Sips),
        ],
        initial: vec![alive(), target(alive()), has(BType::Elixir)],
    }
}

pub fn shield_action(name: String) -> StateAction {
    StateAction {
        name,
        changes: vec![
            balance_change(BType::Equil, 4.0),
            flag_me(FType::Shield, true),
            tick(SType::Shields),
        ],
        initial: vec![
            alive(),
            target(alive()),
            lacks(FType::Shield),
            has(BType::Balance),
            has(BType::Equil),
        ],
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

pub fn add_in_order(
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
}

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

pub fn cure_in_order(afflictions: Vec<FType>) -> StateChange {
    apply_me(remove_in_order(afflictions))
}

pub fn strip_in_order(defenses: Vec<FType>) -> StateChange {
    apply_you(remove_in_order(defenses))
}

pub fn afflict_in_order(afflictions: Vec<FType>) -> StateChange {
    apply_you(add_in_order(afflictions))
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

pub fn focus() -> StateAction {
    StateAction {
        name: "focus".into(),
        changes: vec![
            cure_in_order(MENTAL_AFFLICTIONS.to_vec()),
            balance_change(BType::Focus, 5.0),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Focus),
            lacks(FType::Impatience),
            some(MENTAL_AFFLICTIONS.to_vec()),
        ],
    }
}

pub fn tree() -> StateAction {
    StateAction {
        name: "touch tree".into(),
        changes: vec![
            balance_change(BType::Tree, 11.0),
            cure_in_order(AFFLICTIONS.to_vec()),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Tree),
            lacks(FType::Paralysis),
        ],
    }
}

pub fn herb_action(name: String, afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("eat {}", name),
        changes: vec![
            cure_in_order(afflictions.clone()),
            balance_change(BType::Pill, 2.0),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Pill),
            lacks(FType::Anorexia),
            some(afflictions),
        ],
    }
}

pub fn salve_action(name: String, location: String, afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("apply {} to {}", name, location),
        changes: vec![
            cure_in_order(afflictions.clone()),
            balance_change(BType::Salve, 2.0),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Salve),
            lacks(FType::Slickness),
            some(afflictions),
        ],
    }
}

pub fn smoke_action(name: String, afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("smoke {}", name),
        changes: vec![
            cure_in_order(afflictions.clone()),
            balance_change(BType::Smoke, 2.0),
        ],
        initial: vec![
            alive(),
            target(alive()),
            has(BType::Smoke),
            lacks(FType::Asthma),
            some(afflictions),
        ],
    }
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
        val.insert("stimulant".into(), FType::Energetic);
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

pub fn antipsychotic() -> StateAction {
    herb_action("antipsychotic".into(), ANTIPSYCHOTIC_ORDER.to_vec())
}

pub fn euphoriant() -> StateAction {
    herb_action("euphoriant".into(), EUPHORIANT_ORDER.to_vec())
}

pub fn decongestant() -> StateAction {
    herb_action("decongestant".into(), DECONGESTANT_ORDER.to_vec())
}

pub fn depressant() -> StateAction {
    herb_action("depressant".into(), DEPRESSANT_ORDER.to_vec())
}

pub fn coagulation() -> StateAction {
    herb_action("coagulation".into(), COAGULATION_ORDER.to_vec())
}

pub fn steroid() -> StateAction {
    herb_action("steroid".into(), STEROID_ORDER.to_vec())
}

pub fn opiate() -> StateAction {
    herb_action("opiate".into(), OPIATE_ORDER.to_vec())
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

pub fn epidermal_head() -> StateAction {
    salve_action(
        "epidermal".into(),
        "head".into(),
        EPIDERMAL_HEAD_ORDER.to_vec(),
    )
}

pub fn epidermal_torso() -> StateAction {
    salve_action(
        "epidermal".into(),
        "torso".into(),
        EPIDERMAL_TORSO_ORDER.to_vec(),
    )
}

pub fn mending_skin() -> StateAction {
    salve_action("mending".into(), "skin".into(), MENDING_SKIN_ORDER.to_vec())
}

pub fn mending_legs() -> StateAction {
    salve_action("mending".into(), "legs".into(), MENDING_LEGS_ORDER.to_vec())
}

pub fn mending_arms() -> StateAction {
    salve_action("mending".into(), "arms".into(), MENDING_ARMS_ORDER.to_vec())
}

pub fn mending_head() -> StateAction {
    salve_action("mending".into(), "head".into(), MENDING_HEAD_ORDER.to_vec())
}

pub fn mending_torso() -> StateAction {
    salve_action(
        "mending".into(),
        "torso".into(),
        MENDING_TORSO_ORDER.to_vec(),
    )
}

pub fn mending_left_arm() -> StateAction {
    salve_action(
        "mending".into(),
        "left arm".into(),
        MENDING_LEFT_ARM_ORDER.to_vec(),
    )
}

pub fn mending_right_arm() -> StateAction {
    salve_action(
        "mending".into(),
        "right arm".into(),
        MENDING_RIGHT_ARM_ORDER.to_vec(),
    )
}

pub fn mending_left_leg() -> StateAction {
    salve_action(
        "mending".into(),
        "left leg".into(),
        MENDING_LEFT_LEG_ORDER.to_vec(),
    )
}

pub fn mending_right_leg() -> StateAction {
    salve_action(
        "mending".into(),
        "right leg".into(),
        MENDING_RIGHT_LEG_ORDER.to_vec(),
    )
}

pub fn soothing_head() -> StateAction {
    salve_action(
        "soothing".into(),
        "head".into(),
        SOOTHING_HEAD_ORDER.to_vec(),
    )
}

pub fn soothing_torso() -> StateAction {
    salve_action(
        "soothing".into(),
        "torso".into(),
        SOOTHING_TORSO_ORDER.to_vec(),
    )
}

pub fn soothing_arms() -> StateAction {
    salve_action(
        "soothing".into(),
        "arms".into(),
        SOOTHING_ARMS_ORDER.to_vec(),
    )
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

pub fn soothing_legs() -> StateAction {
    salve_action(
        "soothing".into(),
        "legs".into(),
        SOOTHING_LEGS_ORDER.to_vec(),
    )
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

pub fn willow() -> StateAction {
    smoke_action("willow".into(), WILLOW_ORDER.to_vec())
}

pub fn yarrow() -> StateAction {
    smoke_action("yarrow".into(), YARROW_ORDER.to_vec())
}

pub fn get_curative_actions() -> Vec<StateAction> {
    vec![
        //antipsychotic(),
        //euphoriant(),
        decongestant(),
        //depressant(),
        //coagulation(),
        opiate(),
        //steroid(),
        //mending_head(),
        mending_left_arm(),
        mending_right_arm(),
        mending_left_leg(),
        mending_right_leg(),
        //mending_torso(),
        //epidermal_head(),
        epidermal_torso(),
        //soothing_arms(),
        //soothing_legs(),
        //soothing_head(),
        //soothing_torso(),
        willow(),
        yarrow(),
    ]
}

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

        simple_priorities.insert(FType::Slickness, 2);
        simple_priorities.insert(FType::Asthma, 2);
        simple_priorities.insert(FType::LimpVeins, 2);

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
        FirstAid {
            simple_priorities,
            use_tree: true,
            use_focus: true,
        }
    }

    fn best_cure(&self, state: &AgentState, aff: &FType) -> Option<String> {
        if let Some(herb) = AFFLICTION_SMOKES.get(aff) {
            if state.can_smoke() {
                return Some(format!("smoke {}", herb));
            }
        }
        if let Some(pill) = AFFLICTION_PILLS.get(aff) {
            if state.can_pill() {
                return Some(format!("eat {}", pill));
            }
        }
        if let Some((salve, location)) = AFFLICTION_SALVES.get(aff) {
            if state.can_salve() {
                return Some(format!("apply {} to {}", salve, location));
            }
        }
        // if let Some(elixir) = AFFLICTION_ELIXIRS.get(aff) {
        //     format!("sip {}", elixir)
        // }
        if self.use_focus && MENTAL_AFFLICTIONS.to_vec().contains(aff) && state.can_focus(false) {
            Some(format!("focus"))
        } else if self.use_tree && state.can_tree(false) {
            Some(format!("touch tree"))
        } else {
            None
        }
    }

    fn get_cure(&self, state: &AgentState) -> Option<(FType, String)> {
        let mut top_priority: Option<(FType, u32, String)> = None;
        for aff in state.flags.aff_iter() {
            if let Some(priority) = self.simple_priorities.get(&aff) {
                match top_priority {
                    Some((aff, top, _)) => {
                        if *priority > top {
                            if let Some(cure) = self.best_cure(state, &aff) {
                                top_priority = Some((aff, *priority, cure))
                            }
                        }
                    }
                    None => {
                        if let Some(cure) = self.best_cure(state, &aff) {
                            top_priority = Some((aff, *priority, cure))
                        }
                    }
                }
            }
        }
        top_priority.map(|(aff, _, cure)| (aff, cure))
    }
}

pub fn handle_simple_cure_action(
    simple_cure: &SimpleCureAction,
    agent_states: &mut TimelineState,
    before: &Vec<Observation>,
    after: &Vec<Observation>,
) -> Result<(), String> {
    let mut me = agent_states.get_agent(&simple_cure.caster);
    let results = match &simple_cure.cure_type {
        SimpleCure::Pill(_) => {
            apply_or_infer_balance(&mut me, (BType::Pill, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        SimpleCure::Salve(salve_name, salve_loc) => {
            apply_or_infer_balance(&mut me, (BType::Salve, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        SimpleCure::Smoke(_) => {
            apply_or_infer_balance(&mut me, (BType::Smoke, 2.0), after);
            apply_or_infer_cure(&mut me, &simple_cure.cure_type, after)?;
            Ok(())
        }
        _ => Ok(()),
    };
    agent_states.set_agent(&simple_cure.caster, me);
    results
}
