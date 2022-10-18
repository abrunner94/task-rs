# ⚙️ task.rs 🦀
Create and run your scripts as workflows using one CLI.

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
task create --file workfile.yaml
```

### Run the whole workflow
```bash
task run workfile.yaml 
```

### Run specific tasks in the workflow file
```bash
task run workfile.yaml --task mytask
```

### Run multiple workflow files
```bash
task run --w workfile1.yaml workfile2.yaml workfile3.yaml
```