use num_enum::TryFromPrimitive;
pub type CType = i32;

pub const BALANCE_SCALE: f32 = 100.0;

// Balances
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
pub enum BType {
    Balance,
    Equil,
    Elixir,

    SIZE,
}

// Stats
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum SType {
    Health,
    Mana,

    SIZE,
}

// Flags
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u16)]
pub enum FType {
    Dead,
    Shield,

    SIZE,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AgentTypeId {
    Balance(BType),
    Stat(SType),
    Flag(FType),
}

#[derive(Debug)]
pub enum ChangeType {
    Add,
    Set,
}

#[derive(PartialEq, Debug)]
pub enum AgentRelationId {
    Owner,
    Target,
}

#[derive(Debug)]
pub struct StateChange {
    pub agent_relation: AgentRelationId,
    pub agent_type: AgentTypeId,
    pub change_type: ChangeType,
    pub change: CType,
}

#[derive(Debug)]
pub struct StateMatcher {
    pub agent_relation: AgentRelationId,
    pub agent_type: AgentTypeId,
    pub value: CType,
    pub inverted: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: [bool; FType::SIZE as usize],
}

impl AgentState {
    pub fn wait(&self, duration: i32) -> AgentState {
        let mut base_state = self.clone();
        for i in 0..base_state.balances.len() {
            base_state.balances[i] -= duration;
        }
        base_state
    }
}

impl StateChange {
    pub fn apply(&self, owner: &AgentState, other: &AgentState) -> (AgentState, AgentState) {
        let mut base_state = {
            if self.agent_relation == AgentRelationId::Owner {
                owner.clone()
            } else {
                other.clone()
            }
        };
        match self.agent_type {
            AgentTypeId::Balance(bal) => {
                base_state.balances[bal as usize] = match self.change_type {
                    ChangeType::Add => base_state.balances[bal as usize] + self.change,
                    ChangeType::Set => self.change,
                };
            }
            AgentTypeId::Flag(flag) => base_state.flags[flag as usize] = self.change >= 1,
            AgentTypeId::Stat(stat) => {
                base_state.stats[stat as usize] = match self.change_type {
                    ChangeType::Add => base_state.stats[stat as usize] + self.change,
                    ChangeType::Set => self.change,
                };
                if base_state.stats[stat as usize] > base_state.max_stats[stat as usize] {
                    base_state.stats[stat as usize] = base_state.max_stats[stat as usize];
                }
            }
        };
        if self.agent_relation == AgentRelationId::Owner {
            (base_state, other.clone())
        } else {
            (owner.clone(), base_state)
        }
    }
}

pub const ALIVE: StateMatcher = StateMatcher {
    agent_relation: AgentRelationId::Owner,
    agent_type: AgentTypeId::Flag(FType::Dead),
    value: 0,
    inverted: false,
};

pub fn has(balance: BType) -> StateMatcher {
    StateMatcher {
        agent_relation: AgentRelationId::Owner,
        agent_type: AgentTypeId::Balance(balance),
        value: 1,
        inverted: false,
    }
}

impl StateMatcher {
    pub fn check(&self, owner: &AgentState, other: &AgentState) -> bool {
        let state = {
            if self.agent_relation == AgentRelationId::Owner {
                &owner
            } else {
                &other
            }
        };
        match self.agent_type {
            AgentTypeId::Balance(bal) => {
                if self.value == 1 {
                    state.balances[bal as usize] <= 0
                } else {
                    state.balances[bal as usize] >= 0
                }
            }
            AgentTypeId::Stat(stat) => {
                if self.inverted {
                    state.stats[stat as usize] <= self.value
                } else {
                    state.stats[stat as usize] >= self.value
                }
            }
            AgentTypeId::Flag(flag) => {
                if self.value == 1 {
                    state.flags[flag as usize]
                } else {
                    !state.flags[flag as usize]
                }
            }
        }
    }
}
