use bough_lib::file::FsTree;

fn main() {
    let tree = FsTree::build("./", None).unwrap();
    println!("{tree}");
}
