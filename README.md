# task-rs
task-rs is a simple file-based task runner that executes scripts defined in a YAML file.
Create and run your scripts as workflows using one CLI.

<p align="left">
  <img src="https://github.com/abrunner94/rust-task/blob/main/assets/task-rs.png?raw=true" height="350" title="task-rs logo">
</p>

## Installation
### macOS
Download the Apple Darwin archive from the [releases section](https://github.com/abrunner94/task-rs/releases)
### Linux
Download the Linux archive from the [releases section](https://github.com/abrunner94/task-rs/releases)
### Windows
Download the Windows zip file from the [releases section](https://github.com/abrunner94/task-rs/releases)

## Usage
Workflows are composed of tasks containing one or more commands and are defined in YAML files. 

### Create a workflow
The following command creates a skeleton YAML file named `my_workflow.yaml` in your current working directory that you can modify to fit your needs.
```bash
task create -n my_workflow
```

### Run the whole workflow
The following command runs a single workflow file.
```bash
task run -f workfile.yaml 
```

### Run multiple workflows
The following command runs multiple workflow files and its contents in that order, all at once.
```bash
task run -f workfile1.yaml,workfile2.yaml,workfile3.yaml
```

## Definitions
- `name` is the name of your workflow
- `default` is a list of task names that runs the tasks defined in the `tasks` section in the order it is defined
- `tasks` is a list of task names along with a list of commands / scripts to run

#### Example workflow file
```yaml
name: my_workflow
default:
  - sample_task4
  - sample_task3
  - sample_task2
  - sample_task1
tasks:
- name: sample_task1
  cmds:
  - echo "hello from task 1"
  - python test.py
  - node test.js
- name: sample_task2
  cmds:
  - echo "hello from task 2"
- name: sample_task3
  cmds:
    - echo "hello from task 3"
- name: sample_task4
  cmds:
    - echo "hello from task 4"
```
