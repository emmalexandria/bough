<h1 align="center">bough</h1>


Bough (/baʊ/) is an overengineered project focusing on the creation of file tree diagrams. Its uses range from READMEs and project documentation to use in editor file trees (see [`bough.nvim`](https://github.com/emmalexandria/bough.nvim)).

## Project overview 
- `bough` - *the CLI, described by this README and [here](./bough/README.md).*
- [`bough-lib`](./bough-lib/README.md) - *the Rust crate which powers `bough` and `boughd`.*
- [`boughd`](./boughd/README.md) - *A file-watching daemon for use in applications like `bough.nvim`.*

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
                                              bough 0.1.0

bough is an overengineered CLI for creating file-tree diagrams in a variety of output formats and styles.

It supports:
• HTML, ANSI, raw text, and Markdown output.
• Varying icon styles
• Configuration files

Usage:  bough [options] [PATH]

• PATH : The root path of the tree.

Options:
┌─────┬──────────┬──────┬──────────────────────────────────────────────────────────────────────┐
│short│   long   │value │description                                                           │
├─────┼──────────┼──────┼──────────────────────────────────────────────────────────────────────┤
│ -d  │--depth   │DEPTH │The depth to build the file tree to. 0 will build as much as possible.│
│     │          │      │ Default: 0                                                           │
│ -f  │--format  │FORMAT│                                                                      │
│     │          │      │ Possible values: [html, text, ansi, markdown]                        │
│     │          │      │ Default: text                                                        │
│ -i  │--icons   │STYLE │                                                                      │
│     │          │      │ Possible values: [none, nerd, unicode, emoji]                        │
│     │          │      │ Default: none                                                        │
│ -o  │--out     │ PATH │Output to the given file.                                             │
│ -c  │--copy    │      │Copy the output to the system clipboard.                              │
│ -a  │--all     │      │Display hidden files in the output.                                   │
│ -r  │--root    │      │Show the root folder at the top of the tree.                          │
│ -h  │--help    │      │Print this help output.                                               │
│ -g  │--generate│      │Generate a configuration file in [PATH] respecting passed arguments.  │
│ -V  │--version │      │Print version                                                         │
└─────┴──────────┴──────┴──────────────────────────────────────────────────────────────────────┘

Examples:
1) Build an HTML tree outputting it to a file: bough ./src -o output.html -i nerd
This outputs an HTML tree based on ./src with Nerd Font icons to output.html

See also: bough-lib and boughd!

Made with ♥ by Emma Alexandria <emma.jellemabutler@gmail.com>
```

## Screenshots
