use clap::Parser;

use crate::cli::{print_help_if_needed, Args};

mod cli;

fn main() {
    let args = Args::parse();

    if print_help_if_needed(args) {
        return;
    }
}
