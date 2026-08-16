use crate::instance::ValidationError;
use crate::solomon::parser::SolomonParseError;
use crate::vrplib::error::VrplibError;

/// Represents all possible errors that can occur while loading a VRP instance file.
///
/// This error type aggregates lower-level errors produced during the loading
/// pipeline, including file I/O failures, format-specific errors, and
/// validation errors encountered when constructing a `VrpInstance`.
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

    /// A VRPLib-specific error occurred during tokenization or parsing.
    ///
    /// Holds either a [`VrplibError::Token`] from the lexical analysis stage
    /// or a [`VrplibError::Parse`] from the structural parsing stage.
    #[error("vrplib error: {0}")]
    Vrplib(#[from] VrplibError),

    /// A Solomon parse error occurred while reading a Solomon-formatted file.
    ///
    /// This variant is returned when the file does not conform to the
    /// expected Solomon layout.
    #[error("solomon parse error: {0}")]
    Solomon(#[from] SolomonParseError),

    /// A validation error occurred while constructing a `VrpInstance`.
    ///
    /// This variant is returned when the parsed data is structurally correct
    /// but fails semantic validation, such as invalid problem definitions.
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
}
