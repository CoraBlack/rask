use std::path::PathBuf;

mod task;

fn get_arg() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() || args.len() < 2 {
        return None;
    }

    let arg = args[1].clone();
    Some(arg.clone())}

fn main() {
    let Some(task_path) = get_arg() else {
        panic!("Failed to get task file path!");
    };

    let task_path = PathBuf::from(task_path);

    let task = task::TaskJson::new(task_path)
        .expect("Failed to create task from file!");

    task.run().expect("Failed to run task!");
}
