use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum EdgeWeightType {
    Explicit,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EdgeWeightTypeParseError {
    #[error("Unknown edge weight type: {0}")]
    UnknownType(String),
}

impl TryFrom<&str> for EdgeWeightType {
    type Error = EdgeWeightTypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "EXPLICIT" {
            return Ok(EdgeWeightType::Explicit);
        }
        Err(EdgeWeightTypeParseError::UnknownType(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_explicit() {
        let s = "EXPLICIT";

        let value = EdgeWeightType::try_from(s).unwrap();

        assert_eq!(value, EdgeWeightType::Explicit);
    }

    #[test]
    fn test_parse_fails() {
        let s = "foo_edge_weight";

        let value = EdgeWeightType::try_from(s).unwrap_err();

        assert_eq!(
            value,
            EdgeWeightTypeParseError::UnknownType("foo_edge_weight".to_string())
        );
    }
}
