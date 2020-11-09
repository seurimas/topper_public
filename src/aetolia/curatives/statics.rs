use crate::aetolia::types::*;
use std::collections::HashMap;
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
    pub static ref ANTIPSYCHOTIC_ORDER: Vec<FType> = vec![
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
    pub static ref EUPHORIANT_ORDER: Vec<FType> = vec![
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
    pub static ref DECONGESTANT_ORDER: Vec<FType> = vec![
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
    pub static ref DEPRESSANT_ORDER: Vec<FType> = vec![
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
    pub static ref COAGULATION_ORDER: Vec<FType> = vec![
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
    pub static ref STEROID_ORDER: Vec<FType> = vec![
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
    pub static ref OPIATE_ORDER: Vec<FType> = vec![
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
    pub static ref AFFLICTION_PILLS: HashMap<FType, &'static str> = {
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
    pub static ref EPIDERMAL_HEAD_ORDER: Vec<FType> = vec![
        FType::Indifference,
        FType::Stuttering,
        FType::BlurryVision,
        FType::BurntEyes,
        FType::Gloom,
    ];
}

lazy_static! {
    pub static ref EPIDERMAL_TORSO_ORDER: Vec<FType> = vec![
        FType::Anorexia,
        FType::Gorged,
        FType::EffusedBlood,
        FType::Hypothermia,
    ];
}

lazy_static! {
    pub static ref EPIDERMAL_SKIN_ORDER: Vec<FType> = vec![
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
    pub static ref MENDING_SKIN_ORDER: Vec<FType> = vec![
        FType::LeftArmBroken,
        FType::RightArmBroken,
        FType::LeftLegBroken,
        FType::RightLegBroken,
    ];
}

lazy_static! {
    pub static ref MENDING_ARMS_ORDER: Vec<FType> =
        vec![FType::LeftArmBroken, FType::RightArmBroken,];
}

lazy_static! {
    pub static ref MENDING_LEGS_ORDER: Vec<FType> =
        vec![FType::LeftLegBroken, FType::RightLegBroken,];
}

lazy_static! {
    pub static ref MENDING_HEAD_ORDER: Vec<FType> = vec![
        FType::HeadBruisedCritical,
        FType::DestroyedThroat,
        FType::CrippledThroat,
        FType::HeadBruisedModerate,
        FType::HeadBruised,
    ];
}

lazy_static! {
    pub static ref MENDING_TORSO_ORDER: Vec<FType> = vec![
        FType::TorsoBruisedCritical,
        FType::Lightwound,
        FType::Ablaze,
        FType::CrackedRibs,
        FType::TorsoBruisedModerate,
        FType::TorsoBruised,
    ];
}

lazy_static! {
    pub static ref MENDING_LEFT_ARM_ORDER: Vec<FType> = vec![
        FType::LeftArmBruisedCritical,
        FType::LeftArmBroken,
        FType::LeftArmBruisedModerate,
        FType::LeftArmBruised,
        FType::LeftArmDislocated,
    ];
}

lazy_static! {
    pub static ref MENDING_RIGHT_ARM_ORDER: Vec<FType> = vec![
        FType::RightArmBruisedCritical,
        FType::RightArmBroken,
        FType::RightArmBruisedModerate,
        FType::RightArmBruised,
        FType::RightArmDislocated,
    ];
}

lazy_static! {
    pub static ref MENDING_LEFT_LEG_ORDER: Vec<FType> = vec![
        FType::LeftLegBruisedCritical,
        FType::LeftLegBroken,
        FType::LeftLegBruisedModerate,
        FType::LeftLegBruised,
        FType::LeftLegDislocated,
    ];
}

lazy_static! {
    pub static ref MENDING_RIGHT_LEG_ORDER: Vec<FType> = vec![
        FType::RightLegBruisedCritical,
        FType::RightLegBroken,
        FType::RightLegBruisedModerate,
        FType::RightLegBruised,
        FType::RightLegDislocated,
    ];
}

lazy_static! {
    pub static ref SOOTHING_HEAD_ORDER: Vec<FType> = vec![FType::Whiplash];
}

lazy_static! {
    pub static ref SOOTHING_TORSO_ORDER: Vec<FType> =
        vec![FType::Backstrain, FType::MuscleSpasms, FType::Stiffness];
}

lazy_static! {
    pub static ref SOOTHING_LEGS_ORDER: Vec<FType> = vec![FType::SoreAnkle];
}

lazy_static! {
    pub static ref SOOTHING_ARMS_ORDER: Vec<FType> = vec![FType::SoreWrist, FType::WeakGrip];
}

lazy_static! {
    pub static ref SOOTHING_SKIN_ORDER: Vec<FType> = vec![
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
    pub static ref AFFLICTION_SALVES: HashMap<FType, (String, String)> = {
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
    pub static ref WILLOW_ORDER: Vec<FType> =
        vec![FType::Aeon, FType::Hellsight, FType::Deadening,];
}

lazy_static! {
    pub static ref YARROW_ORDER: Vec<FType> = vec![
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
    pub static ref AFFLICTION_SMOKES: HashMap<FType, &'static str> = {
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
