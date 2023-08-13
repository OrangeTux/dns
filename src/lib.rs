//! A DNS resolver according to [`RFC 1035`].
//!
//! This project can be used both as a library and as a binary.
//! The library exposes relevant types to build your own DNS server or DNS client.
//! The project also contains a DNS resolver as a binary.
//!
//! [`RFC 1035`]: https://datatracker.ietf.org/doc/html/rfc1035
#![warn(missing_docs)]
mod domain_name;
pub mod header;
pub mod message;
pub mod resource_record;
pub mod sections;

pub use message::Message;

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
