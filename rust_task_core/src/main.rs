use cmd_lib::{init_builtin_logger, run_cmd, run_fun};
use crate::task::{TaskBuilder, TaskCommand, Task};
use crate::workflow::{Workflow, WorkflowBuilder};

mod task;
mod workflow;

fn main() {
    let mut commands: Vec<TaskCommand> = vec![
        TaskCommand::new(String::from("python test.py")),
        TaskCommand::new(String::from("echo \"hello\"")),
        TaskCommand::new(String::from("node bla.js")),
    ];

    let task1: Task = TaskBuilder::new("sample task1".to_string())
        .commands(commands.to_vec())
        .build();
    let task2: Task = TaskBuilder::new("sample task2".to_string())
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

    let file = "/Users/alexanderbrunner/CLionProjects/rust-task/sample2.yaml";
    let mut workflow = Workflow::from_file(file);
    workflow.start();
}
