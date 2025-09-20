#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconType {
    None,
    Nerd,
    Unicode,
    Emoji,
}

pub trait Icon {
    fn get_icon(&self, icon_type: IconType) -> Option<String>;
}
