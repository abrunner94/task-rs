use clap::{arg, ArgMatches, Command};

use crate::{TaskBuilder, Workflow, WorkflowBuilder};

pub fn subcommand_create_workflow_file(args: &ArgMatches) {
    let name = args.get_one::<String>("name").unwrap().to_string();

    WorkflowBuilder::new(name)
        .add_task(TaskBuilder::new("example_task".to_string())
            .commands(vec!["echo hello".to_string(), "echo world".to_string()])
            .build()
        )
        .build()
        .to_file("/Users/alexanderbrunner/CLionProjects/rust-task/sample_to_file5.yaml");

    log::info!("Created workflow file");
}

pub fn subcommand_run_workflow_files(args: &ArgMatches) {
    let file = args.get_one::<String>("file").unwrap().to_string();
    log::info!("Running workfile {:?}", file);
}

