use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Errors {
    IOErr(io::Error),
    TomlErr(toml::de::Error),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOErr(e) => write!(f, "main: IO error: {}", e),
            Self::TomlErr(e) => write!(f, "main: Failed to deserialize configuration: {}", e),
        }
    }
}

// Automatic wrapping of retrieved errors
impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Self {
        Errors::IOErr(err)
    }
}

impl From<toml::de::Error> for Errors {
    fn from(err: toml::de::Error) -> Self {
        Errors::TomlErr(err)
    }
}
