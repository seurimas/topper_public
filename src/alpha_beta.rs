use crate::actions::*;
use crate::types::*;

#[derive(Debug)]
pub struct SimulationState {
    pub time: CType,
    pub turn: usize,
    pub states: Vec<AgentState>,
}

impl SimulationState {
    fn new(states: &Vec<AgentState>) -> Self {
        SimulationState {
            time: 0,
            turn: 0,
            states: states.clone(),
        }
    }

    fn apply_action(&self, action: &StateAction, me_id: usize, you_id: usize) -> SimulationState {
        let me = &self.states[me_id];
        let you = &self.states[you_id];
        let new_me = me.clone();
        let new_you = you.clone();
        let (new_me, new_you) = action.apply(&new_me, &new_you);
        let me_pair = (me_id, new_me);
        let you_pair = (you_id, new_you);
        self.update_pair(0, me_pair, you_pair)
    }

    fn maximizing_agent(&self) -> bool {
        self.states[self.turn].flags[FType::Player as usize]
    }

    fn update_pair(
        &self,
        new_turn: usize,
        (me_id, me): (usize, AgentState),
        (you_id, you): (usize, AgentState),
    ) -> Self {
        let mut updated_states = self.states.clone();
        updated_states[me_id] = me;
        updated_states[you_id] = you;
        SimulationState {
            time: self.time,
            turn: new_turn,
            states: updated_states,
        }
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

    fn wait<'s>(&self) -> Option<SimulationState> {
        if let Some(min_balance) = self.wait_time() {
            let new_states = self
                .states
                .iter()
                .map(|state| {
                    let mut new_state = state.clone();
                    new_state.wait(min_balance);
                    new_state
                })
                .collect();
            Some(SimulationState {
                time: self.time + min_balance,
                turn: 0,
                states: new_states,
            })
        } else {
            None
        }
    }

    fn pass(&self) -> SimulationState {
        SimulationState {
            time: self.time,
            turn: self.turn + 1,
            states: self.states.clone(),
        }
    }
}

type PassChecker = Fn(&SimulationState, &Vec<Transition>) -> bool;
type MoveScorer = Fn(&SimulationState, &Transition) -> i32;

struct SimulationIterator<'s> {
    index: usize,
    actions: &'s Vec<StateAction>,
    base_state: &'s SimulationState,
    transitions: Vec<Transition>,
}

impl<'s> SimulationIterator<'s> {
    fn new(
        base_state: &'s SimulationState,
        actions: &'s Vec<StateAction>,
        can_pass: &'s PassChecker,
        score_move: &'s MoveScorer,
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
                        vec![target_index],
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
            base_state,
            actions,
            index: 0,
            transitions,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Transition {
    Act(String, usize, Vec<usize>),
    Wait(CType),
    Pass,
    Null,
}

impl<'s> Iterator for SimulationIterator<'s> {
    type Item = (Transition, SimulationState);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.transitions.len() {
            return None;
        }
        let transition = &self.transitions[self.index];
        self.index += 1;
        match transition {
            Transition::Act(_name, action_id, target_id) => {
                let action = &self.actions[*action_id];
                Some((
                    transition.clone(),
                    self.base_state
                        .apply_action(action, self.base_state.turn, target_id[0]),
                ))
            }
            Transition::Wait(_) => Some((transition.clone(), self.base_state.wait().unwrap())),
            Transition::Pass => Some((transition.clone(), self.base_state.pass())),
            Transition::Null => None,
        }
    }
}

pub type StateScorer = Box<Fn(&SimulationState) -> i32>;

impl SimulationState {
    fn next_iter<'s>(
        &'s self,
        actions: &'s Vec<StateAction>,
        can_pass: &'s PassChecker,
        score_moves: &'s MoveScorer,
    ) -> SimulationIterator<'s> {
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
            &SimulationState::new(&self.initial_states),
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
            &SimulationState::new(&self.initial_states),
            time,
            alpha,
            beta,
            0,
            stats,
        )
    }

    fn alpha_beta(
        &self,
        state: &SimulationState,
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
                // println!("Iterating {} {}...", state.turn, depth);
                for (transition, next_state) in
                    state.next_iter(&self.actions[state.turn], &self.can_pass, &self.score_move)
                {
                    stats.state_count += 1;
                    // println!("Me {:?}", transition);
                    let (transitions, next_value) =
                        self.alpha_beta(&next_state, time, alpha, beta, depth + 1, stats);
                    if next_value
                        > value
                            .get_or_insert((Transition::Null, vec![], i32::min_value()))
                            .2
                    {
                        // println!("+ {} {}", next_value, depth);
                        value = Some((transition, transitions, next_value));
                        alpha = i32::max(alpha, next_value);
                        if alpha >= beta {
                            // println!("Broke {} {}", alpha, beta);
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
                // println!("Iterating {} {}...", state.turn, depth);
                for (transition, next_state) in
                    state.next_iter(&self.actions[state.turn], &self.can_pass, &self.score_move)
                {
                    stats.state_count += 1;
                    // println!("You {:?}", transition);
                    let (transitions, next_value) =
                        self.alpha_beta(&next_state, time, alpha, beta, depth + 1, stats);
                    if next_value
                        < value
                            .get_or_insert((Transition::Null, vec![], i32::max_value()))
                            .2
                    {
                        value = Some((transition, transitions, next_value));
                        beta = i32::min(beta, next_value);
                        if alpha >= beta {
                            // println!("Broked {} {}", alpha, beta);
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
