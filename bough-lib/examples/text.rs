use std::path::PathBuf;

use bough_lib::{
    format::{
        text::{TextOutput, TextOutputConfig},
        TreeFormat,
    },
    icons::IconType,
    Tree, TreeOptions,
};

fn main() {
    let tree = Tree::with_options(TreeOptions {
        icons: IconType::None,
        depth: 2,
        hidden: false,
    })
    .build("./");

    let output = TextOutput::new(TextOutputConfig {
        indent_width: 2,
        icon_type: IconType::None,
        ansi: false,
    })
    .output(&tree, 50)
    .unwrap();

    println!("{output}")
}
