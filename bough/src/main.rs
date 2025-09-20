use clap::Parser;

use crate::cli::{print_help_if_needed, Args};
use bough_lib::{Tree, TreeOptions};

mod cli;

fn main() {
    let args = Args::parse();

    if print_help_if_needed(&args) {
        return;
    }

    let options = TreeOptions {
        icons: args.icons.into(),
        depth: args.depth,
        hidden: args.all_files,
    };

    let tree = Tree::with_options(options).build(args.path);
}
