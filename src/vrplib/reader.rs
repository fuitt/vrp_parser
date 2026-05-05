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

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("token error: {0}")]
    Token(#[from] TokenError),

    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
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
