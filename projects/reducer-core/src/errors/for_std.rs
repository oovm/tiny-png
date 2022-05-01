use super::*;
use std::time::SystemTimeError;

impl From<IoError> for TinyError {
    fn from(value: IoError) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<SystemTimeError> for TinyError {
    fn from(value: SystemTimeError) -> Self {
        Self::FormatError(value.to_string())
    }
}
