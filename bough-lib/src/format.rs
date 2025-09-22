#[cfg(feature = "ansi")]
mod ansi;
mod text;

pub trait TreeFormatItem {}

pub trait TreeFormat<T, C> {
    fn display(&self, tree: &T, config: &C);
}

pub struct FormatConfig {
    hidden: bool,
    git: bool,
}
