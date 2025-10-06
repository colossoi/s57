use thiserror::Error;

/// Specific kinds of parse errors
#[derive(Debug, Error)]
pub enum ParseErrorKind {
    #[error("io error while reading")]
    Io(#[from] #[source] std::io::Error),

    #[error("invalid UTF-8: {0}")]
    InvalidUtf8(#[from] #[source] std::str::Utf8Error),

    #[error("invalid integer: {0}")]
    InvalidInteger(#[from] #[source] std::num::ParseIntError),

    #[error("invalid record leader: {0}")]
    InvalidLeader(String),

    #[error("invalid directory entry: {0}")]
    InvalidDirectory(String),

    #[error("invalid field data: {0}")]
    InvalidField(String),

    #[error("record length {record_length} exceeds available data {available}")]
    RecordTooLarge {
        record_length: usize,
        available: usize,
    },

    #[error("field extends beyond field area: start={start}, length={length}, area_len={area_len}")]
    FieldOutOfBounds {
        start: usize,
        length: usize,
        area_len: usize,
    },

    #[error("unexpected end of file")]
    UnexpectedEof,

    #[error("parse error: {0}")]
    Other(String),
}

/// Parse error with byte offset context
#[derive(Debug, Error)]
#[error("{kind} at byte offset {offset}")]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub offset: usize,
}

impl ParseError {
    /// Create a new parse error with the given kind and byte offset
    pub fn at(kind: ParseErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }
}

/// Result type for S-57 operations
pub type Result<T> = std::result::Result<T, ParseError>;
