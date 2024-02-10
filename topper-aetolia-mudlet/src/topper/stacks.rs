use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use topper_aetolia::classes::LOAD_STACK_FUNC;

static mut STACKS_DIRECTORY: &str = "";

pub fn load_stack(class_name: &String, stack_name: &String) -> String {
    if let Ok(file) = unsafe {
        File::open(format!(
            "{}/{}/{}.json",
            STACKS_DIRECTORY, class_name, stack_name
        ))
    } {
        let mut reader = BufReader::new(file);
        let mut result = String::new();
        reader.read_to_string(&mut result);
        result
    } else {
        unsafe { format!("{}", STACKS_DIRECTORY) }
    }
}

pub fn initialize_load_stack_func(stacks_dir: String) {
    unsafe {
        STACKS_DIRECTORY = Box::leak(stacks_dir.into_boxed_str());
        LOAD_STACK_FUNC = Some(load_stack);
    }
}
