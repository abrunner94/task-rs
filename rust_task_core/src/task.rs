use serde::{Deserialize, Serialize};

use crate::run_cmd;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TaskCommand {
    command: String,
}

impl TaskCommand {
    pub fn new(command: String) -> TaskCommand {
        TaskCommand { command }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Task {
    name: String,
    cmds: Vec<TaskCommand>,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }

    pub fn start(&mut self) {
        for command in self.cmds.iter() {
            let cmd = &command.command;
            if run_cmd!(bash -c $cmd).is_err() {
                println!("errored")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TaskBuilder {
    name: String,
    cmds: Vec<TaskCommand>,
}

impl TaskBuilder {
    pub fn new(name: String) -> TaskBuilder {
        TaskBuilder { name, cmds: vec![] }
    }

    pub fn commands(mut self, commands: Vec<TaskCommand>) -> TaskBuilder {
        self.cmds = commands;
        self
    }

    pub fn commands_from_string_vec(mut self, commands: Vec<String>) -> TaskBuilder {
        let cmds = commands.into_iter().map(TaskCommand::new).collect();

        self.cmds = cmds;
        self
    }

    pub fn add_command(mut self, command: String) -> TaskBuilder {
        let cmd = TaskCommand::new(command);
        self.cmds.push(cmd);
        self
    }

    pub fn build(self) -> Task {
        Task {
            name: self.name,
            cmds: self.cmds,
        }
    }
}
