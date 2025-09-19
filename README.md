<h1 align="center">bough</h1>


Bough (/baʊ/) is an overengineered tool for creating tree diagrams. Useful for READMEs and documentation.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Screenshots](#screenshots)


## Features
- ASCII, HTML, Markdown, or ANSI terminal output support.
- Emoji or Nerd Font based icons.
- Colours based on filetype (for ANSI and HTML output).
- Wrapping support if a fixed width is desired, including support for wrapping descriptions.
- Theming support
- Support for [project files](#project-files)

### Project files
This functionality is designed for projects which want to have a file tree with descriptions explaining the purpose of files and directories. 
Bough will look for a `.boughconf` file in the directory you point it to. This file follows a simple plain text format. Here's the 
`.boughconf` for the Borough repo itself. 



```
# Directories are indicated by a trailing slash. 
# We could also add a description for a file called 'src' by ommitting the trailing slash
# The first part of any line is the folder or file name, and then the description for that file comes after a space.
./src/ The source code for the borough binary.
./src/main.rs The entry point of borough

# Fill this out later

# If a --- delimeter is found, everything after it acts as default values for borough arguments like depth, hidden files, output type, and icon style.
---
# Possible values: ascii,html,ansi,markdown
output=txt
# Possible values: 0 >= n =< usize::MAX, 0 indicating usize::MAX
depth=0
# Possible values: none,emoji,nerd,unicode
icons=none



```

Running `borough gen` will create a new `.boroughconf` file listing all the files that `borough` finds. This respects the `a` argument 
and `.gitignore`.

Comment lines can be included with a `#`. 

## Installation
If you already have a Rust toolchain installed, you can run:

`cargo install bough`.

Otherwise, download a binary release or install using Homebrew:

`brew install bough`.

## Usage
```
Usage: bough [OPTIONS] [path] [COMMAND]

Commands:
  tui   Enter the TUI mode for more advanced editing
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [path]  [default: ./]

Options:
  -i, --icons <style>  The style of icons to use [default: none] [possible values: none, unicode, emoji, nerd]
  -d, --depth <depth>  The depth to build the tree to. 0 will build a tree as large as possible [default: 0]
  -a, --all            Show hidden files
  -r, --root           Show the root directory in the file tree
  -h, --help           Print help
  -V, --version        Print version
```

## Screenshots
