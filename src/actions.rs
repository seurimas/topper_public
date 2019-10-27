use crate::types::*;

pub struct StateAction {
    pub name: String,
    pub changes: Vec<StateChange>,
    pub initial: Vec<StateMatcher>,
}

pub trait StateConditions {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool;
}

impl StateConditions for Vec<StateMatcher> {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool {
        self.iter()
            .find(|&matcher| !matcher(owner, other))
            .is_none()
    }
}

pub fn join(reversions: Vec<StateRevert>) -> StateRevert {
    Box::new(move |mut me, mut you| {
        for reversion in reversions.iter().rev() {
            reversion(&mut me, &mut you);
        }
    })
}

impl StateAction {
    pub fn apply(&self, owner: &mut AgentState, other: &mut AgentState) -> StateRevert {
        join(
            self.changes
                .iter()
                .map(|change| change(owner, other))
                .collect(),
        )
    }
}

impl StateConditions for StateAction {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool {
        self.initial.satisfied(owner, other)
    }
}

pub struct UnstableAction {
    pub desc: String,
    pub paths: Vec<(i32, StateAction)>,
    pub initial: Vec<StateMatcher>,
}

impl StateAction {
    pub fn always(self) -> UnstableAction {
        UnstableAction {
            desc: self.name.clone(),
            paths: vec![(1, self)],
            initial: vec![],
        }
    }
}

pub fn apply_me<F>(change: F) -> StateChange
where
    F: 'static + Fn(&mut AgentState) -> Box<Fn(&mut AgentState)>,
{
    Box::new(move |me, _other| {
        let revert = change(me);
        Box::new(move |me2, _you2| revert(me2))
    })
}

pub fn apply_you<F>(change: F) -> StateChange
where
    F: 'static + Fn(&mut AgentState) -> Box<Fn(&mut AgentState)>,
{
    Box::new(move |_me, other| {
        let revert = change(other);
        Box::new(move |_me2, you2| revert(you2))
    })
}

pub fn heal_change(amount: CType) -> StateChange {
    apply_me(move |new_me| {
        let original = new_me.stats[SType::Health as usize];
        new_me.stats[SType::Health as usize] += amount;
        if new_me.stats[SType::Health as usize] > new_me.max_stats[SType::Health as usize] {
            new_me.stats[SType::Health as usize] = new_me.max_stats[SType::Health as usize];
        }
        Box::new(move |me| me.stats[SType::Health as usize] = original)
    })
}

pub fn check_health(state: &mut AgentState) {
    if state.stats[SType::Health as usize] <= 0 {
        state.set_flag(FType::Dead, true);
    }
}

pub fn attack_change(amount: CType) -> StateChange {
    apply_you(move |new_you| {
        let original = new_you.stats[SType::Health as usize];
        let dead = new_you.is(FType::Dead);
        new_you.stats[SType::Health as usize] -= amount;
        check_health(new_you);
        Box::new(move |me| {
            me.stats[SType::Health as usize] = original;
            me.set_flag(FType::Dead, dead);
        })
    })
}

pub fn tick(accumulator: SType) -> StateChange {
    apply_me(move |new_me| {
        new_me.stats[accumulator as usize] += 1;
        Box::new(move |me| me.stats[accumulator as usize] -= 1)
    })
}

pub fn balance_change(balance: BType, duration: f32) -> StateChange {
    apply_me(move |new_me| {
        let original = new_me.balances[balance as usize];
        if new_me.balances[balance as usize] < 0 {
            new_me.balances[balance as usize] = 0;
        }
        new_me.balances[balance as usize] += (duration * BALANCE_SCALE) as CType;
        Box::new(move |me| {
            me.balances[balance as usize] = original;
        })
    })
}

pub fn flag_me(flag: FType, value: bool) -> StateChange {
    apply_me(move |new_me| {
        let original = new_me.is(flag);
        new_me.set_flag(flag, value);
        Box::new(move |me| me.set_flag(flag, original))
    })
}

