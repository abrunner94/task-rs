use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::task::{Task, TaskBuilder};

#[derive(Default, Serialize, Debug, Eq, PartialEq)]
pub struct Workflow {
    name: String,
    default: Vec<String>,
    tasks: Vec<Task>,
}

impl Workflow {
    pub fn builder() -> WorkflowBuilder {
        WorkflowBuilder::default()
    }

    pub fn start(mut self, task_name: Option<String>) {
        match task_name {
            None => {
                self.tasks.iter_mut().for_each(|t| t.start());
            }
            Some(_) => {
                let mut task_to_run: Vec<Task> = self
                    .tasks
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

    pub fn to_file(self, file_name: &str) -> Result<Workflow, Error> {
        let workflow = Workflow {
            name: self.name,
            default: self.default,
            tasks: self.tasks,
        };

        let yaml = match serde_yaml::to_value(&workflow) {
            Ok(yaml) => yaml,
            Err(_) => {
                let msg = format!("Could not convert yaml file: {}", &file_name);
                return Err(Error::new(ErrorKind::InvalidInput, msg));
            }
        };

        if workflow.tasks.is_empty() {
            log::info!("No tasks have been found. Skipping file creation.");
            return Ok(workflow);
        }

        let file = match OpenOptions::new().write(true).create(true).open(file_name) {
            Ok(file) => file,
            Err(e) => {
                let msg = format!("{} file cannot be written to", &file_name);
                return Err(Error::new(e.kind(), msg));
            }
        };

        match serde_yaml::to_writer(file, &yaml) {
            Ok(written) => written,
            Err(_) => {
                let msg = format!("{} could not write yaml to file", &file_name);
                return Err(Error::new(ErrorKind::Other, msg));
            }
        }

        Ok(workflow)
    }

    pub fn from_file(file_name: &str) -> Result<Workflow, Error> {
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(e) => {
                let msg = format!("{} file does not exist", &file_name);
                return Err(Error::new(e.kind(), msg));
            }
        };

        let yaml: Value = match serde_yaml::from_reader(file) {
            Ok(read) => read,
            Err(_) => {
                return Err(Error::new(ErrorKind::Other, "Could not read yaml file"));
            }
        };

        let default = &yaml["default"];
        let tasks = &yaml["tasks"];
        let workflow_name = &yaml["name"];
        let mut workflow_tasks: Vec<Task> = Vec::new();

        // Build up default task list
        let default_tasks: Vec<Value> = match default.as_sequence() {
            None => {
                log::warn!("No default tasks have been found");
                vec![].to_vec()
            }
            Some(defaults) => defaults.to_vec(),
        };

        let task_list: Vec<Value> = match tasks.as_sequence() {
            None => {
                log::warn!("No tasks have been found");
                vec![].to_vec()
            }
            Some(tasks) => tasks.to_vec(),
        };

        if task_list.is_empty() || default_tasks.is_empty() {
            log::warn!("The default task list and main task list have to be defined");
            return Ok(Workflow {
                name: "invalid".to_string(),
                tasks: vec![],
                default: vec![],
            });
        }

        for def in default_tasks.iter() {
            let default_task_name = def.as_str().unwrap().to_string();

            for task in task_list.iter() {
                let task_name = &task.as_mapping().unwrap()["name"]
                    .as_str()
                    .unwrap()
                    .to_string();

                // Check if the default task name matches a task name in the task list
                if task_name.eq(&default_task_name) {
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
            }
        }

        let workflow = WorkflowBuilder::new(workflow_name.as_str().unwrap().to_string())
            .add_tasks(workflow_tasks)
            .build();

        Ok(workflow)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WorkflowBuilder {
    name: String,
    default: Vec<String>,
    tasks: Vec<Task>,
}

impl WorkflowBuilder {
    pub fn new(name: String) -> WorkflowBuilder {
        WorkflowBuilder {
            name,
            default: vec![],
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

    pub fn add_defaults(mut self, task_names: Vec<String>) -> WorkflowBuilder {
        self.default = task_names;
        self
    }

    pub fn build(self) -> Workflow {
        Workflow {
            name: self.name,
            default: self.default,
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
            default: vec!["sample task1".to_string(), "sample task2".to_string()],
            tasks: vec![
                Task {
                    name: "sample task1".to_string(),
                    cmds: vec!["python test.py".to_string(), "node test.js".to_string()],
                },
                Task {
                    name: "sample task2".to_string(),
                    cmds: vec!["echo hello world".to_string()],
                },
            ],
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
            .add_defaults(vec![task1.name.clone(), task2.name.clone()])
            .add_task(task1)
            .add_task(task2)
            .build();

        assert_eq!(workflow, expected_workflow);
    }

    #[test]
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
            .add_defaults(vec![task1.name.clone(), task2.name.clone()])
            .add_tasks(vec![task1, task2])
            .build();

        assert_eq!(workflow, expected_workflow);
    }
}
