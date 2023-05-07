<div align="center">

# ITex 

![version](https://img.shields.io/github/v/tag/oneelectron/itex?color=orange)
![license](https://img.shields.io/github/license/oneelectron/itex?color=blue)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/oneelectron/itex?color=red)
[![Ubuntu-latest](https://github.com/oneElectron/itex/actions/workflows/ubuntu-latest.yml/badge.svg)](https://github.com/oneElectron/itex/actions/workflows/ubuntu-latest.yml)
[![codecov](https://codecov.io/gh/oneElectron/itex/branch/main/graph/badge.svg?token=HU8FPL07Y7)](https://codecov.io/gh/oneElectron/itex)

</div>

Initialize a latex project in the current folder

Usage:
```
Usage: itex <COMMAND>

Commands:
  build          Build ITex project (requires an itex-build.toml file, and pdflatex to be installed)
  count          Count the number of words in the current ITex project (requires texcount to be installed)
  clean          Clean auxillary build files
  init           Initialize LaTex project
  info           Get info about a template
  get            Get current value of a setting
  list           List installed templates
  new-buildfile  Create a new itex build file
  set            Set a setting
  update         Update installed templates
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

available templates:
- default (just the basics with an out folder and a Makefile)
- iSci
- apa
- Electron (My own template)

# Getting started
The first step is to install itex, see install on windows or macos to install on your platform of choice.  
To copy a template use: 
```
itex init <template name>
```
To get a list of template names use:
```
itex list
```
after copying a template use:
```
itex build
```
to build the project.  
If there is an error in your project use:
```
itex build --debug
```
to get debug output.

## Options
ITex gives you some options to control how the project is built.
Use:
```
itex get
```
To get the full list of options.  
By default only the default_filename is set.
If you changed your tex filename from main.tex to something else, you are also going to want to change this to the new name of the file. 
```
itex set default_filename <Insert filename here (without the .tex)>
```

A description of all the options can be found in the wiki here: https://github.com/oneelectron/itex/wiki/ITex-settings

# Install on MacOS
ITex isn't on homebrew's default taps so you need to add my tap then install ITex:
```
brew tap oneelectron/oneelectron
brew install itex -s
```

# Install on Windows
ITex doesn't have a windows installer yet, so the recommended way is to use cargo to install itex, then use itex to install it's template folder.
Install rust then:
```
cargo install itex
itex --update
```

# Install on Linux
Installing on linux is the same as on windows because there no package managers that have ITex on their lists. With more users and support this could change, but for now:
```
cargo install itex
itex --update
```


# ITex build system
Users can run itex build in order to build their LaTex project.
Doing this requires the ITex to know which file to compile, this is specified in the itex-build.toml which looks something like this:
```toml
default_filename = "main"
tex_filename = "example.tex"

```
- default_filename specifies the base name for any files
- tex_filename specifies the name of the main tex file. If this option is not specified itex will assume the default_filename + .tex. In this example main.tex, if tex_filename were not specified


# Making new templates
To make a new template make a folder where the name of the folder is the template name as this is what ITex uses.

An itex-info.toml is also required with a name and description:
```toml
name = "Default"
description = "The default template."
```


# Development
To setup the development environment:
- Clone the repo
- Run cargo build in order to install dependencies
