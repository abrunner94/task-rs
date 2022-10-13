use std::any::Any;
use std::ffi::OsString;
use std::io::{Read, Write, Result};
use std::path::Path;
use cmd_lib::{AsOsStr, Cmd, CmdEnv, CmdResult, Cmds, CmdString, FunResult, GroupCmds, init_builtin_logger, run_fun, use_custom_cmd};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use serde_yaml::Value;
use syn::__private::TokenStream2;
use syn::parse2;
use syn::parse::Nothing;
use crate::run_cmd;

#[derive(Debug, Default, Clone)]
pub struct TaskCommand {
    command: String
}

impl TaskCommand {
    pub fn new(command: String) -> TaskCommand {
        TaskCommand { command }
    }
}

#[derive(Debug, Default)]
pub struct Task {
    name: String,
    commands: Vec<TaskCommand>,
    new_commands: Vec<TaskCommand>,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }

    pub fn start(&mut self) {
        for command in self.new_commands.iter() {
            let cmd = &command.command;
            if run_cmd!(bash -c $cmd).is_err() {
                println!("errored")
            }
        }
    }
}

#[derive(Default)]
pub struct TaskBuilder {
    name: String,
    commands: Vec<TaskCommand>,
    new_commands: Vec<TaskCommand>,
}

impl TaskBuilder {
    pub fn new(name: String) -> TaskBuilder {
        TaskBuilder { name, commands: vec![], new_commands: vec![] }
    }

    pub fn commands(mut self, commands: Vec<TaskCommand>) -> TaskBuilder {
        self.new_commands = commands;
        self
    }

    pub fn commands_from_string_vec(mut self, commands: Vec<String>) -> TaskBuilder {
        let mut cmds = commands
            .into_iter()
            .map(|c| TaskCommand::new(c))
            .collect();

        println!("{:?}", cmds);

        self.new_commands = cmds;
        self
    }

    pub fn add_command(mut self, command: String) -> TaskBuilder {
        let cmd = TaskCommand::new(command);
        self.commands.push(cmd);
        self
    }

    pub fn build(self) -> Task {
        Task {
            name: self.name,
            commands: self.commands,
            new_commands: self.new_commands
        }
    }
}

//
// pub fn parse_and_run_tasks() {
//     init_builtin_logger();
//
//     let task_file = load_task_file("/Users/abrunner/CLionProjects/rust-task/sample2.yaml");
//     let tasks =  &task_file[0]["tasks"].as_hash().unwrap();
//
//     for (task_name, commands) in tasks.iter() {
//         let command_list = commands["cmds"].as_vec().unwrap();
//         for command in command_list.iter() {
//             let cmd = snailquote::unescape(command.as_str().unwrap()).unwrap();
//             if run_cmd!(bash -c "$cmd").is_err() {
//                 println!("errored")
//             }
//         }
//     }
// }
