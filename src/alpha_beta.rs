use crate::actions::*;
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

#[derive(Debug, Clone)]
pub struct SimulationState {
    pub time: CType,
    pub turn: usize,
    pub states: Vec<AgentState>,
}

impl PartialEq for SimulationState {
    fn eq(&self, other: &Self) -> bool {
        if self.time != other.time {
            false
        } else if self.turn != other.turn {
            false
        } else {
            let mut different = false;
            for i in 0..self.states.len() {
                if self.states[i] != other.states[i] {
                    different = true;
                }
            }
            different
        }
    }
}

pub fn multi_index<T>(slc: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    if a == b {
        panic!();
    } else {
        if a >= slc.len() || b >= slc.len() {
            panic!();
        } else {
            unsafe {
                let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
                let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
                (ar, br)
            }
        }
    }
}

impl SimulationState {
    fn new(states: &Vec<AgentState>) -> Self {
        SimulationState {
            time: 0,
            turn: 0,
            states: states.clone(),
        }
    }

    fn apply_action(
        &mut self,
        action: &StateAction,
        me_id: usize,
        you_id: usize,
    ) -> (usize, usize, StateRevert) {
        let (me, you) = multi_index(&mut self.states, me_id, you_id);
        let revert = action.apply(me, you);
        self.turn = 0;
        (me_id, you_id, revert)
    }

    fn revert(&mut self, revert: &StateRevert, me_id: usize, you_id: usize) {
        let (me, you) = multi_index(&mut self.states, me_id, you_id);
        self.turn = me_id;
        revert(me, you);
    }

    fn maximizing_agent(&self) -> bool {
        self.states[self.turn].is(FType::Player)
    }

    fn wait_time(&self) -> Option<CType> {
        let mut min_balance = CType::max_value();
        for agent_state in self.states.iter() {
            for balance in agent_state.balances.iter() {
                if *balance < min_balance && *balance > 0 {
                    min_balance = *balance;
                }
            }
        }
        if min_balance != CType::max_value() {
            Some(min_balance)
        } else {
            None
        }
    }

    fn wait<'s>(&mut self, min_balance: CType) -> (usize, CType) {
        let original_turn = self.turn;
        for state in self.states.iter_mut() {
            state.wait(min_balance);
        }
        self.turn = 0;
        self.time += min_balance;
        (original_turn, min_balance)
    }

    fn unwait(&mut self, turn: usize, duration: CType) {
        for state in self.states.iter_mut() {
            state.wait(-duration);
        }
        self.turn = turn;
        self.time -= duration;
    }

    fn pass(&mut self) {
        self.turn += 1;
    }

    fn unpass(&mut self) {
        self.turn -= 1;
    }
}

type PassChecker = Fn(&SimulationState, &Vec<Transition>) -> bool;
type MoveScorer = Fn(&SimulationState, &Transition) -> i32;

struct SimulationIterator {
    index: usize,
    transitions: Vec<Transition>,
}

impl SimulationIterator {
    fn new(
        base_state: &SimulationState,
        actions: &Vec<StateAction>,
        can_pass: &PassChecker,
        score_move: &MoveScorer,
    ) -> Self {
        let mut transitions = vec![];
        let me = &base_state.states[base_state.turn];
        for action_index in 0..actions.len() {
            let action = &actions[action_index];
            for target_index in 0..base_state.states.len() {
                let you = &base_state.states[target_index];
                if base_state.turn != target_index && action.satisfied(me, you) {
                    transitions.push(Transition::Act(
                        action.name.clone(),
                        action_index,
                        target_index,
                    ));
                }
            }
        }
        if (can_pass)(&base_state, &transitions) {
            if base_state.turn == base_state.states.len() - 1 {
                if let Some(wait_time) = base_state.wait_time() {
                    transitions.push(Transition::Wait(wait_time));
                }
            } else {
                transitions.push(Transition::Pass);
            }
        }
        //        println!("Before: {:?}", transitions);
        transitions.sort_by_key(|transition| score_move(&base_state, transition));
        //      println!("After: {:?}", transitions);
        SimulationIterator {
            index: 0,
            transitions,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Transition {
    Act(String, usize, usize),
    Wait(CType),
    Pass,
}

pub enum Reversion {
    Unact(usize, usize, StateRevert),
    Unpass,
    Unwait(usize, CType),
}

impl Transition {
    pub fn apply(&self, actions: &Vec<StateAction>, state: &mut SimulationState) -> Reversion {
        match self {
            Transition::Act(_name, action_id, target) => {
                let (me, you, revert) =
                    state.apply_action(&actions[*action_id], state.turn, *target);
                Reversion::Unact(me, you, revert)
            }
            Transition::Wait(duration) => {
                let (turn, time) = state.wait(*duration);
                Reversion::Unwait(turn, time)
            }
            Transition::Pass => {
                state.pass();
                Reversion::Unpass
            }
        }
    }
}

impl Reversion {
    pub fn revert(&self, state: &mut SimulationState) {
        match self {
            Reversion::Unact(me, you, revert) => {
                state.revert(revert, *me, *you);
            }
            Reversion::Unwait(turn, time) => {
                state.unwait(*turn, *time);
            }
            Reversion::Unpass => {
                state.unpass();
            }
        }
    }
}

impl Iterator for SimulationIterator {
    type Item = (Transition);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.transitions.len() {
            return None;
        }
        let transition = &self.transitions[self.index];
        self.index += 1;
        Some(transition.clone())
    }
}

pub type StateScorer = Box<Fn(&SimulationState) -> i32>;

impl SimulationState {
    fn next_iter(
        &self,
        actions: &Vec<StateAction>,
        can_pass: &PassChecker,
        score_moves: &MoveScorer,
    ) -> SimulationIterator {
        SimulationIterator::new(self, actions, can_pass, score_moves)
    }
}

#[derive(Debug)]
pub struct Stats {
    pub max_depth: i32,
    pub evaluated: i32,
    pub state_count: i32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            max_depth: 0,
            evaluated: 0,
            state_count: 0,
        }
    }
}

