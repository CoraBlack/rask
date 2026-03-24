use std::{path::PathBuf};

mod task;
mod utils;

fn main() {
    let Some(task_path) = utils::get_arg() else {
        utils::error_and_exit("Task file is invivid".to_string())
    };

    let task_path = PathBuf::from(task_path);

    let Ok(task) = task::TaskJson::new(task_path) else {
        utils::error_and_exit("Failed to create task".to_string())
    };

    if let Err(e) = task.run() {
        utils::error_and_exit(format!("{}", e).to_string())
    };
}
