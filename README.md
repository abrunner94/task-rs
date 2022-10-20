<div align="center">
<h1>task-rs</h1>
<h3>Create and run your scripts as workflows using one CLI</h3>
</div>
<p align="center">
  <img src="https://github.com/abrunner94/rust-task/blob/main/assets/task-rs.png?raw=true" width="450" title="task-rs logo">
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
Workflows are composed of tasks containing one or more commands and are defined as YAML.

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