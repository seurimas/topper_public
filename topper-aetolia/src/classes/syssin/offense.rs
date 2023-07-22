use super::*;
use crate::agent::*;
use crate::alpha_beta::ActionPlanner;
use crate::classes::group::*;
use crate::classes::*;
use crate::curatives::get_cure_depth;
use crate::db::AetDatabaseModule;
use crate::defense::*;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;

fn use_one_rag(timeline: &AetTimeline) -> bool {
    check_config(timeline, &"ONE_RAG".to_string())
}

fn should_void(timeline: &AetTimeline) -> bool {
    !check_config(timeline, &"NO_VOID".to_string())
}

lazy_static! {
    static ref COAG_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Vomiting,
        FType::Clumsiness,
        FType::Asthma,
        FType::Shyness,
        FType::Stupidity,
        FType::Paresis,
        FType::Sensitivity,
        FType::LeftLegCrippled,
    ];
}

lazy_static! {
    static ref DEC_STACK: Vec<FType> = vec![
        FType::Clumsiness,
        FType::Weariness,
        FType::Asthma,
        FType::Stupidity,
        FType::Paresis,
        FType::Allergies,
        FType::Vomiting,
        FType::LeftLegCrippled,
        FType::LeftArmCrippled,
        FType::Shyness,
    ];
}

lazy_static! {
    static ref KILL_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Vomiting,
        FType::Sensitivity,
        FType::Recklessness,
        FType::Asthma,
        FType::Paresis,
        FType::Slickness,
        FType::Anorexia,
        FType::LeftLegCrippled,
        FType::LeftArmCrippled,
        FType::RightLegCrippled,
        FType::RightArmCrippled,
    ];
}

lazy_static! {
    static ref FIRE_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Clumsiness,
        FType::Asthma,
        FType::Shyness,
        FType::Stupidity,
        FType::Allergies,
        FType::Vomiting,
        FType::LeftLegCrippled,
        FType::LeftArmCrippled,
        FType::RightLegCrippled,
        FType::RightArmCrippled,
        FType::Voyria,
        FType::Stuttering,
    ];
}

lazy_static! {
    static ref PHYS_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Clumsiness,
        FType::Allergies,
        FType::Vomiting,
        FType::Asthma,
        FType::Dizziness,
        FType::Weariness,
        FType::Slickness,
        FType::LeftArmCrippled,
        FType::RightArmCrippled,
        FType::LeftLegCrippled,
        FType::RightLegCrippled,
    ];
}

lazy_static! {
    static ref GANK_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Asthma,
        FType::Clumsiness,
        FType::Slickness,
        FType::Stupidity,
        FType::Anorexia,
        FType::Dizziness,
        FType::LeftLegCrippled,
        FType::Stuttering,
        FType::RightLegCrippled,
        FType::LeftArmCrippled,
        FType::RightArmCrippled,
    ];
}

lazy_static! {
    static ref MONK_STACK: Vec<FType> = vec![
        FType::Allergies,
        FType::Weariness,
        FType::Paresis,
        FType::Stupidity,
        FType::Dizziness,
        FType::Clumsiness,
        FType::Vomiting,
        FType::Asthma,
        FType::LeftArmCrippled,
        FType::RightArmCrippled,
        FType::LeftLegCrippled,
        FType::RightLegCrippled,
    ];
}

lazy_static! {
    static ref PEACE_STACK: Vec<FType> = vec![
        FType::Paresis,
        FType::Asthma,
        FType::Clumsiness,
        FType::Allergies,
        FType::Stupidity,
        FType::Peace,
        FType::Vomiting,
        FType::Slickness,
        FType::LeftArmCrippled,
        FType::RightArmCrippled,
        FType::Dizziness,
        FType::LeftLegCrippled,
        FType::RightLegCrippled,
    ];
}

lazy_static! {
    static ref YEDAN_STACK: Vec<FType> = vec![
        FType::Slickness,
        FType::Paresis,
        FType::Anorexia,
        FType::Stupidity,
        FType::Clumsiness,
        FType::Weariness,
        FType::Asthma,
        FType::Allergies,
        FType::Dizziness,
        FType::Vomiting,
        FType::LeftLegCrippled,
        FType::RightLegCrippled,
    ];
}

lazy_static! {
    static ref BEDAZZLE_STACK: Vec<VenomPlan> = vec![
        VenomPlan::IfDo(
            FType::Vomiting,
            Box::new(VenomPlan::Stick(FType::Allergies)),
        ),
        VenomPlan::IfDo(
            FType::Weariness,
            Box::new(VenomPlan::Stick(FType::Clumsiness)),
        ),
        VenomPlan::IfDo(FType::Laxity, Box::new(VenomPlan::Stick(FType::Stupidity)),),
        VenomPlan::IfDo(FType::Weariness, Box::new(VenomPlan::Stick(FType::Asthma)),),
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::IfNotDo(
            FType::Weariness,
            Box::new(VenomPlan::Stick(FType::Clumsiness)),
        ),
        VenomPlan::Stick(FType::Slickness),
        VenomPlan::Stick(FType::LeftArmCrippled),
        VenomPlan::Stick(FType::RightArmCrippled),
        VenomPlan::Stick(FType::Vomiting),
        VenomPlan::Stick(FType::LeftLegCrippled),
        VenomPlan::Stick(FType::RightLegCrippled),
    ];
}

