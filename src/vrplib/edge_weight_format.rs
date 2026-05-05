use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum EdgeWeightFormat {
    LowerRow,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseEdgeWeightFormatError {
    #[error("unknown edge weight format: {0}")]
    UnknownFormat(String),
}

impl TryFrom<&str> for EdgeWeightFormat {
    type Error = ParseEdgeWeightFormatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "LOWER_ROW" {
            return Ok(EdgeWeightFormat::LowerRow);
        }
        Err(ParseEdgeWeightFormatError::UnknownFormat(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_explicit() {
        let s = "LOWER_ROW";

        let value = EdgeWeightFormat::try_from(s).unwrap();

        assert_eq!(value, EdgeWeightFormat::LowerRow);
    }

    #[test]
    fn test_parse_fails() {
        let s = "foo_edge_weight";

        let value = EdgeWeightFormat::try_from(s).unwrap_err();

        assert_eq!(
            value,
            ParseEdgeWeightFormatError::UnknownFormat("foo_edge_weight".to_string())
        );
    }
}
