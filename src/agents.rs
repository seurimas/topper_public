use crate::types::*;

#[derive(Debug)]
pub struct Action {
    pub name: &'static str,
    pub changes: Vec<StateChange>,
    pub initial: Vec<StateMatcher>,
}

pub trait StateConditions {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool;
}

pub trait StateUpdate {
    fn apply(&self, owner: &AgentState, other: &AgentState) -> (AgentState, AgentState);
}

impl StateConditions for Vec<StateMatcher> {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool {
        self.iter()
            .find(|&matcher| !matcher.check(owner, other))
            .is_none()
    }
}

impl StateUpdate for Action {
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

#[derive(Debug)]
pub struct UnstableAction {
    pub paths: Vec<(i32, Action)>,
    pub initial: Vec<StateMatcher>,
}

impl Action {
    pub fn always(self) -> UnstableAction {
        UnstableAction {
            paths: vec![(1, self)],
            initial: vec![],
        }
    }
}
