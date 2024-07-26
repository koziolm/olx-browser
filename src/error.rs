use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NetworkError(reqwest::Error),
    ParsingError(String),
    IoError(std::io::Error),
    ParseError(String),  
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NetworkError(e) => write!(f, "Network error: {}", e),
            AppError::ParsingError(e) => write!(f, "Parsing error: {}", e),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),  
        }
    }
}

impl Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::NetworkError(error)
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

// to be able to use `?` with String errors
impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::ParseError(error)
    }
}