use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::TokenParseError;
use super::lexer::tokenize;
use super::parser::FormatError;
use super::parser::parse;
use crate::VRPInstance;
use crate::VRPInstanceBuilder;
use crate::instance::InstanceError;

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Token parse error: {0}")]
    TokenParse(#[from] TokenParseError),

    #[error("Format error: {0}")]
    Format(#[from] FormatError),

    #[error("Instance error: {0}")]
    Build(#[from] InstanceError),
}

pub fn read_from_vrplib<P>(filename: P) -> Result<VRPInstance, LoadError>
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

    let parsed = parse(&tokens)?;
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
