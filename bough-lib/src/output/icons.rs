use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg(feature="clap")]
#[derive(ValueEnum)]
pub enum IconType {
    #[default]
    None,
    Nerd,
    Unicode,
    Emoji,
}

impl Display for IconType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "none",
            Self::Nerd => "nerd",
            Self::Unicode => "unicode",
            Self::Emoji => "emoji",
        })
    }
}

impl FromStr for IconType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "nerd" => Ok(Self::Nerd),
            "unicode" => Ok(Self::Unicode),
            "emoji" => Ok(Self::Emoji),
            _ => Err(String::from("Invalid string value for conversion")),
        }
    }
}
