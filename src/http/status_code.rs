use std::fmt::{Display, Formatter, Result as FormatterResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "BadRequest",
            Self::NotFound => "NotFound",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FormatterResult {
        write!(f, "{}", *self as u16)
    }
}
