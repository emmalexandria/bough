use crate::{
    cli::{build_cli, get_matches, Args},
    files::{ignore_git, FileTree, IgnoreFiles},
};

mod cli;
mod files;

fn main() {
    let cli = build_cli();
    let matches = get_matches(cli);

    let args = Args::from(matches.clone());

    let mut ignore = IgnoreFiles {
        hidden: !args.all_files,
        paths: vec![],
    };

    ignore.paths.append(&mut ignore_git().unwrap());

    let init_tree = FileTree::build(args.path, args.depth, ignore);

    println!("{init_tree}");
}
