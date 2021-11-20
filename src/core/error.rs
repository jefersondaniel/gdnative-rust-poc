use std::fmt;

#[derive(Debug, Clone)]
pub struct DataError {
    pub message: String,
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.to_string())
    }
}

impl DataError {
    pub fn new(message: String) -> DataError {
        DataError { message }
    }
}
