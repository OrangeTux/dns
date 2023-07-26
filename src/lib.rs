use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum DecodeError {
    /// Error that indicates byte stream is too short.
    NotEnoughBytes,

    /// Error indicating the (series of) bytes represent an value that's not allowed.
    IllegalValue(String),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason: String = match self {
            Self::NotEnoughBytes => "not enough bytes".into(),
            Self::IllegalValue(value) => value.to_string(),
        };

        write!(f, "failed to decode bytes: {}", reason)
    }
}

impl Error for DecodeError {}
