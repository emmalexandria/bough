use clap::Parser;

use crate::cli::{print_help_if_needed, Args};
use bough_lib::file::FileTree;

mod cli;

fn main() {
    let args = Args::parse();

    if print_help_if_needed(&args) {
        return;
    }

    let tree = FileTree::new("./").unwrap().build().unwrap();

    println!("{:?}", tree);
}