lazy_static! {
    static ref AGGRO_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::IfDo(
            FType::Slickness,
            Box::new(VenomPlan::Stick(FType::Stupidity)),
        ),
        VenomPlan::IfNotDo(
            FType::Weariness,
            Box::new(VenomPlan::Stick(FType::Clumsiness)),
        ),
        VenomPlan::Stick(FType::Slickness),
        VenomPlan::IfDo(
            FType::Weariness,
            Box::new(VenomPlan::Stick(FType::Clumsiness)),
        ),
        VenomPlan::IfDo(
            FType::Impatience,
            Box::new(VenomPlan::Stick(FType::Dizziness)),
        ),
        VenomPlan::OffTree(FType::Paresis),
        VenomPlan::Stick(FType::Allergies),
        VenomPlan::Stick(FType::Vomiting),
        VenomPlan::Stick(FType::LeftLegCrippled),
        VenomPlan::Stick(FType::RightLegCrippled),
        VenomPlan::Stick(FType::LeftArmCrippled),
        VenomPlan::Stick(FType::RightArmCrippled),
    ];
}

lazy_static! {
    static ref SALVE_STACK: Vec<FType> = vec![
        FType::LeftLegCrippled,
        FType::RightLegCrippled,
        FType::Anorexia,
        FType::LeftArmCrippled,
        FType::RightArmCrippled,
        FType::Stupidity,
        FType::Stuttering,
        FType::Asthma,
        FType::Slickness,
        FType::Paresis,
    ];
}

lazy_static! {
    static ref SLIT_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::IfNotDo(
            FType::Hypersomnia,
            Box::new(VenomPlan::OneOf(FType::Vomiting, FType::Allergies))
        ),
        VenomPlan::Stick(FType::Haemophilia),
        VenomPlan::OneOf(FType::Stupidity, FType::Dizziness),
        VenomPlan::OneOf(FType::Asthma, FType::Weariness),
        VenomPlan::OneOf(FType::Recklessness, FType::Clumsiness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightArmCrippled),
        VenomPlan::Stick(FType::Anorexia),
    ];
}

lazy_static! {
    static ref THIN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::IfDo(
            FType::ThinBlood,
            Box::new(VenomPlan::OneOf(FType::Vomiting, FType::Allergies))
        ),
        VenomPlan::IfNotDo(
            FType::ThinBlood,
            Box::new(VenomPlan::Stick(FType::Allergies)),
        ),
        VenomPlan::IfNotDo(
            FType::ThinBlood,
            Box::new(VenomPlan::Stick(FType::Vomiting)),
        ),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Clumsiness, FType::Weariness),
        VenomPlan::IfDo(
            FType::Loneliness,
            Box::new(VenomPlan::OneOf(FType::Recklessness, FType::Sensitivity))
        ),
        VenomPlan::Stick(FType::Slickness),
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightArmCrippled),
    ];
}

