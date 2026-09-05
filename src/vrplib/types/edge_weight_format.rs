use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EdgeWeightFormat {
    LowerRow,
    FullMatrix,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
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
        if value == "FULL_MATRIX" {
            return Ok(EdgeWeightFormat::FullMatrix);
        }
        Err(ParseEdgeWeightFormatError::UnknownFormat(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lower_row() {
        let s = "LOWER_ROW";

        let value = EdgeWeightFormat::try_from(s).unwrap();

        assert_eq!(value, EdgeWeightFormat::LowerRow);
    }

    #[test]
    fn test_parse_full_matrix() {
        let s = "FULL_MATRIX";

        let value = EdgeWeightFormat::try_from(s).unwrap();

        assert_eq!(value, EdgeWeightFormat::FullMatrix);
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
