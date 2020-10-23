use std::error;
use std::num::ParseIntError;
use std::char::ParseCharError;
use std::fmt;

#[derive(Debug)]
pub(crate) enum PositionParseError {
    InvalidRow(char),
    InvalidCol(char),
    InvalidLength(usize)
}

impl fmt::Display for PositionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PositionParseError::InvalidLength(length)
                => write!(f, "Only strings of length 2 can be parsed! Actual length is {}", length),
            PositionParseError::InvalidRow(char)
                => write!(f, "Valid rows range from 1 to 8! Actual row char is {}", char),
            PositionParseError::InvalidCol(char)
                => write!(f, "Valid columns range from a to h! Actual col char is {}", char)
        }
    }
}

impl error::Error for PositionParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None
        }
    }
}