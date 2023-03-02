use std::fmt::Display;

// To implement the std:error::Error trait we need to implement Debug and Display.
#[derive(Debug)]
pub enum Errors {
    BadParamErr(&'static str),
    //OptionErr(String),
    //IOErr(std::io::Error),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadParamErr(e) => write!(f, "Bad parameter: {}", e),
        }
    }
}

// Automatic wrapping of retrieved errors
impl From<&'static str> for Errors {
    fn from(err: &'static str) -> Self {
        Errors::BadParamErr(err)
    }
}
