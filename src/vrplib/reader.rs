use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::error::VrplibError;
use super::lexer::tokenize;
use super::parser::parse;
use crate::LoadError;
use crate::VrpInstance;
use crate::VrpInstanceBuilder;

/// Loads a VRPLib file and constructs a `VrpInstance`.
///
/// # Errors
/// - [`LoadError::Io`] if reading the file fails
/// - [`LoadError::Vrplib`] if tokenizing or parsing the VRPLib file fails
/// - [`LoadError::Validation`] if instance validation fails
/// # Examples
/// ```no_run
/// let instance = vrp_parser::read_from_vrplib("ORTEC-n242-k12.vrp").unwrap();
/// assert_eq!(instance.dimension(), 242);
/// ```
pub fn read_from_vrplib<P>(filename: P) -> Result<VrpInstance<u64>, LoadError>
where
    P: AsRef<Path>,
{
    let mut tokens = vec![];
    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let tokenized = tokenize(&line).map_err(VrplibError::Token)?;
        tokens.push(tokenized);
    }

    let parsed = parse::<u64>(&tokens).map_err(VrplibError::Parse)?;
    let builder = VrpInstanceBuilder::make_from_vrplib(parsed);
    let instance = builder.build()?;
    Ok(instance)
}

/// Loads a VRPLib file and constructs a `VrpInstance<f64>`.
///
/// Identical to [`read_from_vrplib`] except that all numeric values
/// (edge weights, demands, capacity) are represented as `f64`. Edge weights
/// computed from coordinates are rounded to the nearest integer and then
/// stored as `f64`.
///
/// # Errors
/// - [`LoadError::Io`] if reading the file fails
/// - [`LoadError::Vrplib`] if tokenizing or parsing the VRPLib file fails
/// - [`LoadError::Validation`] if instance validation fails
pub fn read_from_vrplib_f64<P>(filename: P) -> Result<VrpInstance<f64>, LoadError>
where
    P: AsRef<Path>,
{
    let mut tokens = vec![];
    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let tokenized = tokenize(&line).map_err(VrplibError::Token)?;
        tokens.push(tokenized);
    }

    let parsed = parse::<f64>(&tokens).map_err(VrplibError::Parse)?;
    let builder = VrpInstanceBuilder::make_from_vrplib(parsed);
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
