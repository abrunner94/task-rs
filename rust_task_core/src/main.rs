use crate::task::{Task, TaskBuilder};
use crate::workflow::{Workflow, WorkflowBuilder};

mod task;
mod workflow;

fn main() {
    // Create commands
    let commands1: Vec<String> = vec![
        "python test.py".to_string(),
        "node test.js".to_string(),
    ];

    let commands2: Vec<String> = vec!["echo \"hello\"".to_string()];

    // Build up your tasks using commands
    let task1: Task = TaskBuilder::new("sample task1".to_string())
        .commands(commands1)
        .build();
    let task2: Task = TaskBuilder::new("sample task2".to_string())
        .commands(commands2)
        .build();

    // Create workflow with tasks and start it
    // WorkflowBuilder::new("my workflow".to_string())
    //     .add_task(task1)
    //     .add_task(task2)
    //     .build()
    //     .to_file("/Users/abrunner/CLionProjects/rust-task/sample_to_file5.yaml")
    //     .start(None);

    // Read workflow from file
    let file = "/Users/abrunner/CLionProjects/rust-task/sample_to_file5.yaml";
    Workflow::from_file(file).start(None);

    // Write workflow to file
    // WorkflowBuilder::new("test".to_string())
    //     .add_tasks(vec![task1, task2])
    //     .build()
    //     .to_file("/Users/alexanderbrunner/CLionProjects/rust-task/sample_to_file4.yaml");
}
