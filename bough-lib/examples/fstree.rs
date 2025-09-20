use bough_lib::fs_tree::FsTree;

fn main() {
    let tree = FsTree::build("/Users/emma", None).unwrap();
    println!("{}", tree.len())
}
