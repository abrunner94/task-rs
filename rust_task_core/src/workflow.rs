use std::borrow::Borrow;
use std::fs::File;
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use crate::{ run_cmd, Task, TaskBuilder, TaskCommand};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Workflow {
    name: String,
    tasks: Vec<Task>
}

impl Workflow {
    pub fn builder() -> WorkflowBuilder { WorkflowBuilder::default() }

    pub fn start(mut self) {
        for task in self.tasks.iter_mut() {
            task.start();
        }
    }

    pub fn to_file(mut self, file_name: &str) -> Workflow {
        let workflow = Workflow {
            name: self.name,
            tasks: self.tasks
        };

        let yaml = serde_yaml::to_string(&workflow)
            .expect("could not convert struct to string");

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .expect("could not open file");
        serde_yaml::to_writer(f, &yaml).expect("could not write yaml file");

        workflow
    }

    pub fn from_file(file_name: &str) -> Workflow {
        let file = File::open(file_name)
            .expect("could not open file");
        let yaml: serde_yaml::Value = serde_yaml::from_reader(file)
            .expect("could not read yaml file");

        println!("{:?}", yaml);

        let tasks = &yaml["tasks"];
        let workflow_name = &yaml["name"];

        let mut workflow_tasks: Vec<Task> = Vec::new();

        for (task, command_mapping) in tasks.as_mapping().unwrap().into_iter() {
            let commands = command_mapping["cmds"].as_sequence().unwrap();
            let commands_as_str: Vec<String> = commands
                .into_iter()
                .map(|c|  String::from(c.as_str().unwrap()))
                .collect();

            workflow_tasks.push(
            TaskBuilder::new(String::from(task.as_str().unwrap()))
                    .commands_from_string_vec(commands_as_str)
                    .build()
            )
        }

        WorkflowBuilder::new(String::from(workflow_name.as_str().unwrap()))
            .add_tasks(workflow_tasks)
            .build()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WorkflowBuilder {
    name: String,
    tasks: Vec<Task>
}

impl WorkflowBuilder {
    pub fn new(name: String) -> WorkflowBuilder {
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

