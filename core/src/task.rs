use cmd_lib::run_cmd;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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
            let run = match cfg!(target_os = "windows") {
                true => run_cmd!(cmd /C $cmd),
                false => run_cmd!(bash -c $cmd),
            };
            if run.is_err() {
                log::error!("Could not run task");
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

#[cfg(test)]
mod tests {
    use crate::task::{Task, TaskBuilder};

    #[test]
    fn it_creates_tasks() {
        let expected_task = Task {
            name: "sample task1".to_string(),
            cmds: vec!["python test.py".to_string(), "node test.js".to_string()],
        };
        let commands: Vec<String> = vec!["python test.py".to_string(), "node test.js".to_string()];

        let task1 = TaskBuilder::new("sample task1".to_string())
            .commands(commands)
            .build();
        let task2 = TaskBuilder::new("sample task1".to_string())
            .add_command("python test.py".to_string())
            .add_command("node test.js".to_string())
            .build();

        assert_eq!(task1, task2);
        assert_eq!(task1, expected_task);
        assert_eq!(task2, expected_task);
    }

    #[test]
    fn it_creates_tasks_with_empty_commands() {
        let expected_task = Task {
            name: "sample task1".to_string(),
            cmds: vec![],
        };
        let task = TaskBuilder::new("sample task1".to_string()).build();

        assert_eq!(task, expected_task);
    }
}