lazy_static! {
    static ref CARNIFEX_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::IfDo(
            FType::MentalFatigue,
            Box::new(VenomPlan::OneOf(FType::Weariness, FType::Stupidity))
        ),
        VenomPlan::Stick(FType::Vomiting),
        VenomPlan::Stick(FType::Allergies),
        VenomPlan::OneOf(FType::Asthma, FType::Slickness),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightLegCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref SCIOMANCER_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::IfDo(
            FType::MentalFatigue,
            Box::new(VenomPlan::OneOf(FType::Weariness, FType::Stupidity))
        ),
        VenomPlan::Stick(FType::Vomiting),
        VenomPlan::Stick(FType::Allergies),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
        VenomPlan::OneOf(FType::Asthma, FType::Slickness),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightLegCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref WAYFARER_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightLegCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref ZEALOT_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightLegCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref SYSSIN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::Stick(FType::Clumsiness),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightLegCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref PRAENOMEN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::OneOf(FType::Weariness, FType::Stupidity),
        VenomPlan::OneOf(FType::Dizziness, FType::Peace),
        VenomPlan::OneOf(FType::Allergies, FType::Vomiting),
        VenomPlan::OneOf(FType::Haemophilia, FType::Dizziness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightLegCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref INDORANI_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::IfNotDo(
            FType::Hypochondria,
            Box::new(VenomPlan::Stick(FType::Clumsiness))
        ),
        VenomPlan::Stick(FType::Disfigurement),
        VenomPlan::OneOf(FType::Allergies, FType::Weariness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightLegCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref LUMINARY_STACK: Vec<VenomPlan> = vec![
        VenomPlan::OnTree(FType::Paresis),
        VenomPlan::Stick(FType::Asthma),
        VenomPlan::IfNotDo(
            FType::Hypochondria,
            Box::new(VenomPlan::Stick(FType::Clumsiness))
        ),
        VenomPlan::OneOf(FType::Paresis, FType::Allergies),
        VenomPlan::OneOf(FType::Peace, FType::Vomiting),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightArmCrippled),
        VenomPlan::OneOf(FType::Sensitivity, FType::Dizziness),
    ];
}

lazy_static! {
    static ref SHAMAN_STACK: Vec<VenomPlan> = vec![
        VenomPlan::Stick(FType::Allergies),
        VenomPlan::Stick(FType::Paresis),
        VenomPlan::OneOf(FType::Asthma, FType::Clumsiness),
        VenomPlan::OneOf(FType::Vomiting, FType::Stupidity),
        VenomPlan::OneOf(FType::Peace, FType::Weariness),
        VenomPlan::OneOf(FType::Slickness, FType::Anorexia),
        VenomPlan::OneOf(FType::LeftLegCrippled, FType::LeftArmCrippled),
        VenomPlan::OneOf(FType::RightLegCrippled, FType::RightArmCrippled),
        VenomPlan::OneOf(FType::Dizziness, FType::Squelched),
    ];
}

lazy_static! {
    pub static ref SOFT_STACK: Vec<FType> = vec![FType::Slickness, FType::Asthma, FType::Anorexia];
}

lazy_static! {
    static ref SOFT_BUFFER: Vec<FType> = vec![FType::Clumsiness, FType::Stupidity];
}

lazy_static! {
    static ref THIN_BUFFER_STACK: Vec<FType> = vec![FType::Allergies, FType::Vomiting];
}

lazy_static! {
    static ref LOCK_BUFFER_STACK: Vec<FType> =
        vec![FType::Paresis, FType::Stupidity, FType::Clumsiness];
}

lazy_static! {
    static ref STACKING_STRATEGIES: HashMap<String, Vec<VenomPlan>> = {
        let mut val = HashMap::new();
        val.insert("coag".into(), get_simple_plan(COAG_STACK.to_vec()));
        val.insert("dec".into(), get_simple_plan(DEC_STACK.to_vec()));
        val.insert("phys".into(), get_simple_plan(PHYS_STACK.to_vec()));
        val.insert("gank".into(), get_simple_plan(GANK_STACK.to_vec()));
        val.insert("fire".into(), get_simple_plan(FIRE_STACK.to_vec()));
        val.insert("kill".into(), get_simple_plan(KILL_STACK.to_vec()));
        val.insert("aggro".into(), AGGRO_STACK.to_vec());
        val.insert("salve".into(), get_simple_plan(SALVE_STACK.to_vec()));
        val.insert("peace".into(), get_simple_plan(PEACE_STACK.to_vec()));
        val.insert("slit".into(), SLIT_STACK.to_vec());
        val.insert("Monk".into(), get_simple_plan(MONK_STACK.to_vec()));
        val.insert("Luminary".into(), LUMINARY_STACK.to_vec());
        val.insert("Carnifex".into(), CARNIFEX_STACK.to_vec());
        val.insert("Sciomancer".into(), SCIOMANCER_STACK.to_vec());
        val.insert("Wayfarer".into(), WAYFARER_STACK.to_vec());
        val.insert("Praenomen".into(), PRAENOMEN_STACK.to_vec());
        val.insert("Syssin".into(), SYSSIN_STACK.to_vec());
        val.insert("Shaman".into(), SHAMAN_STACK.to_vec());
        val.insert("Templar".into(), get_simple_plan(PHYS_STACK.to_vec()));
        val.insert("Indorani".into(), INDORANI_STACK.to_vec());
        val.insert("Zealot".into(), ZEALOT_STACK.to_vec());
        val.insert("yedan".into(), get_simple_plan(YEDAN_STACK.to_vec()));
        val.insert("bedazzle".into(), BEDAZZLE_STACK.to_vec());
        val.insert("thin".into(), THIN_STACK.to_vec());
        val
    };
}

lazy_static! {
    static ref HARD_HYPNO: Vec<Hypnosis> = vec![
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Vertigo),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
    ];
}

lazy_static! {
    static ref ERADICATE_STACK: Vec<Hypnosis> = vec![
        Hypnosis::Eradicate,
        Hypnosis::Eradicate,
        Hypnosis::Eradicate,
        Hypnosis::Eradicate,
        Hypnosis::Eradicate,
        Hypnosis::Eradicate,
        Hypnosis::Eradicate,
        Hypnosis::Eradicate,
        Hypnosis::Trigger("lion".to_string()),
    ];
}

