use super::*;

impl From<walkdir::Error> for TinyError {
    fn from(value: walkdir::Error) -> Self {
        Self::IoError(value.to_string())
    }
}
