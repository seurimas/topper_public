use crate::types::*;

pub struct Action {
    pub name: String,
    pub changes: Vec<StateChange>,
    pub initial: Vec<StateMatcher>,
}

pub trait StateConditions {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool;
}

impl StateConditions for Vec<StateMatcher> {
    fn satisfied(&self, owner: &AgentState, other: &AgentState) -> bool {
        self.iter()
            .find(|&matcher| !matcher(owner, other))
            .is_none()
    }
}

impl Action {
    pub fn apply(&self, owner: &AgentState, other: &AgentState) -> (AgentState, AgentState) {
        let mut me = owner.clone();
        let mut you = other.clone();
        for change in self.changes.iter() {
            change(&mut me, &mut you);
        }
        (me, you)
    }
}

pub struct UnstableAction {
    pub desc: String,
    pub paths: Vec<(i32, Action)>,
    pub initial: Vec<StateMatcher>,
}

impl Action {
    pub fn always(self) -> UnstableAction {
        UnstableAction {
            desc: self.name.clone(),
            paths: vec![(1, self)],
            initial: vec![],
        }
    }
}
