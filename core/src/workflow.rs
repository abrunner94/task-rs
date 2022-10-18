use std::fs::File;
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use crate::task::{Task, TaskBuilder};

#[derive(Default, Serialize, Debug, Eq, PartialEq)]
pub struct Workflow {
    name: String,
    tasks: Vec<Task>,
}

impl Workflow {
    pub fn builder() -> WorkflowBuilder {
        WorkflowBuilder::default()
    }

    pub fn start(mut self, task_name: Option<String>) {
        match task_name {
            None => { self.tasks.iter_mut().for_each(|t| t.start()); }
            Some(_) => {
                let mut task_to_run: Vec<Task> = self.tasks
                    .into_iter()
                    .filter(|t| t.name.eq(task_name.as_ref().unwrap().as_str()))
                    .collect();

                if task_to_run.is_empty() {
                    log::info!("No runnable tasks have been found. Skipping operation.");
                }

                task_to_run.iter_mut().for_each(|t| t.start());
            }
        }
    }

    pub fn to_file(self, file_name: &str) -> Workflow {
        let workflow = Workflow { name: self.name, tasks: self.tasks };
        let yaml = serde_yaml::to_value(&workflow).expect("could not convert struct to string");

        if workflow.tasks.is_empty() {
            log::info!("No tasks have been found. Skipping file operation.");
            return workflow
        }

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .expect("could not open file");
        serde_yaml::to_writer(f, &yaml).expect("could not write yaml file");

        workflow
    }

    pub fn from_file(file_name: &str) -> Workflow {
        // TODO: Maybe we should make it so that when the CLI runs, it will look
        // at the current working directory of the CLI instead of providing the full path
        // to the file

        let file = File::open(file_name).expect("could not open file");
        let yaml: serde_yaml::Value =
            serde_yaml::from_reader(file).expect("could not read yaml file");

        let tasks = &yaml["tasks"];
        let workflow_name = &yaml["name"];
        let mut workflow_tasks: Vec<Task> = Vec::new();

        for task in tasks.as_sequence().unwrap().iter() {
            let commands = task["cmds"].as_sequence().unwrap();

            let commands_as_str: Vec<String> = commands
                .iter()
                .map(|c| {
                    let value = c.as_str();
                    String::from(value.unwrap())
                })
                .collect();

            workflow_tasks.push(
                TaskBuilder::new(String::from(task["name"].as_str().unwrap()))
                    .commands(commands_as_str)
                    .build(),
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
    tasks: Vec<Task>,
}

impl WorkflowBuilder {
    pub fn new(name: String) -> WorkflowBuilder {
        WorkflowBuilder {
            name,
            tasks: vec![],
        }
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
        Workflow {
            name: self.name,
            tasks: self.tasks,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::task::{Task, TaskBuilder};
    use crate::workflow::{Workflow, WorkflowBuilder};

    fn get_expected_workflow() -> Workflow {
        Workflow {
            name: "my workflow".to_string(),
            tasks: vec![
                Task {
                    name: "sample task1".to_string(),
                    cmds: vec![
                        "python test.py".to_string(),
                        "node test.js".to_string()
                    ]
                },
                Task {
                    name: "sample task2".to_string(),
                    cmds: vec!["echo hello world".to_string()]
                }
            ]
        }
    }

    #[test]
    fn it_creates_workflow_using_add_task() {
        let expected_workflow = get_expected_workflow();
        let task1: Task = TaskBuilder::new("sample task1".to_string())
            .add_command("python test.py".to_string())
            .add_command("node test.js".to_string())
            .build();
        let task2: Task = TaskBuilder::new("sample task2".to_string())
            .add_command("echo hello world".to_string())
            .build();

        let workflow = WorkflowBuilder::new("my workflow".to_string())
            .add_task(task1)
            .add_task(task2)
            .build();

        assert_eq!(workflow, expected_workflow);
    }

    fn it_creates_workflow_using_add_tasks() {
        let expected_workflow = get_expected_workflow();
        let task1: Task = TaskBuilder::new("sample task1".to_string())
            .add_command("python test.py".to_string())
            .add_command("node test.js".to_string())
            .build();
        let task2: Task = TaskBuilder::new("sample task2".to_string())
            .add_command("echo hello world".to_string())
            .build();
        let workflow = WorkflowBuilder::new("my workflow".to_string())
            .add_tasks(vec![task1, task2])
            .build();

        assert_eq!(workflow, expected_workflow);
    }
}