use bough_lib::file::FileTree;

fn main() {
    let tree = FileTree::new("./").unwrap().build().unwrap();

    println!("{:?}", tree)
}
