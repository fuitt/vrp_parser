use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::lexer::tokenize;
use super::parser::parse;
use crate::LoadError;
use crate::VrpInstance;
use crate::VrpInstanceBuilder;

/// Loads a Solomon-formatted instance file and constructs a `VrpInstance<f64>`.
///
/// The Solomon format encodes a CVRPTW instance: nodes with 2D coordinates,
/// demands, time windows, service times, and a vehicle fleet with uniform
/// capacity. Edge weights are computed as rounded Euclidean distances.
///
/// Node 0 in the file is the depot; all remaining nodes are customers.
/// The depot index stored in [`VrpInstance::depots`] is `0` (0-based).
///
/// # Errors
/// - [`LoadError::Io`] if reading the file fails
/// - [`LoadError::Solomon`] if parsing the file fails
/// - [`LoadError::Validation`] if instance validation fails
pub fn read_from_solomon<P>(filename: P) -> Result<VrpInstance<f64>, LoadError>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    let tokens = tokenize(&lines);
    let data = parse(&tokens)?;
    let builder = VrpInstanceBuilder::make_from_solomon(data);
    let instance = builder.build()?;
    Ok(instance)
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect()
}
