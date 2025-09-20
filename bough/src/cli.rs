use std::{fmt::Display, path::PathBuf};

use bough_lib::icons::IconType;
use clap::{command, ArgMatches, Command, CommandFactory, Parser, ValueEnum};
use clap_help::Printer;
use crossterm::style::Color;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconArg {
    None,
    Nerd,
    Unicode,
    Emoji,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    HTML,
    Text,
    ANSI,
    Markdown,
}

impl Display for IconArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "none",
            Self::Nerd => "nerd",
            Self::Unicode => "unicode",
            Self::Emoji => "emoji",
        })
    }
}

impl Into<IconType> for IconArg {
    fn into(self) -> IconType {
        match self {
            Self::None => IconType::None,
            Self::Nerd => IconType::Nerd,
            Self::Unicode => IconType::Unicode,
            Self::Emoji => IconType::Emoji,
        }
    }
}

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash)]
#[command(author, version, about, disable_help_flag = true)]
pub struct Args {
    #[arg(
        long,
        short,
        default_value_t = 0,
        help = "The depth to build the file tree to. 0 will build as much as possible."
    )]
    pub depth: usize,
    #[arg(long, short, default_value = "text")]
    pub format: OutputFormat,
    #[arg(long, short, default_value_t = IconArg::None, value_name="STYLE")]
    pub icons: IconArg,
    #[arg(long, short, value_name = "PATH", help = "Output to the given file.")]
    pub out: Option<PathBuf>,
    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Copy the output to the system clipboard."
    )]
    pub copy: bool,
    #[arg(default_value = "./", help = "The root path of the tree.")]
    pub path: PathBuf,
    #[arg(long = "all", short, help = "Display hidden files in the output.")]
    pub all_files: bool,
    #[arg(long, short, help = "Show the root folder at the top of the tree.")]
    pub root: bool,
    #[arg(long, short, help = "Print this help output.")]
    pub help: bool,
    #[arg(
        long,
        short,
        help = "Generate a configuration file in [PATH] respecting passed arguments."
    )]
    pub generate: bool,
}

pub fn print_help_if_needed(args: Args) -> bool {
    if args.help {
        build_help_printer(Args::command()).print_help();
        return true;
    }

    false
}

static INTRODUCTION: &str = "
**bough** is an overengineered CLI for creating file-tree diagrams in a variety of output formats and styles.

It supports:
* HTML, ANSI, raw text, and Markdown output.
* Varying icon styles
* Configuration files
";

static AFTER_HELP: &str = "
See also: `bough-lib` and `boughd`!
";

static AUTHOR: &str = "*Made with ♥ by ${author}";

static EXAMPLE_TEMPLATE: &str = "
**Examples:**
${examples
**${example-number})** ${example-title}: `${example-cmd}`
${example-comments}
}
";

struct Example<'a> {
    pub title: &'a str,
    pub cmd: &'a str,
    pub comments: &'a str,
}

static EXAMPLES: [Example<'static>; 1] = [Example {
    title: "Build an HTML tree outputting it to a file",
    cmd: "bough ./src -o output.html -i nerd",
    comments: "This outputs an HTML tree based on `./src` with Nerd Font icons to `output.html`",
}];

fn build_help_printer(cmd: Command) -> Printer<'static> {
    let mut p = Printer::new(cmd)
        .without("author")
        .with("introduction", INTRODUCTION);

    p.template_keys_mut().push("examples");
    p.template_keys_mut().push("after");
    p.set_template("examples", EXAMPLE_TEMPLATE);
    for (i, example) in EXAMPLES.iter().enumerate() {
        p.expander_mut()
            .sub("examples")
            .set("example-number", i + 1)
            .set("example-title", example.title)
            .set("example-cmd", example.cmd)
            .set_md("example-comments", example.comments);
    }
    p.template_keys_mut().push("author_custom");
    p.set_template("author_custom", AUTHOR);
    p.set_template("after", AFTER_HELP);

    let skin = p.skin_mut();
    skin.headers[0].set_fg(Color::AnsiValue(198));
    skin.bold.set_fg(Color::AnsiValue(198));

    p
}

pub fn get_matches(cmd: Command) -> ArgMatches {
    cmd.get_matches()
}