pub fn attack_action(name: String, damage: CType, balance: BType, duration: f32) -> StateAction {
    StateAction {
        name,
        changes: vec![
            attack_change(damage),
            balance_change(balance, duration),
            flag_me(FType::Shield, false),
        ],
        initial: vec![
            alive(),
            target(alive()),
            target(lacks(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
        ],
    }
}

pub fn wiff_action(name: String, balance: BType, duration: f32) -> StateAction {
    StateAction {
        name,
        changes: vec![balance_change(balance, duration)],
        initial: vec![
            alive(),
            target(alive()),
            target(is(FType::Shield)),
            has(BType::Balance),
            has(BType::Equil),
        ],
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
    Box::new(|me| {})
}

fn revert_flag(flag: FType) -> Box<Fn(&mut AgentState)> {
    Box::new(move |me2: &mut AgentState| me2.set_flag(flag, true))
}

pub fn cure_in_order(afflictions: Vec<FType>) -> StateChange {
    apply_me(move |me| {
        let mut revert = noop();
        for affliction in afflictions.iter() {
            if me.is(*affliction) {
                revert = revert_flag(*affliction);
                me.set_flag(*affliction, false);
            }
        }
        revert
    })
}

pub fn herb_action(name: String, afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("eat {}", name),
        changes: vec![
            cure_in_order(afflictions.clone()),
            balance_change(BType::Pill, 3.0),
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

pub fn antipsychotic() -> StateAction {
    herb_action(
        "antipsychotic".into(),
        vec![
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
        ],
    )
}

pub fn euphoriant() -> StateAction {
    herb_action(
        "euphoriant".into(),
        vec![
            FType::SelfPity,
            FType::Stupidity,
            FType::Dizziness,
            FType::Faintness,
            FType::Shyness,
            FType::Epilepsy,
            FType::Impatience,
            FType::Dissonance,
            FType::Infested,
        ],
    )
}

pub fn decongestant() -> StateAction {
    herb_action(
        "decongestant".into(),
        vec![
            FType::Baldness,
            FType::Clumsiness,
            FType::Hypochondria,
            FType::Weariness,
            FType::Asthma,
            FType::Sensitivity,
            FType::RingingEars,
            FType::Impairment,
            FType::BloodPoison,
        ],
    )
}

pub fn depressant() -> StateAction {
    herb_action(
        "depressant".into(),
        vec![
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
        ],
    )
}

pub fn coagulation() -> StateAction {
    herb_action(
        "coagulation".into(),
        vec![
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
        ],
    )
}

pub fn steroid() -> StateAction {
    herb_action(
        "steroid".into(),
        vec![
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
        ],
    )
}

pub fn opiate() -> StateAction {
    herb_action(
        "opiate".into(),
        vec![
            FType::Paralysis,
            FType::Mirroring,
            FType::CrippledBody,
            FType::Crippled,
            FType::Blisters,
            FType::Slickness,
            FType::Heartflutter,
            FType::Sandrot,
        ],
    )
}

pub fn salve_action(name: String, location: String, afflictions: Vec<FType>) -> StateAction {
    StateAction {
        name: format!("apply {} to {}", name, location),
        changes: vec![
            cure_in_order(afflictions.clone()),
            balance_change(BType::Salve, 3.0),
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

pub fn epidermal_head() -> StateAction {
    salve_action(
        "epidermal".into(),
        "head".into(),
        vec![
            FType::Indifference,
            FType::Stuttering,
            FType::BlurryVision,
            FType::BurntEyes,
            FType::Gloom,
        ],
    )
}

pub fn epidermal_torso() -> StateAction {
    salve_action(
        "epidermal".into(),
        "torso".into(),
        vec![
            FType::Anorexia,
            FType::Gorged,
            FType::EffusedBlood,
            FType::Hypothermia,
        ],
    )
}

pub fn mending_head() -> StateAction {
    salve_action(
        "mending".into(),
        "head".into(),
        vec![
            FType::CritBruiseHead,
            FType::DestroyedThroat,
            FType::CrippledThroat,
            FType::ModBruiseHead,
            FType::BruiseHead,
        ],
    )
}

pub fn mending_torso() -> StateAction {
    salve_action(
        "mending".into(),
        "torso".into(),
        vec![
            FType::CritBruiseTorso,
            FType::LightWound,
            FType::Ablaze,
            FType::CrackedRibs,
            FType::ModBruiseTorso,
            FType::BruiseTorso,
        ],
    )
}

pub fn mending_left_arm() -> StateAction {
    salve_action(
        "mending".into(),
        "left arm".into(),
        vec![
            FType::CritBruiseLeftArm,
            FType::BrokenLeftArm,
            FType::ModBruiseLeftArm,
            FType::BruiseLeftArm,
            FType::DislocatedLeftArm,
        ],
    )
}

pub fn mending_right_arm() -> StateAction {
    salve_action(
        "mending".into(),
        "right arm".into(),
        vec![
            FType::CritBruiseRightArm,
            FType::BrokenRightArm,
            FType::ModBruiseRightArm,
            FType::BruiseRightArm,
            FType::DislocatedRightArm,
        ],
    )
}

pub fn mending_left_leg() -> StateAction {
    salve_action(
        "mending".into(),
        "left leg".into(),
        vec![
            FType::CritBruiseLeftLeg,
            FType::BrokenLeftLeg,
            FType::ModBruiseLeftLeg,
            FType::BruiseLeftLeg,
            FType::DislocatedLeftLeg,
        ],
    )
}

pub fn mending_right_leg() -> StateAction {
    salve_action(
        "mending".into(),
        "right leg".into(),
        vec![
            FType::CritBruiseRightLeg,
            FType::BrokenRightLeg,
            FType::ModBruiseRightLeg,
            FType::BruiseRightLeg,
            FType::DislocatedRightLeg,
        ],
    )
}

pub fn soothing_head() -> StateAction {
    salve_action("soothing".into(), "head".into(), vec![FType::Whiplash])
}

pub fn soothing_torso() -> StateAction {
    salve_action(
        "soothing".into(),
        "torso".into(),
        vec![FType::Backstrain, FType::MuscleSpasms, FType::Stiffness],
    )
}

pub fn soothing_arms() -> StateAction {
    salve_action(
        "soothing".into(),
        "arms".into(),
        vec![FType::SoreWrist, FType::WeakGrip],
    )
}

pub fn soothing_legs() -> StateAction {
    salve_action("soothing".into(), "legs".into(), vec![FType::Whiplash])
}