pub fn get_top_hypno<'s>(
    me: &String,
    target_name: &String,
    target: &AgentState,
    hypnos: &Vec<Hypnosis>,
) -> Option<Box<ActiveTransition>> {
    let mut hypno_idx = 0;
    for i in 0..target.hypno_state.hypnosis_stack.len() {
        if target.hypno_state.hypnosis_stack.get(i) == hypnos.get(hypno_idx) {
            hypno_idx += 1;
        }
    }
    if hypno_idx < hypnos.len() {
        if let Some(next_hypno) = hypnos.get(hypno_idx) {
            if !target.hypno_state.hypnotized {
                Some(Box::new(SeparatorAction::pair(
                    Box::new(HypnotiseAction::new(&me, &target_name)),
                    Box::new(SuggestAction::new(&me, &target_name, next_hypno.clone())),
                )))
            } else {
                Some(Box::new(SuggestAction::new(
                    &me,
                    &target_name,
                    next_hypno.clone(),
                )))
            }
        } else {
            panic!(
                "get_top_hypno: Len checked {} vs {}",
                hypno_idx,
                hypnos.len()
            )
        }
    } else if target.hypno_state.hypnotized {
        Some(Box::new(SealAction::new(&me, &target_name, 3)))
    } else {
        None
    }
}

fn should_bind(me: &AgentState, target: &AgentState, strategy: &String) -> bool {
    if !target.is(FType::Asleep)
        || target.is(FType::WritheBind)
        || target.lock_duration().is_none()
        || target.aff_count() < 9
    {
        false
    } else {
        true
    }
}

lazy_static! {
    static ref SLIT_BOOST_AFFS: Vec<FType> = vec![
        FType::Asleep,
        FType::Hypersomnia,
        FType::Narcolepsy,
        FType::WritheBind
    ];
}

fn should_slit(me: &AgentState, target: &AgentState, strategy: &String) -> bool {
    if !target.is_prone() {
        false
    } else if target.is(FType::Asleep) && target.affs_count(&SLIT_BOOST_AFFS) > 3 {
        target.is(FType::Rebounding) || target.aff_count() > 3
    } else if target.is(FType::Haemophilia)
        && target.affs_count(&vec![FType::Lethargy, FType::Allergies, FType::Vomiting]) >= 1
        && target.affs_count(&SLIT_BOOST_AFFS) > 1
    {
        true
    } else {
        false
    }
}

fn should_bedazzle(
    me: &AgentState,
    target: &AgentState,
    strategy: &String,
    before_flay: bool,
) -> bool {
    if !before_flay && me.is(FType::LeftArmCrippled) && !me.is(FType::RightArmCrippled) {
        true
    } else if before_flay && me.is(FType::RightArmCrippled) && !me.is(FType::LeftArmCrippled) {
        true
    } else if target.affs_count(&BEDAZZLE_AFFS.to_vec()) >= 5 {
        false
    } else if strategy.eq_ignore_ascii_case("bedazzle")
        && target.affs_count(&vec![FType::Vomiting, FType::Laxity, FType::Weariness]) < 2
        && !target.is(FType::ThinBlood)
        && !target.lock_duration().is_some()
    {
        true
    } else if strategy.eq_ignore_ascii_case("bedazzle")
        && (me.is(FType::Clumsiness) || target.is(FType::Rebounding))
        && target.affs_count(&vec![
            FType::Vomiting,
            FType::Laxity,
            FType::Weariness,
            FType::Dizziness,
        ]) < 3
        && !target.is(FType::ThinBlood)
        && !target.lock_duration().is_some()
    {
        true
    } else {
        false
    }
}

fn needs_fitness(timeline: &AetTimeline, me: &String) -> bool {
    let me = timeline.state.borrow_agent(me);
    me.balanced(BType::Fitness)
        && me.is(FType::Asthma)
        && me.is(FType::Anorexia)
        && me.is(FType::Slickness)
        && (!me.balanced(BType::Tree) || me.is(FType::Paresis) || me.is(FType::Paralysis))
        && (!me.balanced(BType::Focus) || me.is(FType::Impatience) || me.is(FType::Stupidity))
}

fn go_for_asp(_timeline: &AetTimeline, you: &AgentState, strategy: &String) -> bool {
    if strategy.eq("asp") && you.aff_count() > 3 {
        true
    } else {
        false
    }
}

fn go_for_thin_blood(_timeline: &AetTimeline, you: &AgentState, _strategy: &String) -> bool {
    let mut buffer_count = 0;
    if you.is(FType::Lethargy) {
        buffer_count = buffer_count + 1;
    }
    if you.is(FType::Vomiting) {
        buffer_count = buffer_count + 1;
    }
    if you.is(FType::Allergies) {
        buffer_count = buffer_count + 1;
    }
    (buffer_count >= 2
        || (buffer_count >= 1 && you.get_balance(BType::Pill) > 1.0)
        || you.lock_duration().map(|dur| dur > 5.0).unwrap_or(false))
        && !you.is(FType::ThinBlood)
        && (!you.is(FType::Fangbarrier)
            || !you.can_tree(true)
            || you.get_balance(BType::Tree) > 3.0)
}

