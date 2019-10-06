use crate::types::*;

pub fn healChange(amount: CType) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Owner,
        agent_type: AgentTypeId::Stat(SType::Health),
        change_type: ChangeType::Add,
        change: amount,
    }
}

pub fn attackChange(amount: CType) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Target,
        agent_type: AgentTypeId::Stat(SType::Health),
        change_type: ChangeType::Add,
        change: -amount,
    }
}

pub fn balanceChange(balance: BType, duration: f32) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Owner,
        agent_type: AgentTypeId::Balance(balance),
        change_type: ChangeType::Add,
        change: (duration * BALANCE_SCALE) as i32,
    }
}

pub fn flagMe(flag: FType, value: bool) -> StateChange {
    StateChange {
        agent_relation: AgentRelationId::Owner,
        agent_type: AgentTypeId::Flag(flag),
        change_type: ChangeType::Set,
        change: if value { 1 } else { 0 },
    }
}

pub fn attackAction(damage: CType, balance: BType, duration: f32) -> StableAction {
    StableAction {
        changes: vec![
            attackChange(damage),
            balanceChange(balance, duration),
            flagMe(FType::Shield, false),
        ],
        initial: vec![ALIVE, has(balance)],
    }
}

pub fn healAction(heal: CType) -> StableAction {
    StableAction {
        changes: vec![healChange(heal), balanceChange(BType::Elixir, 4.0)],
        initial: vec![ALIVE, has(BType::Elixir)],
    }
}

pub fn shieldAction() -> StableAction {
    StableAction {
        changes: vec![
            balanceChange(BType::Balance, 3.0),
            flagMe(FType::Shield, true),
        ],
        initial: vec![ALIVE, has(BType::Balance), has(BType::Equil)],
    }
}
