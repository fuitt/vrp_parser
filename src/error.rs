use crate::instance::ValidationError;
use crate::vrplib::lexer::TokenError;
use crate::vrplib::parser::ParseError;

/// Represents all possible errors that can occur while loading a VRP instance file.
///
/// This error type aggregates lower-level errors produced during the loading
/// pipeline, including file I/O failures, tokenization errors, parsing errors,
/// and validation errors encountered when constructing a `VrpInstance`.
///
/// Each variant corresponds to a specific stage of the loading process.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// An underlying file I/O error occurred while reading the file.
    ///
    /// This typically indicates that the file does not exist, cannot be opened,
    /// or could not be read due to operating system–level issues.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// A tokenization error occurred while converting raw text into tokens.
    ///
    /// This variant is returned when the lexical analysis stage fails, such as
    /// encountering an unexpected character or malformed token.
    #[error("token error: {0}")]
    Token(#[from] TokenError),

    /// A parsing error occurred while interpreting the tokenized input.
    ///
    /// This indicates that the file contains syntactically invalid structures
    /// or violates the expected format.
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    /// A validation error occurred while constructing a `VrpInstance`.
    ///
    /// This variant is returned when the parsed data is structurally correct
    /// but fails semantic validation, such as invalid problem definitions.
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
}
