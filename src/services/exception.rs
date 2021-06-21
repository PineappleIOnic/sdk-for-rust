use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct AppwriteException {
    pub message: String,
    pub code: i32,
    pub version: String
}

impl fmt::Display for AppwriteException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"ERROR: '{}' CODE: {}", self.message, self.code)
    }
}

impl Error for AppwriteException {
    fn description(&self) -> &str {
        &self.message
    }
}

impl AppwriteException {
    pub fn new(message: String, code: i32, version: String) -> Self {
        Self {
            message: message,
            code: code,
            version: version
        }
    }
}