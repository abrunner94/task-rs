mod examples;
mod commands;

use clap::{command, arg, Command};
use simple_logger::SimpleLogger;

use core::task::{Task, TaskBuilder};
use core::workflow::{Workflow, WorkflowBuilder};
use crate::commands::{create_workflow_file, run_workflow_files};

fn main() {
    SimpleLogger::new().init().unwrap();

    let main_command = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true);

    // Create a workflow file
    create_workflow_file(&main_command);

    // Run the workflow
    run_workflow_files(&main_command);

}
