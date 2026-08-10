use std::convert::TryFrom;

use crate::ProblemType;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum ParseProblemTypeError {
    #[error("unknown problem type: {0}")]
    UnknownType(String),
}

impl TryFrom<&str> for ProblemType {
    type Error = ParseProblemTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "CVRP" {
            return Ok(ProblemType::CVRP);
        }
        Err(ParseProblemTypeError::UnknownType(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cvrp() {
        let s = "CVRP";

        let value = ProblemType::try_from(s).unwrap();

        assert_eq!(value, ProblemType::CVRP);
    }

    #[test]
    fn test_parse_fails() {
        let s = "foo_vrp";

        let value = ProblemType::try_from(s).unwrap_err();

        assert_eq!(
            value,
            ParseProblemTypeError::UnknownType("foo_vrp".to_string())
        );
    }
}
