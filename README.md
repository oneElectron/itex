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
itex <options> template
  -i --info                 get template info
  -l --list                 output a list of templates
  -s --disable-os-search    prevent itex from searching the os for the templates folder
  -o --output <path>        output template to given folder <path>
  -u --update               update the itex-templates folder
```

available templates:
- default (just the basics with an out folder and a Makefile)
- iSci
- apa
- Electron (My own template)

## Install on MacOS
ITex isn't on homebrew's default taps so you need to add my tap then install ITex:
```
brew tap oneelectron/oneelectron
brew install itex -s
```

## Install on Windows
ITex doesn't have a windows installer yet, so the recommended way is to use cargo to install itex, then use itex to install it's template folder.
Install rust then:
```
cargo install itex
itex --update
```

## Making new templates
To make a new template make a folder where the name of the folder is the template name as this is what ITex uses.

A itex-info.json is also required with a name, id, and description:
```json
{
    "name": "Name here",
    "description": "Description here",
    "files": [
        "Place all file names here except for itex-info.json like so:",
        "main.tex", 
        "Makefile"
    ],
    "id": 0 // This will be filled in by a maintainer
}
```

On top of this a Makefile is required. The standard Makefile looks like this:
```Makefile
NAME = main

pdf:
	@pdflatex -no-shell-escape -output-directory ./out/ $(NAME).tex
	@if [ -a ./out/main.aux ]; then rm ./out/$(NAME).aux; fi;
	@if [ -a ./out/main.out ]; then rm ./out/$(NAME).out; fi;
	@if [ -a ./out/main.log ]; then rm ./out/$(NAME).log; fi;
	@if [ -a ./out/main.bcf ]; then rm ./out/$(NAME).bcf; fi;
	@if [ -a ./out/main.toc ]; then rm ./out/$(NAME).toc; fi;
	@if [ -a ./out/texput.log ]; then rm ./out/texput.log; fi;
	@if [ -a ./out/main.run.xml ]; then rm ./out/main.run.xml; fi;
	
count:
	@texcount $(NAME).tex

clean: 
	@if [ -a ./out/main.aux ]; then rm ./out/$(NAME).aux; fi;
	@if [ -a ./out/main.out ]; then rm ./out/$(NAME).out; fi;
	@if [ -a ./out/main.log ]; then rm ./out/$(NAME).log; fi;
	@if [ -a ./out/main.bcf ]; then rm ./out/$(NAME).bcf; fi;
	@if [ -a ./out/main.toc ]; then rm ./out/$(NAME).toc; fi;
	@if [ -a ./out/texput.log ]; then rm ./out/texput.log; fi;
	@if [ -a ./out/main.run.xml ]; then rm ./out/main.run.xml; fi;
	
debug:
	pdflatex -no-shell-escape -output-directory ./out/ main.tex

```

## Development
To setup the development environment:
- Clone the repo
- Run cargo build in order to install dependencies
