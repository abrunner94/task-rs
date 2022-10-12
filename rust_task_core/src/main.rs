#![deny(elided_lifetimes_in_paths)]

use std::borrow::{Borrow, BorrowMut};
use cmd_lib::{init_builtin_logger, run_cmd, run_fun};
use yaml_rust::Yaml;
use crate::task::{load_task_file, parse_and_run_tasks, TaskBuilder, TaskCommand, Task};
use snailquote;
use crate::workflow::{Workflow, WorkflowBuilder};

mod task;
mod workflow;

fn main() {
    let mut commands: Vec<TaskCommand> = vec![
        TaskCommand::new("python test.py"),
        TaskCommand::new("echo \"hello\""),
        TaskCommand::new("node bla.js"),
    ];

    let task1: Task = TaskBuilder::new("sample task1")
        .commands(commands.to_vec())
        .build();
    let task2: Task = TaskBuilder::new("sample task2")
        .commands(commands.to_vec())
        .build();

    // WorkflowBuilder::new("my workflow")
    //     .add_task(task1)
    //     .add_task(task2)
    //     .build()
    //     .start();

    // WorkflowBuilder::new("my workflow")
    //     .add_tasks(vec![task1, task2])
    //     .build()
    //     .start();

    // let mut tasks = vec![task1, task2];
    // for task in  tasks.iter_mut() {
    //     task.start();
    // }

    let file = String::from("/Users/abrunner/CLionProjects/rust-task/sample2.yaml");
    let mut workflow = Workflow::from_file(file, "test workflow");
    workflow.start();
}
