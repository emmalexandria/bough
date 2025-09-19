<h1 align="center">bough</h1>


Bough (/baʊ/) is an overengineered CLI tool for creating file tree diagrams. Useful for READMEs and documentation.

The Bough project provides a crate called `bough-lib` which is used by the CLI for drawing file trees. This crate is designed to be usable outside of the CLI by any application which wants to draw a file tree.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Screenshots](#screenshots)

## Features
- ASCII, HTML, Markdown, or ANSI terminal output support.
- Emoji, Non-emoji unicode, or Nerd Font based icons.
- Colours based on filetype (for ANSI and HTML output).
- Wrapping support if a fixed width is desired.
- Theming support
- Support for [project files](#project-files)

### Project Files
Bough has support for a plain-text configuration format. Bough will look for a `.boughconf` file in the directory you point it to, or one can be supplied with the `-c` flag. The primary purpose of a `.boughconf` is to add descriptions to files in your project, but it can also be used to add default values for `bough` arguments (e.g. depth, theme, icon style). 

Please note that `.boughconf` files are only read from the directory passed into `bough`. If you run `bough` on the root of your repo, `<root>/.boughconf` will be used exclusively. If you pass in `<root>/src`, only `<root>/src/.boughconf` will be read. While limiting in some ways, 
this is intentional for the sake of simplicity.

 Here is the `.boughconf` for the Bough repo itself. 

*(Comments begin with #)*

```
# Directories are indicated by a trailing slash. 
# We could also add a description for a file called 'src' by ommitting the trailing slash
# The first part of any line is the folder or file name, and then the description for that file comes after a space.
bough/ The bough binary project.
bough-lib/ The bough-lib project.
Cargo.toml The workspace Cargo.toml file


# Fill this out later

# If a --- delimeter is found, everything after it acts as default values for bough arguments like depth, hidden files, output type, and icon style.
---
# Possible values: ascii,html,ansi,markdown
output=txt
# Possible values: 0 >= n =< usize::MAX, 0 indicating usize::MAX
depth=0
# Possible values: none,emoji,nerd,unicode
icons=none

# Below are options that are unused in the project's bough.conf

# Possible values: any .toml theme file
# theme=<YOURTHEME>.toml

# Possible values: true, false
# hidden=false
```

Running `bough gen` will create a new `.boughconf` file listing all the files that `bough` finds. This respects the `a` argument 
and `.gitignore`.


## Installation
If you already have a Rust toolchain installed, you can run:

`cargo install bough`.

Otherwise, download a binary release or install using Homebrew:

`brew install bough`.

## Usage
```
Usage: bough [OPTIONS] [path] [COMMAND]

Commands:
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
