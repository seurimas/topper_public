use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use topper_bt::unpowered::UnpoweredFunction;

use super::{AetBehaviorTreeDef, BehaviorController, BehaviorModel, DEFAULT_BEHAVIOR_TREE};

pub static mut LOAD_TREE_FUNC: Option<fn(&String) -> String> = None;

lazy_static! {
    pub static ref LOADED_TREES: RwLock<
        HashMap<
            String,
            Arc<
                Mutex<
                    Box<
                        UnpoweredFunction<
                                Model = BehaviorModel,
                                Controller = BehaviorController,
                            > + Sync
                            + Send,
                    >,
                >,
            >,
        >,
    > = { RwLock::new(HashMap::new()) };
}

pub fn clear_behavior_trees() {
    LOADED_TREES.write().unwrap().clear();
}

pub fn get_tree(
    tree_name: &String,
) -> Arc<
    Mutex<
        Box<
            dyn UnpoweredFunction<Model = BehaviorModel, Controller = BehaviorController>
                + Sync
                + Send,
        >,
    >,
> {
    {
        let trees = LOADED_TREES.read().unwrap();
        if let Some(tree) = trees.get(tree_name) {
            return Arc::clone(tree);
        }
    }
    {
        let mut trees = LOADED_TREES.write().unwrap();
        let tree_json = unsafe { LOAD_TREE_FUNC.unwrap()(tree_name) };
        println!("{}", tree_json);
        match serde_json::from_str::<AetBehaviorTreeDef>(&tree_json) {
            Ok(tree_def) => {
                let tree = Arc::new(Mutex::new(tree_def.create_tree()));
                trees.insert(tree_name.to_string(), Arc::clone(&tree));
                tree
            }
            Err(err) => {
                println!("Failed to load {}: {:?}", tree_name, err);
                Arc::new(Mutex::new(DEFAULT_BEHAVIOR_TREE.create_tree()))
            }
        }
    }
}
