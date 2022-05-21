use serde::{Deserialize, Serialize};

use super::{Repeat, RepeatUntilFail, Selector, Sequence, UnpoweredFunction};

#[derive(Serialize, Deserialize)]
pub enum UnpoweredTreeDef<U: UserNodeDefinition> {
    Sequence(Vec<UnpoweredTreeDef<U>>),
    Selector(Vec<UnpoweredTreeDef<U>>),
    Repeat(Box<UnpoweredTreeDef<U>>, usize),
    RepeatUntilFail(Box<UnpoweredTreeDef<U>>),
    User(U),
}

pub trait UserNodeDefinition {
    type Model: 'static;
    type Controller: 'static;
    fn create_node(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = Self::Model, Controller = Self::Controller>>;
}

impl<M: 'static, C: 'static, D: 'static> UserNodeDefinition for D
where
    D: UnpoweredFunction<Model = M, Controller = C> + Clone,
{
    type Model = M;
    type Controller = C;

    fn create_node(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = Self::Model, Controller = Self::Controller>> {
        Box::new(self.clone())
    }
}

impl<U: UserNodeDefinition> UnpoweredTreeDef<U> {
    pub fn create_tree(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller>> {
        match self {
            UnpoweredTreeDef::Sequence(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Sequence::new(nodes))
            }
            UnpoweredTreeDef::Selector(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Selector::new(nodes))
            }
            UnpoweredTreeDef::Repeat(node_def, repeats) => {
                let node = node_def.create_tree();
                Box::new(Repeat::new(node, *repeats))
            }
            UnpoweredTreeDef::RepeatUntilFail(node_def) => {
                let node = node_def.create_tree();
                Box::new(RepeatUntilFail::new(node))
            }
            UnpoweredTreeDef::User(node_def) => node_def.create_node(),
        }
    }
}
