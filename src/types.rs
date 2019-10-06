type BType = i8;
type SType = i8;
type FType = i16;
type CType = i32;

pub const BALANCE_SCALE: f32 = 100.0;

// Balanes
pub const BALANCE: BType = 0;
pub const EQUIL: BType = 1;
pub const ELIXIR: BType = 2;

// Stats
pub const HEALTH: SType = 0;
pub const MANA: SType = 1;
pub const WILLPOWER: SType = 2;
pub const ENDURANCE: SType = 3;

// Flags
pub const DEAD: FType = 0;
pub const SHIELD: FType = 1;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AgentTypeId {
    Balance(i8),
    Stat(i8),
    Flag(i16),
}

#[derive(Debug)]
pub enum ChangeType {
    Add,
    Subtract,
    Set,
}

#[derive(PartialEq, Debug)]
pub enum AgentRelationId {
    Owner,
    Target,
    None,
}

#[derive(Debug)]
pub struct StateChange {
    pub agentRelation: AgentRelationId,
    pub agentType: AgentTypeId,
    pub changeType: ChangeType,
    pub change: CType,
}

#[derive(Debug)]
pub struct StateMatcher {
    pub agentRelation: AgentRelationId,
    pub agentType: AgentTypeId,
    pub value: CType,
    pub inverted: bool,
}

pub const ALIVE: StateMatcher = StateMatcher {
    agentRelation: AgentRelationId::Owner,
    agentType: AgentTypeId::Flag(DEAD),
    value: 1,
    inverted: false,
};

pub fn has(balance: BType) -> StateMatcher {
    StateMatcher {
        agentRelation: AgentRelationId::Owner,
        agentType: AgentTypeId::Balance(balance),
        value: 1,
        inverted: false,
    }
}

impl StateMatcher {
    pub fn check(&self, agent: &MainAgent) -> bool {
        let value = agent.components.get(&self.agentType);
        if let Some(&value) = value {
            match self.agentType {
                AgentTypeId::Balance(_) => {
                    if self.value == 1 {
                        value >= 1
                    } else {
                        value <= 0
                    }
                }
                AgentTypeId::Stat(_) => {
                    if self.inverted {
                        value <= self.value
                    } else {
                        value >= self.value
                    }
                }
                AgentTypeId::Flag(_) => {
                    if self.value == 1 {
                        value >= 1
                    } else {
                        value <= 0
                    }
                }
            }
        } else {
            false
        }
    }
}

pub struct StableAction {
    pub changes: Vec<StateChange>,
    pub initial: Vec<StateMatcher>,
}

pub struct UnstableAction {
    pub paths: Vec<(i32, StableAction)>,
}
