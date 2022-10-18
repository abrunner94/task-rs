use clap::{Arg, command, Command};
use simple_logger::SimpleLogger;

use crate::commands::{create_workflow_file, run_workflow_files};

mod examples;
mod commands;

fn main() {
    SimpleLogger::new().init().unwrap();

    let main_command = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Creates a new workfile")
                .arg(Arg::new("name")
                    .help("a required name for your new workfile")
                    .short('n')
                    .long("name")
                    .ignore_case(true)
                )
        )
        .subcommand(
            Command::new("run")
                .about("Runs a workfile")
                .arg(
                    Arg::new("files")
                        .help("a list of workfiles to run")
                        .short('f')
                        .long("files")
                        .ignore_case(true)
                        .value_delimiter(',')
                ),
        )
        .get_matches();

    match main_command.subcommand() {
        Some(("create", sub_matches)) => create_workflow_file(sub_matches),
        Some(("run", sub_matches)) => run_workflow_files(sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
