use std::borrow::{Borrow, BorrowMut};
use cmd_lib::{init_builtin_logger, run_cmd, run_fun};
use yaml_rust::Yaml;
use crate::task::{load_task_file, parse_and_run_tasks, TaskBuilder, TaskCommand, TaskRunner};
use snailquote;

mod task;

fn main() {
    let mut commands: Vec<TaskCommand> = vec![
        TaskCommand::new("python test.py"),
        TaskCommand::new("echo \"hello\""),
        TaskCommand::new("node bla.js"),
    ];

    let task_runner1: TaskRunner = TaskBuilder::new("sample task1")
        .commands(commands.to_vec())
        .build();
    let task_runner2: TaskRunner = TaskBuilder::new("sample task2")
        .commands(commands.to_vec())
        .build();
    
    let mut task_runners = vec![task_runner1, task_runner2];
    for runner in  task_runners.iter_mut() {
        runner.start();
    }
}
