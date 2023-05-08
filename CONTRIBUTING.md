# Contributing
This file describes how to contribute to ITex

## New templates
To make a new template make a folder where the name of the folder is the template name as this is what ITex uses.

An itex-info.toml is also required with a name and description:

```
name = "Default"
description = "The default template."
```

Once you are done create a pull request

## Development
This section will go over the structure and style of the project.

### Setup
First clone the project (or your fork).  

There is a justfile in the project root to help with development. 
Just is a command runner like make, but more modern and written in rust, you can use cargo to install just: cargo install just.
Here are some of the targets in the just file along with some info:
```shell
default: test # This is the default target that runs when you type just without an argument
test-all # This will run test with and without the updater feature
bereit # The german word for ready, this should be run before a pr. It runs cargo fmt, tests, and cargo clippy to make sure everything looks ok.
```

