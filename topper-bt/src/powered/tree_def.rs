use super::{ConsumeGas, PoweredFunction, Repeat, RepeatUntilFail, Selector, Sequence};

pub enum PoweredTreeDef<U: UserNodeDefinition> {
    Sequence(Vec<PoweredTreeDef<U>>),
    Selector(Vec<PoweredTreeDef<U>>),
    Repeat(Box<PoweredTreeDef<U>>, usize),
    RepeatUntilFail(Box<PoweredTreeDef<U>>),
    UseGas(i32),
    User(U),
}

pub trait UserNodeDefinition {
    type World: 'static;
    fn create_node(&self) -> Box<dyn PoweredFunction<World = Self::World>>;
}

impl<U: UserNodeDefinition> PoweredTreeDef<U> {
    pub fn create_tree(&self) -> Box<dyn PoweredFunction<World = U::World>> {
        match self {
            PoweredTreeDef::Sequence(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Sequence::new(nodes))
            }
            PoweredTreeDef::Selector(node_defs) => {
                let nodes = node_defs
                    .iter()
                    .map(|node_def| node_def.create_tree())
                    .collect();
                Box::new(Selector::new(nodes))
            }
            PoweredTreeDef::Repeat(node_def, repeats) => {
                let node = node_def.create_tree();
                Box::new(Repeat::new(node, *repeats))
            }
            PoweredTreeDef::RepeatUntilFail(node_def) => {
                let node = node_def.create_tree();
                Box::new(RepeatUntilFail::new(node))
            }
            PoweredTreeDef::UseGas(gas_used) => Box::new(ConsumeGas::new(*gas_used)),
            PoweredTreeDef::User(node_def) => node_def.create_node(),
        }
    }
}
