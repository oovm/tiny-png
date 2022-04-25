use super::*;

impl From<IoError> for TinyError {
    fn from(value: IoError) -> Self {
        Self::IoError(value.to_string())
    }
}
