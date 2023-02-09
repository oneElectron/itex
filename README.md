<div align="center">

# ITex 

![version](https://img.shields.io/github/v/tag/oneelectron/itex?color=orange)
![licence](https://img.shields.io/github/license/oneelectron/itex?color=blue)
![lines of code](https://img.shields.io/tokei/lines/github/oneelectron/itex?color=green)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/oneElectron/itex?color=orange)

</div>

Initialize a latex project in the current folder

Usage:
```
itex <options> template

  -d --debug                use debug mode
  -l --list                 output a list of templates
  -s --disable-os-search    prevent itex from searching the os for the templates folder
```

available templates:
- default (just the basics with an out folder and a Makefile)
- iSci
- tmlr

## Install on MacOS
itex isn't on homebrew's default taps so you need to add my tap then install itex:
```
brew tap oneelectron/oneelectron
brew install itex -s
```
