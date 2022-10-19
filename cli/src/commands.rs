use std::env;
use std::path::PathBuf;

use clap::ArgMatches;

use core::task::TaskBuilder;
use core::workflow::{Workflow, WorkflowBuilder};

pub fn create_workflow_file(args: &ArgMatches) {
    let cwd = env::current_dir()
        .unwrap_or(PathBuf::from("could not get cwd"))
        .to_str()
        .unwrap()
        .to_string();
    // TODO: Check if a Workfile.yaml exists in cwd
    // If so, create a Workfile2.yaml file, and so on
    let name = args.get_one::<String>("name").unwrap().to_string();
    let path = format!("{}/{}.yaml", &cwd, &name);

    WorkflowBuilder::new(name)
        .add_task(TaskBuilder::new("example_task".to_string())
            .commands(vec!["echo hello".to_string(), "echo world".to_string()])
            .build()
        )
        .build()
        .to_file(path.as_str());
    log::info!("Created workflow file");
}

pub fn run_workflow_files(args: &ArgMatches) {
    args.get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|f| {
            log::info!("Running workfile {:?}", f);
            Workflow::from_file(f).start(None)
        });
}
