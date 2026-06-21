use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::ParseError;
use super::TokenError;
use super::lexer::tokenize;
use super::parser::parse;
use crate::VRPInstance;
use crate::VRPInstanceBuilder;
use crate::instance::ValidationError;

///
/// Represents all possible errors that can occur while loading a VRPLib file.
///
/// This error type aggregates lower-level errors produced during the loading
/// pipeline, including file I/O failures, tokenization errors, parsing errors,
/// and validation errors encountered when constructing a `VRPInstance`.
///
/// Each variant corresponds to a specific stage of the loading process.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// An underlying file I/O error occurred while reading the VRPLib file.
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
    /// This indicates that the VRPLib file contains syntactically invalid
    /// structures or violates the expected VRPLib format.
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    /// A validation error occurred while constructing a `VRPInstance`.
    ///
    /// This variant is returned when the parsed data is structurally correct
    /// but fails semantic validation, such as invalid problem definitions.
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
}

/// Loads a VRPLib file and constructs a `VRPInstance`.
///
/// # Errors
/// - [`LoadError::Io`] if reading the file fails
/// - [`LoadError::Token`] if tokenizing the VRPLib file fails
/// - [`LoadError::Parse`] if parsing the VRPLib file fails
/// - [`LoadError::Validation`] if instance validation fails
/// # Examples
/// ```no_run
/// let instance = vrp_parser::read_from_vrplib("ORTEC-n242-k12.vrp").unwrap();
/// assert_eq!(instance.dimension(), 242);
/// ```
pub fn read_from_vrplib<P>(filename: P) -> Result<VRPInstance<u64>, LoadError>
where
    P: AsRef<Path>,
{
    let mut tokens = vec![];
    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let tokenized = tokenize(&line)?;
        tokens.push(tokenized);
    }

    let parsed = parse::<u64>(&tokens)?;
    let builder = VRPInstanceBuilder::make_from_vrplib(parsed);
    let instance = builder.build()?;
    Ok(instance)
}

/// Loads a VRPLib file and constructs a `VRPInstance<f64>`.
///
/// Identical to [`read_from_vrplib`] except that all numeric values
/// (edge weights, demands, capacity) are represented as `f64`. Edge weights
/// computed from coordinates are rounded to the nearest integer and then
/// stored as `f64`.
///
/// # Errors
/// - [`LoadError::Io`] if reading the file fails
/// - [`LoadError::Token`] if tokenizing the VRPLib file fails
/// - [`LoadError::Parse`] if parsing the VRPLib file fails
/// - [`LoadError::Validation`] if instance validation fails
pub fn read_from_vrplib_f64<P>(filename: P) -> Result<VRPInstance<f64>, LoadError>
where
    P: AsRef<Path>,
{
    let mut tokens = vec![];
    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let tokenized = tokenize(&line)?;
        tokens.push(tokenized);
    }

    let parsed = parse::<f64>(&tokens)?;
    let builder = VRPInstanceBuilder::make_from_vrplib(parsed);
    let instance = builder.build()?;
    Ok(instance)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
