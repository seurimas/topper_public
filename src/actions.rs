use crate::agents::*;
use crate::types::*;

pub fn heal_change(amount: CType) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Owner,
        agent_type: AgentTypeId::Stat(SType::Health),
        change_type: ChangeType::Add,
        change: amount,
    }
}

pub fn attack_change(amount: CType) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Target,
        agent_type: AgentTypeId::Stat(SType::Health),
        change_type: ChangeType::Add,
        change: -amount,
    }
}

pub fn balance_change(balance: BType, duration: f32) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Owner,
        agent_type: AgentTypeId::Balance(balance),
        change_type: ChangeType::Add,
        change: (duration * BALANCE_SCALE) as i32,
    }
}

pub fn flag_me(flag: FType, value: bool) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Owner,
        agent_type: AgentTypeId::Flag(flag),
        change_type: ChangeType::Set,
        change: if value { 1 } else { 0 },
    }
}

pub fn attack_action(name: &'static str, damage: CType, balance: BType, duration: f32) -> Action {
    Action {
        name,
        changes: vec![
            attack_change(damage),
            balance_change(balance, duration),
            flag_me(FType::Shield, false),
        ],
        initial: vec![ALIVE, has(balance)],
    }
}

pub fn heal_action(name: &'static str, heal: CType) -> Action {
    Action {
        name,
        changes: vec![heal_change(heal), balance_change(BType::Elixir, 4.0)],
        initial: vec![ALIVE, has(BType::Elixir)],
    }
}

pub fn shield_action(name: &'static str) -> Action {
    Action {
        name,
        changes: vec![
            balance_change(BType::Balance, 3.0),
            flag_me(FType::Shield, true),
        ],
        initial: vec![ALIVE, has(BType::Balance), has(BType::Equil)],
    }
}
