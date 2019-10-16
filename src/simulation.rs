use crate::agents::*;
use crate::types::*;

#[derive(PartialEq, Clone, Copy)]
pub enum SimulationStrategy {
    Maximin,
    Strict,
}

pub struct SimulationAgent {
    pub actions: Vec<UnstableAction>,
    pub initial_state: AgentState,
    pub strategy: SimulationStrategy,
}

impl SimulationAgent {
    pub fn new(strategy: SimulationStrategy, actions: Vec<UnstableAction>) -> Self {
        SimulationAgent {
            actions,
            initial_state: Default::default(),
            strategy,
        }
    }

    pub fn initialize_stat(&mut self, stat: SType, value: CType, max_value: CType) {
        self.initial_state.stats[stat as usize] = value;
        self.initial_state.max_stats[stat as usize] = max_value;
    }
}

pub type StateScorer = Fn(&AgentSimulationSlice) -> i32;

#[derive(Debug, Clone)]
pub struct AgentSimulationSlice {
    pub entrance: String,
    pub time: CType,
    pub states: Vec<AgentState>,
}

const TURN_SIZE: CType = 10;

impl AgentSimulationSlice {
    pub fn generate_moves(
        &self,
        my_actions: &Vec<UnstableAction>,
        strategy: SimulationStrategy,
        id: usize,
        targets: &Vec<usize>,
    ) -> Vec<AgentSimulationSlice> {
        let mut states = Vec::new();
        self.act(&mut states, my_actions, id, targets);
        states
    }

    fn wait(&self) -> Option<AgentSimulationSlice> {
        let mut min_balance = CType::max_value();
        for agent_state in self.states.iter() {
            for balance in agent_state.balances.iter() {
                if *balance < min_balance && *balance > 0 {
                    min_balance = *balance;
                }
            }
        }
        if min_balance != CType::max_value() {
            let new_states = self
                .states
                .iter()
                .map(|state| {
                    let mut new_state = state.clone();
                    new_state.wait(min_balance);
                    new_state
                })
                .collect();
            Some(AgentSimulationSlice {
                entrance: ".".to_string(),
                time: self.time + min_balance,
                states: new_states,
            })
        } else {
            None
        }
    }

