use std::fmt;

/// Errors that can occur when reading, writing, or querying a DDS file.
#[derive(Debug)]
pub enum Error {
    /// A formatting error from [`std::fmt`].
    Fmt(fmt::Error),
    /// An I/O error from reading or writing.
    Io(std::io::Error),
    /// A general error with a descriptive message.
    General(String),
    /// The file does not start with the `"DDS "` magic number.
    BadMagicNumber,
    /// A header field contains an invalid or unrecognized value.
    InvalidField(String),
    /// The file was truncated before all expected data could be read.
    ShortFile,
    /// The format is not supported well enough to complete the requested operation.
    UnsupportedFormat,
    /// An array layer index was out of bounds.
    OutOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Fmt(ref e) => write!(f, "{}", e),
            Error::Io(ref e) => write!(f, "{}", e),
            Error::General(ref s) => write!(f, "General Error: {}", s),
            Error::BadMagicNumber => write!(f, "Bad Magic Number"),
            Error::InvalidField(ref s) => write!(f, "Invalid Field: {}", s),
            Error::ShortFile => write!(f, "File is cut short"),
            Error::UnsupportedFormat => {
                write!(f, "Format is not supported well enough for this operation")
            }
            Error::OutOfBounds => write!(f, "Request is out of bounds"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Fmt(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Error {
        Error::Fmt(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error::General(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::General(s)
    }
}
