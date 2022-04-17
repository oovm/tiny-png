#[derive(Debug, Copy, Clone)]
pub enum TinyError {
    ImageOptimized,
    UnknownError,
}

pub type Result<T> = std::result::Result<T, TinyError>;
