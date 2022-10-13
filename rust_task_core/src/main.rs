use cmd_lib::{init_builtin_logger, run_cmd, run_fun};
use crate::task::{TaskBuilder, TaskCommand, Task};
use crate::workflow::{Workflow, WorkflowBuilder};

mod task;
mod workflow;

fn main() {
    // Create commands
    let mut commands: Vec<TaskCommand> = vec![
        TaskCommand::new(String::from("python test.py")),
        TaskCommand::new(String::from("echo \"hello\"")),
        TaskCommand::new(String::from("node bla.js")),
    ];

    // Create tasks
    let task1: Task = TaskBuilder::new("sample task1".to_string())
        .commands(commands.to_vec())
        .build();
    let task2: Task = TaskBuilder::new("sample task2".to_string())
        .commands(commands.to_vec())
        .build();

    // Create workflows
    // WorkflowBuilder::new("my workflow")
    //     .add_task(task1)
    //     .add_task(task2)
    //     .build()
    //     .start();

    // Read workflow from file
    let file = "/Users/alexanderbrunner/CLionProjects/rust-task/sample_to_file.yaml";
    Workflow::from_file(file).start();

    // // Write workflow to file
    // WorkflowBuilder::new("test".to_string())
    //     .add_tasks(vec![task1, task2])
    //     .build()
    //     .to_file("/Users/alexanderbrunner/CLionProjects/rust-task/sample_to_file.yaml");
}
