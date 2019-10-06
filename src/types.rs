pub type CType = i32;

pub const BALANCE_SCALE: f32 = 100.0;

// Balanes
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BType {
    Balance,
    Equil,
    Elixir,

    SIZE,
}

// Stats
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SType {
    Health,
    Mana,

    SIZE,
}

// Flags
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug)]
pub struct StateMatcher {
    pub agent_relation: AgentRelationId,
    pub agent_type: AgentTypeId,
    pub value: CType,
    pub inverted: bool,
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

#[derive(Debug)]
pub struct StableAction {
    pub changes: Vec<StateChange>,
    pub initial: Vec<StateMatcher>,
}

#[derive(Debug)]
pub struct UnstableAction {
    pub paths: Vec<(i32, StableAction)>,
    pub initial: Vec<StateMatcher>,
}

trait StateConditions {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool;
}

impl StateConditions for Vec<StateMatcher> {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool {
        self.iter()
            .find(|&matcher| !matcher.check(owner, other))
            .is_none()
    }
}

trait StateUpdate {
    fn apply(&self, owner: &AgentState, other: &AgentState) -> (AgentState, AgentState);
}

impl StateUpdate for StableAction {
    fn apply(&self, owner: &AgentState, other: &AgentState) -> (AgentState, AgentState) {
        let mut me = owner.clone();
        let mut you = other.clone();
        for change in self.changes.iter() {
            let (new_me, new_you) = change.apply(&me, &you);
            me = new_me;
            you = new_you;
        }
        (me, you)
    }
}

impl StableAction {
    pub fn always(self) -> UnstableAction {
        UnstableAction {
            paths: vec![(1, self)],
            initial: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AgentState {
    pub balances: [CType; BType::SIZE as usize],
    pub stats: [CType; SType::SIZE as usize],
    pub max_stats: [CType; SType::SIZE as usize],
    pub flags: [bool; FType::SIZE as usize],
}

#[derive(Debug)]
pub struct MainAgent {
    pub actions: Vec<UnstableAction>,
    pub initial_state: AgentState,
}

impl MainAgent {
    pub fn new(actions: Vec<UnstableAction>) -> Self {
        MainAgent {
            actions,
            initial_state: Default::default(),
        }
    }

    pub fn initialize_stat(&mut self, stat: SType, value: CType, max_value: CType) {
        self.initial_state.stats[stat as usize] = value;
        self.initial_state.max_stats[stat as usize] = max_value;
    }
}

#[derive(Debug)]
pub struct AgentSimulation {
    pub time: CType,
    pub me_state: AgentState,
    pub enemy_state: AgentState,
}

impl AgentSimulation {
    pub fn next_state(
        &self,
        my_actions: &Vec<UnstableAction>,
        enemy_actions: &Vec<UnstableAction>,
    ) -> Vec<AgentSimulation> {
        let mut states = Vec::new();
        self.act(
            &mut states,
            my_actions,
            &self.me_state,
            &self.enemy_state,
            false,
        );
        self.act(
            &mut states,
            enemy_actions,
            &self.enemy_state,
            &self.me_state,
            true,
        );
        states
    }

    fn act(
        &self,
        states: &mut Vec<AgentSimulation>,
        uactions: &Vec<UnstableAction>,
        owner: &AgentState,
        target: &AgentState,
        invert: bool,
    ) {
        for uaction in uactions.iter() {
            if uaction.initial.satisfied(owner, target) {
                for (_weight, action) in uaction.paths.iter() {
                    if action.initial.satisfied(owner, target) {
                        let (updated_owner, updated_target) = action.apply(&owner, &target);
                        if invert {
                            states.push(AgentSimulation {
                                time: self.time,
                                me_state: updated_target,
                                enemy_state: updated_owner,
                            });
                        } else {
                            states.push(AgentSimulation {
                                time: self.time,
                                me_state: updated_owner,
                                enemy_state: updated_target,
                            });
                        }
                    }
                }
            }
        }
    }
}
