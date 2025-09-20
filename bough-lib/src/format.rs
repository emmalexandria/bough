pub mod html;
pub mod text;

use crate::tree::Tree;

pub trait TreeFormat {
    type Error;
    type Output;

    fn output(&self, tree: &Tree, width: usize) -> Result<Self::Output, Self::Error>;
}