pub fn should_lock(
    me: Option<&AgentState>,
    you: &AgentState,
    strategy: &String,
    lockers: &Vec<&str>,
    count: usize,
) -> bool {
    if let Some(me) = me {
        if !strategy.eq("aggro")
            && lockers.len() <= count
            && ((you.dodge_state.can_dodge_at(me.get_qeb_balance())
                && you.affs_count(&vec![
                    FType::Hypochondria,
                    FType::Clumsiness,
                    FType::Weariness,
                ]) < 1)
                || (you.is(FType::Hypersomnia) && !you.is(FType::Asleep)))
        {
            return false;
        }
    }
    (!you.can_focus(true) || you.is(FType::Stupidity) || you.get_balance(BType::Focus) > 2.5)
        && (!you.can_tree(true) || you.get_balance(BType::Tree) > 2.5)
        && lockers.len() < 3
        && lockers.len() > 0
}

pub fn get_flay_action(timeline: &AetTimeline, target: &String, def: String, v1: String) -> String {
    let action = if use_one_rag(timeline) && !v1.eq_ignore_ascii_case("") {
        format!("stand;;hw {};;flay {}", v1, target)
    } else {
        format!("stand;;envenom whip with {};;flay {}", v1, target)
    };
    let action = if should_call_venoms(timeline) && !v1.eq_ignore_ascii_case("") {
        format!("{};;{}", call_venom(target, &v1, None), action)
    } else {
        action
    };

    action
}

pub fn get_dstab_action(
    timeline: &AetTimeline,
    target: &String,
    v1: &String,
    v2: &String,
) -> String {
    let action = if use_one_rag(timeline) {
        format!("hr {};;hr {};;stand;;dstab {};;dash d", v2, v1, target)
    } else {
        format!("stand;;dstab {} {} {};;dash d", target, v1, v2)
    };
    if should_call_venoms(timeline) {
        format!("{};;{}", call_venoms(target, v1, v2, None), action)
    } else {
        action
    }
}

pub fn get_dstab_asp_action(timeline: &AetTimeline, target: &String, v1: &String) -> String {
    let action = if use_one_rag(timeline) {
        format!("wipe dirk;;hr {};;stand;;dstab {};;dash d", v1, target)
    } else {
        format!("wipe dirk;;stand;;dstab {} {};;dash d", target, v1)
    };
    if should_call_venoms(timeline) {
        format!("{};;{}", call_venom(target, v1, None), action)
    } else {
        action
    }
}

pub fn get_slit_action(timeline: &AetTimeline, target: &String, v1: &String) -> String {
    let action = if use_one_rag(timeline) {
        format!("stand;;hr {};;dstab {};;dash d", v1, target)
    } else {
        format!("stand;;slit {} {};;dash d", target, v1)
    };
    if should_call_venoms(timeline) {
        format!("{};;{}", call_venom(target, v1, None), action)
    } else {
        action
    }
}

pub fn get_bind_action(timeline: &AetTimeline, target: &String) -> String {
    format!("stand;;outc rope;;bind {};;inc rope", target)
}

pub fn add_delphs(
    timeline: &AetTimeline,
    me: &AgentState,
    you: &AgentState,
    strategy: &String,
    venoms: &mut Vec<&'static str>,
    count: usize,
) {
    if you.is(FType::Allergies) || you.is(FType::Vomiting) {
        return;
    }
    if you.is(FType::Hypersomnia) {
        match (
            you.is(FType::Insomnia),
            you.is(FType::Asleep),
            you.is(FType::Instawake),
        ) {
            (true, false, true) => {
                if get_cure_depth(you, FType::Hypersomnia).cures > 1 {
                    venoms.insert(0, "delphinium");
                }
            }
            (false, false, true) => {
                if count == 2 {
                    venoms.insert(0, "delphinium");
                    venoms.insert(0, "delphinium");
                }
            }
            (true, _, _) | (_, false, _) | (_, _, true) => {
                venoms.insert(0, "delphinium");
            }
            _ => {}
        }
        if venoms.len() >= count && Some(&"darkshade") == venoms.get(venoms.len() - count) {
            venoms.remove(venoms.len() - count);
        }
        if venoms.len() >= count && Some(&"euphorbia") == venoms.get(venoms.len() - count) {
            venoms.remove(venoms.len() - count);
        }
    }
}

pub fn get_stack<'s>(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Option<Vec<VenomPlan>> {
    if strategy.eq("class") {
        if let Some(class) = db.and_then(|db| db.get_class(target)) {
            let class_name = format!("{:?}", class.normal());
            if STACKING_STRATEGIES.contains_key(&class_name) {
                return STACKING_STRATEGIES.get(&class_name).cloned();
            } else if is_affected_by(&class, FType::Clumsiness) {
                return STACKING_STRATEGIES.get("phys").cloned();
            } else if is_affected_by(&class, FType::Peace) {
                return STACKING_STRATEGIES.get("peace").cloned();
            } else {
                return STACKING_STRATEGIES.get("aggro").cloned();
            }
        } else {
            return STACKING_STRATEGIES.get("aggro").cloned();
        }
    }
    db.and_then(|db| db.get_venom_plan(&format!("syssin_{}", strategy)))
        .or(STACKING_STRATEGIES.get(strategy).cloned())
}