    fn act(
        &self,
        states: &mut Vec<AgentSimulationSlice>,
        uactions: &Vec<UnstableAction>,
        id: usize,
        targets: &Vec<usize>,
    ) -> bool {
        let mut acted = false;
        let owner = &self.states[id];
        for target_id in targets.iter() {
            let target = &self.states[*target_id];
            for uaction in uactions.iter() {
                if uaction.initial.satisfied(&owner, &target) {
                    for (_weight, action) in uaction.paths.iter() {
                        if action.initial.satisfied(&owner, &target) {
                            // println!("{}", uaction.desc);
                            let updated_owner = owner.clone();
                            let updated_target = target.clone();
                            let (updated_owner, updated_target) =
                                action.apply(&updated_owner, &updated_target);
                            let mut new_states = self.states.clone();
                            new_states[id] = updated_owner;
                            new_states[*target_id] = updated_target;
                            states.push(AgentSimulationSlice {
                                entrance: uaction.desc.clone(),
                                time: self.time,
                                states: new_states,
                            });
                            acted = true;
                        }
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
    pub turn_id: usize,
    next: Option<Vec<SimulationNode>>,
}

impl SimulationNode {
    pub fn size(&self) -> u32 {
        if let Some(slices) = &self.next {
            1 + slices
                .iter()
                .map(|node| node.size())
                .fold(0, |acc, val| acc + val)
        } else {
            1
        }
    }
}

pub struct AgentSimulation {
    agents: Vec<SimulationAgent>,
    ally_ids: Vec<usize>,
    enemy_ids: Vec<usize>,
    sides: Vec<bool>,
    pub evaluated: i32,
}

impl AgentSimulation {
    pub fn new() -> Self {
        AgentSimulation {
            agents: vec![],
            ally_ids: vec![],
            enemy_ids: vec![],
            sides: vec![],
            evaluated: 0,
        }
    }

    pub fn add_ally(&mut self, agent: SimulationAgent) {
        self.agents.push(agent);
        self.ally_ids.push(self.agents.len() - 1);
        self.sides.push(true);
    }

    fn alpha_beta(
        &mut self,
        eval: &'static StateScorer,
        time: CType,
        mut alpha: i32,
        mut beta: i32,
        node: &mut SimulationNode,
        depth: i32,
    ) -> (i32, Vec<String>) {
        if node.next.is_none() {
            if node.slice.time > time {
            } else {
                self.fill(node);
            }
        }
        if let Some(next) = &mut node.next {
            if self.sides[node.turn_id] {
                let mut value = i32::min_value();
                let mut path = vec![];
                for next_node in next.iter_mut() {
                    let (next_value, next_path) =
                        self.alpha_beta(eval, time, alpha, beta, next_node, depth + 1);
                    if next_value >= value {
                        value = next_value;
                        path = next_path;
                    }
                    alpha = i32::max(alpha, value);
                    if alpha >= beta {
                        break;
                    }
                }
                path = path.clone();
                path.push(node.slice.entrance.clone());
                // println!("a {:?}", path);
                (value, path)
            } else {
                let mut value = i32::max_value();
                let mut path = vec![];
                for next_node in next.iter_mut() {
                    let (next_value, next_path) =
                        self.alpha_beta(eval, time, alpha, beta, next_node, depth + 1);
                    if next_value <= value {
                        value = next_value;
                        path = next_path;
                    }
                    beta = i32::min(beta, value);
                    if alpha >= beta {
                        break;
                    }
                }
                path = path.clone();
                path.push(node.slice.entrance.clone());
                // println!("e {:?}", path);
                (value, path)
            }
        } else {
            self.evaluated += 1;
            (eval(&node.slice), vec![node.slice.entrance.clone()])
        }
    }

    pub fn best_path(
        &mut self,
        eval: &'static StateScorer,
        time: CType,
        node: &mut SimulationNode,
    ) -> Vec<String> {
        self.alpha_beta(eval, time, i32::min_value(), i32::max_value(), node, 0)
            .1
    }

    pub fn add_enemy(&mut self, agent: SimulationAgent) {
        self.agents.push(agent);
        self.enemy_ids.push(self.agents.len() - 1);
        self.sides.push(false);
    }

    fn fill(&self, node: &mut SimulationNode) {
        let turn = node.turn_id;
        let next = node.next.get_or_insert(Vec::new());
        let friend = self.sides[turn];
        let targets = if friend {
            &self.enemy_ids
        } else {
            &self.ally_ids
        };
        let actions = &self.agents[turn].actions;
        let strategy = self.agents[turn].strategy;
        /* println!(
            "{} {} {:?}",
            node.slice.time,
            turn,
            node.slice
                .states
                .iter()
                .map(|state| state.stats[SType::Health as usize])
                .collect::<Vec<_>>()
        ); */
        let moves = node
            .slice
            .generate_moves(&actions, strategy, turn, &targets);
        let moves_found = moves.len();
        for next_slice in moves.into_iter() {
            let mut new_node = SimulationNode {
                slice: next_slice,
                turn_id: 0,
                next: None,
            };
            next.push(new_node);
        }
        if moves_found == 0 || strategy != SimulationStrategy::Strict {
            if node.turn_id < self.agents.len() - 1 {
                let new_node = SimulationNode {
                    slice: AgentSimulationSlice {
                        entrance: "pass".to_string(),
                        time: node.slice.time,
                        states: node.slice.states.clone(),
                    },
                    turn_id: node.turn_id + 1,
                    next: None,
                };
                next.push(new_node);
            } else {
                if let Some(waited) = node.slice.wait() {
                    let new_node = SimulationNode {
                        slice: waited,
                        turn_id: 0,
                        next: None,
                    };
                    next.push(new_node);
                }
            }
        }
        if next.len() == 0 {
            node.next = None;
        }
    }

    pub fn root(&mut self) -> SimulationNode {
        SimulationNode {
            slice: AgentSimulationSlice {
                entrance: "Start".to_string(),
                time: 0,
                states: self
                    .agents
                    .iter()
                    .map(|agent| agent.initial_state.clone())
                    .collect(),
            },
            turn_id: 0,
            next: None,
        }
    }
}
