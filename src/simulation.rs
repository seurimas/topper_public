use crate::agents::*;
use crate::types::*;

#[derive(Debug)]
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
    pub time: CType,
    pub me_state: AgentState,
    pub enemy_state: AgentState,
}

impl AgentSimulationSlice {
    pub fn next_state(
        &self,
        my_actions: &Vec<UnstableAction>,
        enemy_actions: &Vec<UnstableAction>,
    ) -> Vec<AgentSimulationSlice> {
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
        self.wait(&mut states, &self.me_state, false);
        self.wait(&mut states, &self.enemy_state, true);
        states
    }

    fn wait(&self, states: &mut Vec<AgentSimulationSlice>, current: &AgentState, enemy: bool) {
        let mut min_balance = CType::max_value();
        for balance in current.balances.iter() {
            if *balance < min_balance && *balance > 0 {
                min_balance = *balance;
            }
        }

        if min_balance != CType::max_value() {
            if enemy {
                states.push(AgentSimulationSlice {
                    time: self.time + min_balance,
                    me_state: self.me_state.clone(),
                    enemy_state: current.wait(min_balance),
                });
            } else {
                states.push(AgentSimulationSlice {
                    time: self.time + min_balance,
                    me_state: current.wait(min_balance),
                    enemy_state: self.enemy_state.clone(),
                });
            }
        }
    }

    fn act(
        &self,
        states: &mut Vec<AgentSimulationSlice>,
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
                            states.push(AgentSimulationSlice {
                                time: self.time,
                                me_state: updated_target,
                                enemy_state: updated_owner,
                            });
                        } else {
                            states.push(AgentSimulationSlice {
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

#[derive(Debug)]
pub struct SimulationNode {
    slice: AgentSimulationSlice,
    next: Option<Vec<SimulationNode>>,
}

impl SimulationNode {
    pub fn next_till(
        &mut self,
        time: CType,
        my_actions: &Vec<UnstableAction>,
        enemy_actions: &Vec<UnstableAction>,
        depth: i32,
    ) {
        println!("{}, {}, {}", time, self.slice.time, depth);
        if time < self.slice.time {
            return;
        }
        if self.next.is_none() {
            self.next = Some(
                self.slice
                    .next_state(my_actions, enemy_actions)
                    .drain(..)
                    .map(|slice| SimulationNode {
                        slice: slice,
                        next: None,
                    })
                    .collect(),
            );
        }
        if let Some(next) = &mut self.next {
            next.iter_mut()
                .for_each(|node| node.next_till(time, my_actions, enemy_actions, depth + 1));
        }
    }
}

#[derive(Debug)]
pub struct AgentSimulation {
    root: SimulationNode,
    me: SimulationAgent,
    enemy: SimulationAgent,
}

impl AgentSimulation {
    pub fn new(me: SimulationAgent, enemy: SimulationAgent) -> Self {
        AgentSimulation {
            root: SimulationNode {
                slice: AgentSimulationSlice {
                    time: 0,
                    me_state: me.initial_state.clone(),
                    enemy_state: enemy.initial_state.clone(),
                },
                next: None,
            },
            me,
            enemy,
        }
    }
    pub fn next_till(&mut self, time: CType) {
        self.root
            .next_till(time, &self.me.actions, &self.enemy.actions, 0);
    }
}