pub fn choose_venoms(
    timeline: &AetTimeline,
    who_am_i: &String,
    target: &String,
    strategy: &String,
    venom_plan: &Vec<VenomPlan>,
    db: Option<&impl AetDatabaseModule>,
    count: usize,
) -> Vec<&'static str> {
    let me = timeline.state.borrow_agent(who_am_i);
    let you = timeline.state.borrow_agent(target);
    let mut venoms = get_venoms_from_plan(&venom_plan.to_vec(), 2, &you);
    let lockers = get_venoms(SOFT_STACK.to_vec(), 3, &you);
    let mut priority_buffer = false;
    if should_lock(Some(&me), &you, &strategy, &lockers, count) {
        add_buffers(&mut venoms, &lockers);
        priority_buffer = true;
    } else if lockers.len() == 0 {
        let buffer = get_venoms(LOCK_BUFFER_STACK.to_vec(), 2, &you);
        add_buffers(&mut venoms, &buffer);
        priority_buffer = buffer.len() > 0;
    }
    if !priority_buffer {
        if go_for_thin_blood(timeline, &you, strategy) {
            if you.is(FType::Fangbarrier) && !you.is(FType::Hypersomnia) {
                let mut buffer = get_venoms(THIN_BUFFER_STACK.to_vec(), 1, &you);
                add_buffers(&mut venoms, &buffer);
                return venoms;
            } else {
                return vec!["scytherus"];
            }
        } else if you.is(FType::Hypersomnia) {
            add_delphs(&timeline, &me, &you, &strategy, &mut venoms, count);
        }
        let mut buffer = get_venoms(THIN_BUFFER_STACK.to_vec(), 2, &you);
        if strategy.eq("thin") {
            buffer.clear();
        }
        if you.lock_duration().map_or(false, |dur| dur > 10.0) && !you.is(FType::Voyria) {
            buffer.insert(buffer.len(), "voyria");
        }
        if you.is(FType::ThinBlood) && buffer.len() > 0 {
            add_buffers(&mut venoms, &buffer);
        } else if !you.can_tree(false) {
            let mut hypno_buffers = vec![];
            let mut buffer_count = 1;
            if you.is(FType::Impatience)
                || you.hypno_state.get_next_hypno_aff() == Some(FType::Impatience)
            {
                if you.is(FType::Impatience) {
                    hypno_buffers.push(FType::Stupidity);
                    match check_config_int(timeline, &"SYSSIN_IMPATIENCE_DEPTH".to_string()) {
                        3 => {
                            hypno_buffers.push(FType::Shyness);
                            hypno_buffers.push(FType::Dizziness);
                            buffer_count = 2;
                        }
                        2 => {
                            hypno_buffers.push(FType::Dizziness);
                            buffer_count = 2;
                        }
                        _ => {}
                    }
                } else {
                    hypno_buffers.push(FType::Shyness);
                }
            }
            if you.is(FType::Impatience)
                && (you.is(FType::Loneliness)
                    || you.hypno_state.get_next_hypno_aff() == Some(FType::Loneliness))
            {
                hypno_buffers.push(FType::Recklessness);
            } else if you.is(FType::Impatience)
                && (you.is(FType::Vertigo)
                    || you.hypno_state.get_next_hypno_aff() == Some(FType::Vertigo))
            {
                hypno_buffers.push(FType::Recklessness);
            }
            if you.is(FType::Generosity)
                || you.hypno_state.get_next_hypno_aff() == Some(FType::Generosity)
            {
                hypno_buffers.push(FType::Peace);
                if !you.is(FType::Impatience) {
                    hypno_buffers.push(FType::Stupidity);
                }
            }
            let hypno_buffers = get_venoms(hypno_buffers, buffer_count, &you);
            add_buffers(&mut venoms, &hypno_buffers);
        }
    }
    venoms
}

