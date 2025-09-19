use std::path::PathBuf;

use bough_lib::output::icons::IconType;
use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command, ValueEnum};

#[derive(ValueEnum, Clone, Copy)]
pub enum IconArg {
    None,
    Nerd,
    Unicode,
    Emoji,
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

// I personally prefer the clap builder api, but having a struct of arguments is still nice
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Args {
    pub depth: usize,
    pub icons: IconType,
    pub path: PathBuf,
    pub all_files: bool,
    pub root: bool,
}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Self {
            depth: matches.get_one::<usize>("depth").copied().unwrap_or(0),
            icons: matches
                .get_one::<IconArg>("icons")
                .copied()
                .unwrap_or(IconArg::None)
                .into(),
            path: matches
                .get_one::<PathBuf>("path")
                .cloned()
                .unwrap_or(PathBuf::from("./")),
            all_files: matches.get_one::<bool>("all").copied().unwrap_or_default(),
            root: matches.get_one::<bool>("root").copied().unwrap_or_default(),
        }
    }
}

pub fn build_cli() -> Command {
    let tui = command!("tui").about("Enter the TUI mode for more advanced editing");

    command!()
        .subcommand(tui)
        .arg(
            Arg::new("path")
                .num_args(1)
                .value_parser(value_parser!(PathBuf))
                .default_value("./"),
        )
        .arg(
            Arg::new("icons")
                .value_name("style")
                .short('i')
                .long("icons")
                .default_value("none")
                .value_parser(value_parser!(IconArg))
                .help("The style of icons to use"),
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_parser(value_parser!(usize))
                .default_value("0")
                .help("The depth to build the tree to. 0 will build a tree as large as possible"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Show hidden files"),
        )
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .action(ArgAction::SetTrue)
                .help("Show the root directory in the file tree"),
        )
}

pub fn get_matches(cmd: Command) -> ArgMatches {
    cmd.get_matches()
}
