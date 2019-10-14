use crate::agents::*;
use crate::types::*;

pub struct SimulationAgent {
    pub actions: Vec<UnstableAction>,
    pub initial_state: AgentState,
}

impl SimulationAgent {
    pub fn new(actions: Vec<UnstableAction>) -> Self {
        SimulationAgent {
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
pub struct AgentSimulationSlice {
    pub entrance: String,
    pub time: CType,
    pub my_turn: bool,
    pub me_state: AgentState,
    pub enemy_state: AgentState,
}

const TURN_SIZE: CType = 10;

impl AgentSimulationSlice {
    pub fn next_state(
        &self,
        my_actions: &Vec<UnstableAction>,
        enemy_actions: &Vec<UnstableAction>,
        enemy_waits: bool,
    ) -> Vec<AgentSimulationSlice> {
        let mut states = Vec::new();
        if self.my_turn {
            self.act(
                &mut states,
                my_actions,
                &self.me_state,
                &self.enemy_state,
                true,
            );
            self.pass(&mut states);
        } else {
            let acted = self.act(
                &mut states,
                enemy_actions,
                &self.enemy_state,
                &self.me_state,
                false,
            );
            if enemy_waits || !acted {
                self.wait(&mut states);
            }
        }
        states
    }

    fn wait(&self, states: &mut Vec<AgentSimulationSlice>) {
        let mut min_balance = CType::max_value();
        for balance in self.me_state.balances.iter() {
            if *balance < min_balance && *balance > 0 {
                min_balance = *balance;
            }
        }
        for balance in self.enemy_state.balances.iter() {
            if *balance < min_balance && *balance > 0 {
                min_balance = *balance;
            }
        }
        if min_balance != CType::max_value() {
            println!(
                "Wait({}): {} ({}, {})",
                min_balance,
                self.time,
                self.me_state.stats[SType::Health as usize],
                self.enemy_state.stats[SType::Health as usize]
            );
            let mut new_me = self.me_state.clone();
            let mut new_you = self.enemy_state.clone();
            new_me.wait(min_balance);
            new_you.wait(min_balance);
            states.push(AgentSimulationSlice {
                entrance: ".".to_string(),
                time: self.time + min_balance,
                me_state: new_me,
                enemy_state: new_you,
                my_turn: !self.my_turn,
            });
        }
    }

    fn pass(&self, states: &mut Vec<AgentSimulationSlice>) {
        println!(
            "Pass: {} ({}/{})",
            self.time,
            self.me_state.stats[SType::Health as usize],
            self.enemy_state.stats[SType::Health as usize]
        );
        states.push(AgentSimulationSlice {
            entrance: "x".to_string(),
            time: self.time,
            me_state: self.me_state.clone(),
            enemy_state: self.enemy_state.clone(),
            my_turn: !self.my_turn,
        });
    }

    fn act(
        &self,
        states: &mut Vec<AgentSimulationSlice>,
        uactions: &Vec<UnstableAction>,
        owner: &AgentState,
        target: &AgentState,
        my_move: bool,
    ) -> bool {
        let mut acted = false;
        for uaction in uactions.iter() {
            if uaction.initial.satisfied(owner, target) {
                for (_weight, action) in uaction.paths.iter() {
                    if action.initial.satisfied(owner, target) {
                        let (updated_owner, updated_target) = action.apply(&owner, &target);
                        println!(
                            "{}: {} ({}/{})",
                            uaction.desc,
                            self.time,
                            self.me_state.stats[SType::Health as usize],
                            self.enemy_state.stats[SType::Health as usize]
                        );
                        if my_move {
                            states.push(AgentSimulationSlice {
                                entrance: uaction.desc.clone(),
                                time: self.time,
                                me_state: updated_owner,
                                enemy_state: updated_target,
                                my_turn: self.my_turn,
                            });
                        } else {
                            states.push(AgentSimulationSlice {
                                entrance: uaction.desc.clone(),
                                time: self.time,
                                me_state: updated_target,
                                enemy_state: updated_owner,
                                my_turn: self.my_turn,
                            });
                        }
                        acted = true;
                    }
                }
            }
        }
        acted
    }
}

#[derive(Debug)]
pub struct SimulationNode {
    pub slice: AgentSimulationSlice,
    next: Option<Vec<SimulationNode>>,
}

impl SimulationNode {
    pub fn next_till(
        &mut self,
        time: CType,
        my_actions: &Vec<UnstableAction>,
        enemy_actions: &Vec<UnstableAction>,
        enemy_waits: bool,
        depth: i32,
    ) {
        if time < self.slice.time {
            return;
        }
        if self.next.is_none() {
            self.next = Some(
                self.slice
                    .next_state(my_actions, enemy_actions, enemy_waits)
                    .drain(..)
                    .map(|slice| SimulationNode {
                        slice: slice,
                        next: None,
                    })
                    .collect(),
            );
        }
        if let Some(next) = &mut self.next {
            next.iter_mut().for_each(|node| {
                node.next_till(time, my_actions, enemy_actions, enemy_waits, depth + 1)
            });
        }
    }
}

pub struct AgentSimulation {
    pub root: SimulationNode,
    me: SimulationAgent,
    enemy: SimulationAgent,
}

impl AgentSimulation {
    pub fn new(me: SimulationAgent, enemy: SimulationAgent) -> Self {
        AgentSimulation {
            root: SimulationNode {
                slice: AgentSimulationSlice {
                    entrance: "Start".to_string(),
                    time: 0,
                    my_turn: true,
                    me_state: me.initial_state.clone(),
                    enemy_state: enemy.initial_state.clone(),
                },
                next: None,
            },
            me,
            enemy,
        }
    }
    pub fn next_till(&mut self, time: CType, enemy_waits: bool) {
        self.root
            .next_till(time, &self.me.actions, &self.enemy.actions, enemy_waits, 0);
    }
}