pub fn get_balance_attack<'s>(
    timeline: &AetTimeline,
    who_am_i: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Box<dyn ActiveTransition> {
    if let Some(stack) = get_stack(timeline, target, strategy, db) {
        let me = timeline.state.borrow_agent(who_am_i);
        let you = timeline.state.borrow_agent(target);
        let mut two_venoms = choose_venoms(&timeline, who_am_i, target, strategy, &stack, db, 2);
        let v2 = two_venoms.get(1);
        let v1 = two_venoms.get(0);
        let v_one = choose_venoms(&timeline, who_am_i, target, strategy, &stack, db, 1).pop();
        if needs_fitness(&timeline, who_am_i) {
            return Box::new(FitnessAction::new(who_am_i.to_string()));
        } else if needs_restore(&timeline, who_am_i) {
            return Box::new(RestoreAction::new(who_am_i.to_string()));
        } else if let Ok(true) = get_equil_attack(timeline, who_am_i, target, strategy, db)
            .act(&timeline)
            .map(|act| act.starts_with("seal"))
        {
            return Box::new(Inactivity);
        } else if (you.is(FType::Shielded)
            || you.is(FType::Rebounding)
            || you.will_be_rebounding(me.get_qeb_balance()))
            && !should_slit(&me, &you, &strategy)
        {
            if !you.is(FType::Shielded) && should_bedazzle(&me, &you, &strategy, true) {
                return Box::new(BedazzleAction::new(who_am_i, &target));
            }
            let defense = if you.is(FType::Shielded) {
                "shield"
            } else {
                "rebounding"
            };
            if let Some(venom) = v_one {
                return Box::new(FlayAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    defense.to_string(),
                    venom.to_string(),
                ));
            } else {
                return Box::new(FlayAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    defense.to_string(),
                    "".to_string(),
                ));
            }
        } else {
            if should_bedazzle(&me, &you, &strategy, false) {
                return Box::new(BedazzleAction::new(who_am_i, &target));
            } else if should_slit(&me, &you, &strategy)
                && v_one.is_some()
                && v_one != Some("scytherus")
            {
                return Box::new(SlitAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    v_one.unwrap().to_string(),
                ));
            } else if should_bind(&me, &you, &strategy) {
                return Box::new(BindAction::new(who_am_i.to_string(), target.to_string()));
            } else if go_for_thin_blood(timeline, &you, strategy) && you.is(FType::Fangbarrier) {
                return Box::new(FlayAction::fangbarrier(
                    who_am_i.to_string(),
                    target.to_string(),
                    v_one.map(|venom| venom.to_string()).unwrap_or_default(),
                ));
            } else if v_one
                .map(|venom| venom.eq_ignore_ascii_case("scytherus"))
                .unwrap_or(false)
            {
                return Box::new(BiteAction::new(who_am_i, &target, &"scytherus"));
            } else if go_for_asp(timeline, &you, strategy) {
                return Box::new(DoublestabAction::new_asp(
                    who_am_i.to_string(),
                    target.to_string(),
                    v_one.unwrap_or(&"").to_string(),
                ));
            } else if let (Some(v1), Some(v2)) = (v1, v2) {
                return Box::new(DoublestabAction::new(
                    who_am_i.to_string(),
                    target.to_string(),
                    v2.to_string(),
                    v1.to_string(),
                ));
            } else if you.is(FType::Fangbarrier) {
                return Box::new(FlayAction::fangbarrier(
                    who_am_i.to_string(),
                    target.to_string(),
                    v_one.map(|venom| venom.to_string()).unwrap_or_default(),
                ));
            } else {
                return Box::new(BiteAction::new(who_am_i, &target, &"camus"));
            }
        }
    } else if strategy == "damage" {
        let you = timeline.state.borrow_agent(target);
        if you.is(FType::Fangbarrier) {
            return Box::new(FlayAction::fangbarrier(
                who_am_i.to_string(),
                target.to_string(),
                get_venoms_from_plan(&AGGRO_STACK.to_vec(), 1, &you)
                    .get(0)
                    .map(|venom| venom.to_string())
                    .unwrap_or_default(),
            ));
        } else {
            return Box::new(BiteAction::new(who_am_i, &target, &"camus"));
        }
    } else if strategy == "group" {
        let you = timeline.state.borrow_agent(target);
        if you.is_prone() {
            return Box::new(GarroteAction::new(who_am_i, target));
        } else {
            return get_balance_attack(timeline, who_am_i, target, &"salve".to_string(), db);
        }
    } else if strategy == "shield" {
        let me = timeline.state.borrow_me();
        if me.can_touch() && !me.is(FType::Shielded) {
            return Box::new(ShieldAction::new(who_am_i));
        } else if needs_fitness(timeline, who_am_i) {
            return Box::new(FitnessAction::new(who_am_i.to_string()));
        } else {
            return Box::new(Action::new(
                "firstaid elevate paresis;;firstaid elevate frozen;;firstaid elevate paralysis"
                    .to_string(),
            ));
        }
    } else if let Some(captures) = ERADICATE_PLAN.captures(strategy) {
        if let Some(names) = captures.get(1) {
            for name in names.as_str().split(",") {
                let you = timeline.state.borrow_agent(&name.to_string());
                if let Some(hypno) =
                    get_top_hypno(who_am_i, &name.to_string(), &you, &ERADICATE_STACK)
                {
                    return Box::new(SeparatorAction::pair(
                        Box::new(Trace::new(format!(
                            "{}: {}",
                            name,
                            you.hypno_state.suggestion_count()
                        ))),
                        hypno,
                    ));
                }
            }
            return Box::new(Inactivity);
        } else {
            println!("No names found for eradicate: {}", strategy);
            return Box::new(Inactivity);
        }
    } else {
        return Box::new(Inactivity);
    }
}

