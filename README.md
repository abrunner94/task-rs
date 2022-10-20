# task-rs
task-rs is a simple file-based task runner that executes scripts defined in a YAML file.
That way, you can create and run your scripts as workflows using one CLI.

<p align="left">
  <img src="https://github.com/abrunner94/rust-task/blob/main/assets/task-rs.png?raw=true" height="350" title="task-rs logo">
</p>

## Installation
Follow the instructions for your operating system.
### macOS
TODO
### Linux
TODO
### Windows
TODO

## Usage
Workflows are composed of tasks containing one or more commands and are defined in YAML files.

### Create a workflow
```bash
task create -n my_workflow
```

### Run the whole workflow
```bash
task run -f workfile.yaml 
```

### Run multiple workflow files
```bash
task run -f workfile1.yaml,workfile2.yaml,workfile3.yaml
```
