use std::borrow::{Borrow, BorrowMut};
use std::hash::Hash;
use serde::__private::de::Borrowed;
use yaml_rust::{Yaml, YamlLoader};
use crate::{load_task_file, run_cmd, Task, TaskBuilder, TaskCommand};

#[derive(Default, Debug)]
pub struct Workflow {
    name: &'static str,
    tasks: Vec<Task>
}

impl Workflow {
    pub fn builder() -> WorkflowBuilder { WorkflowBuilder::default() }

    pub fn start(&mut self) {
        for task in self.tasks.iter_mut() {
            task.start();
        }
    }

    pub fn from_file(file_name: String, workflow_name: &'_ str) -> Workflow {
        // let task_file = load_task_file(file_name.as_str()).clone();
        // let mut task_hash = &task_file[0]["tasks"].as_hash().unwrap();
        // let mut w_tasks: Vec<Task> = vec![];
        // let workflow_builder: WorkflowBuilder = WorkflowBuilder::new("bla");
        //
        // for (task_name, commands) in task_hash.iter() {
        //     let command_list = commands["cmds"].as_vec().unwrap().to_vec();
        //     for command in command_list.iter() {
        //         let cmd = snailquote::unescape(command.as_str().unwrap()).unwrap();
        //         w_tasks.push(
        //             TaskBuilder::new(task_name.as_str().unwrap())
        //                 .add_command(cmd.as_str())
        //                 .build()
        //         )
        //     }
        // }

        Workflow {
            tasks: vec![],
            name: "t"
        }
    }

    fn load_task_file(file_name: String) -> Vec<Yaml> {
        let mut file = std::fs::read_to_string(file_name).unwrap();
        return YamlLoader::load_from_str(&file).unwrap();
    }
}

#[derive(Default, Debug)]
pub struct WorkflowBuilder {
    name: &'static str,
    tasks: Vec<Task>
}

impl WorkflowBuilder {
    pub fn new(name: &'static str) -> WorkflowBuilder {
        WorkflowBuilder { name, tasks: vec![] }
    }

    pub fn add_task(mut self, task: Task) -> WorkflowBuilder {
        self.tasks.push(task);
        self
    }

    pub fn add_tasks(mut self, tasks: Vec<Task>) -> WorkflowBuilder {
        self.tasks = tasks;
        self
    }

    pub fn build(self) -> Workflow {
        Workflow { name: self.name, tasks: self.tasks }
    }
}

