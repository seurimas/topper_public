use crate::agents::*;
use crate::types::*;

pub fn apply_me<F>(change: F) -> StateChange
where
    F: 'static + Fn(&mut AgentState),
{
    Box::new(move |me, _other| {
        change(me);
    })
}

pub fn apply_you<F>(change: F) -> StateChange
where
    F: 'static + Fn(&mut AgentState),
{
    Box::new(move |_me, other| {
        change(other);
    })
}

pub fn heal_change(amount: CType) -> StateChange {
    apply_me(move |new_me| {
        new_me.stats[SType::Health as usize] += amount;
    })
}

pub fn attack_change(amount: CType) -> StateChange {
    apply_you(move |new_you| {
        new_you.stats[SType::Health as usize] -= amount;
    })
}

pub fn balance_change(balance: BType, duration: f32) -> StateChange {
    apply_me(move |new_me| {
        if new_me.balances[balance as usize] < 0 {
            new_me.balances[balance as usize] = 0;
        }
        new_me.balances[balance as usize] += (duration * BALANCE_SCALE) as CType;
    })
}

pub fn flag_me(flag: FType, value: bool) -> StateChange {
    apply_me(move |new_me| {
        new_me.flags[flag as usize] = value;
    })
}

pub fn attack_action(name: String, damage: CType, balance: BType, duration: f32) -> Action {
    Action {
        name,
        changes: vec![
            attack_change(damage),
            balance_change(balance, duration),
            flag_me(FType::Shield, false),
        ],
        initial: vec![alive(), has(balance)],
    }
}

pub fn heal_action(name: String, heal: CType) -> Action {
    Action {
        name,
        changes: vec![heal_change(heal), balance_change(BType::Elixir, 4.0)],
        initial: vec![alive(), has(BType::Elixir)],
    }
}

pub fn shield_action(name: String) -> Action {
    Action {
        name,
        changes: vec![
            balance_change(BType::Balance, 3.0),
            flag_me(FType::Shield, true),
        ],
        initial: vec![alive(), has(BType::Balance), has(BType::Equil)],
    }
}
