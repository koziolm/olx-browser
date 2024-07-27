use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {                     
    NetworkError(reqwest::Error),
    ParsingError(String),
    IoError(std::io::Error),
    ParseError(String),  
    SerializationError(String),
    Other(String)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NetworkError(e) => write!(f, "Network error: {}", e),
            AppError::ParsingError(e) => write!(f, "Parsing error: {}", e),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),  
            AppError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            AppError::Other(e) => write!(f, "Other error: {}", e)  
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

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::SerializationError(error.to_string())
    }
}           

//error handling for csv saving
impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Other(error.to_string())
    }
}

impl From<csv::Error> for AppError {
    fn from(error: csv::Error) -> Self {
        AppError::Other(error.to_string())
    }
}

impl From<csv::IntoInnerError<csv::Writer<Vec<u8>>>> for AppError {
    fn from(error: csv::IntoInnerError<csv::Writer<Vec<u8>>>) -> Self {
        AppError::Other(error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        AppError::Other(error.to_string())
    }
}

// to be able to use `?` with String errors
impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::ParseError(error)
    }
}