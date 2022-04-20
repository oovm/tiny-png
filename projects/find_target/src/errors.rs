use oxipng::PngError;

#[derive(Debug, Clone)]
pub enum TinyError {
    FormatError(String),
    TimedOut,
    ImageOptimized,
    ChunkError(String),
    UnknownError,
}

pub type Result<T> = std::result::Result<T, TinyError>;

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