pub fn get_hypno_stack_name(timeline: &AetTimeline, target: &String, strategy: &String) -> String {
    timeline
        .state
        .get_my_hint(&"HYPNO_STACK".to_string())
        .unwrap_or(strategy.to_string())
}

pub fn get_hypno_stack<'s>(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Vec<Hypnosis> {
    db.and_then(|db| {
        let stack = get_hypno_stack_name(timeline, target, strategy);
        if stack == "normal" {
            None // Default to HARD_HYPNO
        } else if stack == "class" {
            if let Some(class) = db.get_class(target) {
                db.get_hypno_plan(&class.to_string())
            } else {
                db.get_hypno_plan(&format!("hypno_{}", stack))
            }
        } else {
            db.get_hypno_plan(&format!("hypno_{}", stack))
        }
    })
    .unwrap_or(HARD_HYPNO.to_vec())
}

pub fn get_equil_attack<'s>(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Box<dyn ActiveTransition> {
    if strategy.eq("damage")
        || strategy.eq("shield")
        || strategy.eq("runaway")
        || ERADICATE_PLAN.is_match(strategy)
    {
        return Box::new(Inactivity);
    }
    let you = timeline.state.borrow_agent(target);
    let stack = get_hypno_stack(timeline, target, strategy, db);
    let hypno_action = get_top_hypno(me, target, &you, &stack);
    hypno_action.unwrap_or(Box::new(Inactivity))
}

pub fn get_shadow_attack<'s>(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
) -> Box<dyn ActiveTransition> {
    if strategy == "pre"
        || strategy == "shield"
        || strategy == "runaway"
        || ERADICATE_PLAN.is_match(strategy)
    {
        Box::new(Inactivity)
    } else {
        let you = timeline.state.borrow_agent(target);
        if !should_void(timeline)
            || you.is(FType::Void)
            || you.is(FType::Weakvoid)
            || you.hypno_state.active
        {
            if you.lock_duration().is_some() {
                Box::new(SleightAction::new(me, &target, &"blank"))
            } else if strategy == "salve" {
                Box::new(SleightAction::new(me, &target, &"abrasion"))
            } else {
                Box::new(SleightAction::new(me, &target, &"dissipate"))
            }
        } else {
            Box::new(SleightAction::new(me, &target, &"void"))
        }
    }
}

pub fn get_snap(timeline: &AetTimeline, me: &String, target: &String, _strategy: &String) -> bool {
    let you = timeline.state.borrow_agent(target);
    if get_top_hypno(me, target, &you, &HARD_HYPNO.to_vec()).is_none()
        && you.hypno_state.sealed.is_some()
    {
        return true;
    } else {
        return false;
    }
}

pub fn get_action_plan(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> ActionPlan {
    let mut action_plan = ActionPlan::new(me);
    let mut balance = get_balance_attack(timeline, me, target, strategy, db);
    if should_regenerate(&timeline, me) {
        balance = Box::new(RegenerateAction::new(me.to_string()));
    }
    if let Some(parry) = get_needed_parry(timeline, me, target, strategy, db) {
        balance = Box::new(SeparatorAction::pair(
            Box::new(ParryAction::new(me.to_string(), parry)),
            balance,
        ));
    }
    let equil = get_equil_attack(timeline, me, target, strategy, db);
    let shadow = get_shadow_attack(timeline, me, target, strategy);
    if let Ok(_activation) = balance.act(&timeline) {
        action_plan.add_to_qeb(balance);
    }
    if let Ok(_activation) = equil.act(&timeline) {
        action_plan.add_to_qeb(equil);
    }
    if let Ok(activation) = shadow.act(&timeline) {
        if activation.starts_with("shadow sleight void") {
            action_plan.queue_for(BType::Secondary, shadow);
        } else {
            action_plan.add_to_qeb(shadow);
        }
    }
    let me = timeline.state.borrow_agent(me);
    for pipe_refill in get_needed_refills(&me) {
        action_plan.add_to_front_of_qeb(Box::new(pipe_refill));
    }
    action_plan
}

struct SyssinActionPlanner;
const STRATEGIES: [&'static str; 3] = ["phys", "bedazzle", "aggro"];

impl ActionPlanner for SyssinActionPlanner {
    fn get_strategies(&self) -> &'static [&'static str] {
        &STRATEGIES
    }
    fn get_plan(
        &self,
        timeline: &AetTimeline,
        actor: &String,
        target: &String,
        strategy: &str,
        db: Option<&impl AetDatabaseModule>,
    ) -> ActionPlan {
        get_action_plan(timeline, actor, target, &strategy.to_string(), db)
    }
}

pub fn get_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let action_plan = get_action_plan(&timeline, &timeline.who_am_i(), &target, &strategy, db);
    action_plan.get_inputs(&timeline)
}
