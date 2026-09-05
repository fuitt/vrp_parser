use std::convert::TryFrom;

use crate::ProblemType;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum ParseProblemTypeError {
    #[error("unknown problem type: {0}")]
    UnknownType(String),
    #[error("problem type {0} is not supported in VRPLib format")]
    NotSupportedInVrplib(String),
}

impl TryFrom<&str> for ProblemType {
    type Error = ParseProblemTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "CVRP" => Ok(ProblemType::Cvrp),
            "CVRPTW" => Err(ParseProblemTypeError::NotSupportedInVrplib(
                value.to_string(),
            )),
            _ => Err(ParseProblemTypeError::UnknownType(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cvrp() {
        let s = "CVRP";

        let value = ProblemType::try_from(s).unwrap();

        assert_eq!(value, ProblemType::Cvrp);
    }

    #[test]
    fn test_parse_cvrptw_fails_with_specific_error() {
        let s = "CVRPTW";

        let value = ProblemType::try_from(s).unwrap_err();

        assert_eq!(
            value,
            ParseProblemTypeError::NotSupportedInVrplib("CVRPTW".to_string())
        );
    }

    #[test]
    fn test_parse_unknown_fails() {
        let s = "foo_vrp";

        let value = ProblemType::try_from(s).unwrap_err();

        assert_eq!(
            value,
            ParseProblemTypeError::UnknownType("foo_vrp".to_string())
        );
    }
}
