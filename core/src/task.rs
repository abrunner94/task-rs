use std::{iter, slice};
use cmd_lib::run_cmd;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub cmds: Vec<String>,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }

    pub fn start(&mut self) {
        for command in self.cmds.iter() {
            let cmd = &command;
            // TODO: Add Windows powershell support
            if run_cmd!(bash -c $cmd).is_err() {
                println!("errored")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TaskBuilder {
    name: String,
    cmds: Vec<String>,
}

impl TaskBuilder {
    pub fn new(name: String) -> TaskBuilder {
        TaskBuilder { name, cmds: vec![] }
    }

    pub fn commands(mut self, commands: Vec<String>) -> TaskBuilder {
        self.cmds = commands;
        self
    }

    pub fn add_command(mut self, command: String) -> TaskBuilder {
        self.cmds.push(command);
        self
    }

    pub fn build(self) -> Task {
        Task {
            name: self.name,
            cmds: self.cmds,
        }
    }
}
