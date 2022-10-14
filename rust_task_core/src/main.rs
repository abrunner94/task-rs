use crate::task::{Task, TaskBuilder, TaskCommand};
use crate::workflow::{Workflow, WorkflowBuilder};
use cmd_lib::{init_builtin_logger, run_cmd, run_fun};

mod task;
mod workflow;

fn main() {
    // Create commands
    let commands1: Vec<TaskCommand> = vec![
        TaskCommand::new(String::from("python test.py")),
        TaskCommand::new(String::from("node test.js")),
    ];

    let commands2: Vec<TaskCommand> = vec![TaskCommand::new(String::from("echo \"hello\""))];

    // Create tasks
    let task1: Task = TaskBuilder::new("sample task1".to_string())
        .commands(commands1.to_vec())
        .build();
    let task2: Task = TaskBuilder::new("sample task2".to_string())
        .commands(commands2.to_vec())
        .build();

    // Create workflows
    // WorkflowBuilder::new("my workflow")
    //     .add_task(task1)
    //     .add_task(task2)
    //     .build()
    //     .start();

    // Read workflow from file
    let file = "/Users/alexanderbrunner/CLionProjects/rust-task/sample_to_file2.yaml";
    Workflow::from_file(file).start();

    // Write workflow to file
    // WorkflowBuilder::new("test".to_string())
    //     .add_tasks(vec![task1, task2])
    //     .build()
    //     .to_file("/Users/alexanderbrunner/CLionProjects/rust-task/sample_to_file2.yaml");
}
