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

pub fn afflict(flag: FType) -> StateChange {
    apply_you(move |new_you| {
        let original = new_you.is(flag);
        new_you.set_flag(flag, true);
        Box::new(move |you| you.set_flag(flag, original))
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
