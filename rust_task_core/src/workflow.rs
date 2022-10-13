use std::borrow::Borrow;
use std::hash::Hash;
use crate::{ run_cmd, Task, TaskBuilder, TaskCommand};

#[derive(Default, Debug)]
pub struct Workflow {
    name: String,
    tasks: Vec<Task>
}

impl Workflow {
    pub fn builder() -> WorkflowBuilder { WorkflowBuilder::default() }

    pub fn start(&mut self) {
        for task in self.tasks.iter_mut() {
            task.start();
        }
    }

    pub fn from_file(file_name: &str) -> Workflow {
        let file = std::fs::File::open(file_name).expect("could not open file");
        let yaml: serde_yaml::Value = serde_yaml::from_reader(file).expect("could not read yaml file");

        let tasks = &yaml["tasks"];
        let workflow_name = &yaml["name"];

        let mut workflow_tasks: Vec<Task> = Vec::new();

        for (task, command_mapping) in tasks.as_mapping().cloned().unwrap().into_iter() {
            println!("{:?}", task);

            let commands = command_mapping["cmds"].as_sequence().unwrap().to_vec();
            let commands_as_str: Vec<String> = commands.into_iter().map(|c|  String::from(c.as_str().unwrap())).collect();

            // println!("{:?}, {:?}", task_name.as_str().unwrap(), commands_as_str);

            workflow_tasks.push(
            TaskBuilder::new("bla".to_string())
                .commands_from_string_vec(commands_as_str)
                .build()
            )

        }

        // Workflow {
        //     name: String::from("bla"),
        //     tasks: vec![]
        // }
        let workflow = WorkflowBuilder::new(String::from(workflow_name.as_str().unwrap()))
            .add_tasks(workflow_tasks)
            .build();

        workflow
    }
}

#[derive(Default, Debug)]
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

