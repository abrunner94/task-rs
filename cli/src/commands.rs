use clap::{arg, Command};
use crate::{TaskBuilder, Workflow, WorkflowBuilder};

pub fn create_workflow_file(command: &Command) {
    let create_sub = command.to_owned()
        .subcommand(
            Command::new("create")
                .arg(arg!(-n --name <NAME> "a required name for your workfile")),
        )
        .get_matches();

    match create_sub.subcommand() {
        Some(("create", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap().to_string();

            WorkflowBuilder::new(name)
                .add_task(TaskBuilder::new("example_task".to_string())
                    .commands(vec!["echo hello".to_string(), "echo world".to_string()])
                    .build()
                )
                .build()
                .to_file("/Users/alexanderbrunner/CLionProjects/rust-task/sample_to_file5.yaml");

            log::info!("Created workflow file");
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

pub fn run_workflow_files(command: &Command) {
    let run_sub = command.to_owned()
        .subcommand(
            Command::new("run")
                .arg(arg!(-f --file <FILE_NAME> "a required file path of your workfile")),
        )
        .arg(arg!(--tasks [TASK_NAMES]))
        .get_matches();

    match run_sub.subcommand() {
        Some(("run", sub_matches)) => {
            let file_name = sub_matches.get_one::<String>("FILE_NAME").unwrap().to_string();
            let task_names = sub_matches.get_many::<String>("TASK_NAMES");

            println!("{:?}", task_names.unwrap());

            // TODO: Check if there are task names passed in
            Workflow::from_file(file_name.as_str())
                .start(None);

            log::info!("Created workflow file");
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

