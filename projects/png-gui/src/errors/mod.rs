pub use std::io::Error as IoError;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use oxipng::PngError;

mod for_std;
pub type TinyResult<T = ()> = Result<T, TinyError>;

#[derive(Debug)]
pub enum TinyError {
    IoError(IoError),
    FormatError(String),
    TimedOut,
    ImageOptimized,
    ChunkError(String),
    UnknownError,
}

impl From<PngError> for TinyError {
    fn from(error: PngError) -> Self {
        match error {
            PngError::DeflatedDataTooLong(e) => TinyError::ChunkError(format!("DeflatedDataTooLong {}", e)),
            PngError::TimedOut => TinyError::TimedOut,
            PngError::NotPNG => TinyError::FormatError(format!("Except png, found unknown")),
            PngError::APNGNotSupported => TinyError::ChunkError("APNGNotSupported".to_string()),
            PngError::InvalidData => TinyError::ChunkError("InvalidData".to_string()),
            PngError::TruncatedData => TinyError::ChunkError("TruncatedData".to_string()),
            PngError::ChunkMissing(e) => TinyError::ChunkError(e.to_string()),
            PngError::Other(e) => TinyError::ChunkError(e.to_string()),
            _ => panic!("Unsolved error {}", error),
        }
    }
}

impl Display for TinyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for TinyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TinyError::IoError(e) => Some(e),
            _ => None,
        }
    }
}