pub struct ABSimulation {
    pub eval: StateScorer,
    pub actions: Vec<Vec<StateAction>>,
    pub can_pass: Box<PassChecker>,
    pub score_move: Box<MoveScorer>,
    pub initial_states: Vec<AgentState>,
}

impl ABSimulation {
    pub fn new(
        eval: StateScorer,
        can_pass: Box<PassChecker>,
        score_move: Box<MoveScorer>,
        actions: Vec<Vec<StateAction>>,
        initial_states: Vec<AgentState>,
    ) -> Self {
        ABSimulation {
            eval,
            actions,
            can_pass,
            score_move,
            initial_states,
        }
    }

    pub fn run(&self, time: CType, stats: &mut Stats) -> (Vec<Transition>, i32) {
        self.alpha_beta(
            &mut SimulationState::new(&self.initial_states),
            time,
            i32::min_value(),
            i32::max_value(),
            0,
            stats,
        )
    }

    pub fn run_with_window(
        &self,
        time: CType,
        alpha: i32,
        beta: i32,
        stats: &mut Stats,
    ) -> (Vec<Transition>, i32) {
        self.alpha_beta(
            &mut SimulationState::new(&self.initial_states),
            time,
            alpha,
            beta,
            0,
            stats,
        )
    }

    fn alpha_beta(
        &self,
        state: &mut SimulationState,
        time: CType,
        mut alpha: i32,
        mut beta: i32,
        depth: i32,
        stats: &mut Stats,
    ) -> (Vec<Transition>, i32) {
        if state.time > time {
            stats.max_depth = i32::max(stats.max_depth, depth);
            stats.evaluated += 1;
            (vec![], (self.eval)(state))
        } else {
            if state.maximizing_agent() {
                let mut value = None;
                let next_transitions = state
                    .next_iter(&self.actions[state.turn], &self.can_pass, &self.score_move)
                    .transitions;
                for transition in next_transitions {
                    let reversion = transition.apply(&self.actions[state.turn], state);
                    stats.state_count += 1;
                    let (transitions, next_value) =
                        self.alpha_beta(state, time, alpha, beta, depth + 1, stats);
                    reversion.revert(state);
                    if let Some((_transition, _transitions, best_value)) = &value {
                        if next_value > *best_value {
                            value = Some((transition, transitions, next_value));
                            alpha = i32::max(alpha, next_value);
                            if alpha >= beta {
                                break;
                            }
                        }
                    } else {
                        value = Some((transition, transitions, next_value));
                        alpha = i32::max(alpha, next_value);
                        if alpha >= beta {
                            break;
                        }
                    }
                }
                if let Some((lower, mut transitions, value)) = value {
                    transitions.push(lower);
                    (transitions, value)
                } else {
                    stats.max_depth = i32::max(stats.max_depth, depth);
                    stats.evaluated += 1;
                    (vec![], (self.eval)(state))
                }
            } else {
                let mut value = None;
                let next_transitions = state
                    .next_iter(&self.actions[state.turn], &self.can_pass, &self.score_move)
                    .transitions;
                for transition in next_transitions {
                    let reversion = transition.apply(&self.actions[state.turn], state);
                    stats.state_count += 1;
                    let (transitions, next_value) =
                        self.alpha_beta(state, time, alpha, beta, depth + 1, stats);
                    reversion.revert(state);
                    if let Some((_transition, _transitions, best_value)) = &value {
                        if next_value < *best_value {
                            value = Some((transition, transitions, next_value));
                            beta = i32::min(beta, next_value);
                            if alpha >= beta {
                                break;
                            }
                        }
                    } else {
                        value = Some((transition, transitions, next_value));
                        beta = i32::min(beta, next_value);
                        if alpha >= beta {
                            break;
                        }
                    }
                }
                if let Some((lower, mut transitions, value)) = value {
                    transitions.push(lower);
                    (transitions, value)
                } else {
                    stats.max_depth = i32::max(stats.max_depth, depth);
                    stats.evaluated += 1;
                    (vec![], (self.eval)(state))
                }
            }
        }
    }
}
