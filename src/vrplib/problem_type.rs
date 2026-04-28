use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum ProblemType {
    CVRP,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ProblemTypeParseError {
    #[error("Unknown problem type: {0}")]
    UnknownType(String),
}

impl TryFrom<&str> for ProblemType {
    type Error = ProblemTypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "CVRP" {
            return Ok(ProblemType::CVRP);
        }
        Err(ProblemTypeParseError::UnknownType(value.to_string()))
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
            ProblemTypeParseError::UnknownType("foo_vrp".to_string())
        );
    }
}
