use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use topper_aetolia::bt::{BehaviorController, BehaviorModel, LOAD_TREE_FUNC};

static mut BEHAVIOR_TREES_DIR: &str = "";

pub fn load_tree(tree_name: &String) -> String {
    if let Ok(file) = unsafe { File::open(format!("{}/{}.json", BEHAVIOR_TREES_DIR, tree_name)) } {
        let mut reader = BufReader::new(file);
        let mut result = String::new();
        reader.read_to_string(&mut result);
        result
    } else {
        format!("")
    }
}

pub fn initialize_load_tree_func(behavior_trees_dir: String) {
    unsafe {
        BEHAVIOR_TREES_DIR = Box::leak(behavior_trees_dir.into_boxed_str());
        LOAD_TREE_FUNC = Some(load_tree);
    }
}
