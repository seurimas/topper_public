use serde::{Deserialize, Serialize};

use super::{nodes::*, UnpoweredFunction};

#[derive(Serialize, Deserialize, Clone)]
pub enum UnpoweredTreeDef<U: UserNodeDefinition> {
    Sequence(Vec<UnpoweredTreeDef<U>>),
    Selector(Vec<UnpoweredTreeDef<U>>),
    Repeat(Box<UnpoweredTreeDef<U>>, usize),
    RepeatUntilSuccess(Box<UnpoweredTreeDef<U>>),
    RepeatUntilFail(Box<UnpoweredTreeDef<U>>),
    Succeeder(Box<UnpoweredTreeDef<U>>),
    Failer(Box<UnpoweredTreeDef<U>>),
    Inverter(Box<UnpoweredTreeDef<U>>),
    User(U),
}

pub trait UserNodeDefinition {
    type Model: 'static;
    type Controller: 'static;
    fn create_node(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = Self::Model, Controller = Self::Controller> + Send + Sync>;
}

impl<M: 'static, C: 'static, D: 'static> UserNodeDefinition for D
where
    D: UnpoweredFunction<Model = M, Controller = C> + Clone + Send + Sync,
{
    type Model = M;
    type Controller = C;

    fn create_node(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = Self::Model, Controller = Self::Controller> + Send + Sync>
    {
        Box::new(self.clone())
    }
}

impl<U: UserNodeDefinition> UnpoweredTreeDef<U> {
    pub fn create_tree(
        &self,
    ) -> Box<dyn UnpoweredFunction<Model = U::Model, Controller = U::Controller> + Send + Sync>
    {
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
            UnpoweredTreeDef::RepeatUntilSuccess(node_def) => {
                let node = node_def.create_tree();
                Box::new(RepeatUntilSuccess::new(node))
            }
            UnpoweredTreeDef::Succeeder(node_def) => {
                let node = node_def.create_tree();
                Box::new(Succeeder::new(node))
            }
            UnpoweredTreeDef::Inverter(node_def) => {
                let node = node_def.create_tree();
                Box::new(Inverter::new(node))
            }
            UnpoweredTreeDef::Failer(node_def) => {
                let node = node_def.create_tree();
                Box::new(Failer::new(node))
            }
            UnpoweredTreeDef::User(node_def) => node_def.create_node(),
        }
    }
}
