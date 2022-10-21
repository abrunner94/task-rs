use std::env;
use std::path::PathBuf;

use clap::ArgMatches;

use core::task::TaskBuilder;
use core::workflow::{Workflow, WorkflowBuilder};

pub fn create_workflow_file(args: &ArgMatches) {
    let cwd = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("could not get cwd"))
        .to_str()
        .unwrap()
        .to_string();

    let name = args.get_one::<String>("name").unwrap().to_string();
    let path = format!("{}/{}.yaml", &cwd, &name);

    let workflow = WorkflowBuilder::new(name)
        .add_task(
            TaskBuilder::new("example_task".to_string())
                .commands(vec!["echo hello".to_string(), "echo world".to_string()])
                .build(),
        )
        .add_defaults(vec!["example_task".to_string()])
        .build();

    match workflow.to_file(path.as_str()) {
        Ok(workflow) => {
            log::info!("Created workflow file");
            workflow.start(None);
        }
        Err(err) => {
            let msg = "Could not create workflow".to_string();
            log::error!("{}: {}", &msg, &err);
        }
    };
}

pub fn run_workflow_files(args: &ArgMatches) {
    args.get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|f| {
            log::info!("Running workfile {}", f);
            let workflow = Workflow::from_file(f);
            match workflow {
                Ok(workflow) => workflow.start(None),
                Err(err) => {
                    let msg = "Could not start workflow".to_string();
                    log::error!("{}: {}", &msg, &err);
                }
            }
        });
}
