use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NodeCoordType {
    TwodCoords,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum NodeCoordTypeParseError {
    #[error("Unknown node coordinate type: {0}")]
    UnknownType(String),
}

impl TryFrom<&str> for NodeCoordType {
    type Error = NodeCoordTypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "TWOD_COORDS" {
            return Ok(NodeCoordType::TwodCoords);
        }
        Err(NodeCoordTypeParseError::UnknownType(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_explicit() {
        let s = "TWOD_COORDS";

        let value = NodeCoordType::try_from(s).unwrap();

        assert_eq!(value, NodeCoordType::TwodCoords);
    }

    #[test]
    fn test_parse_fails() {
        let s = "foo_node_coord";

        let value = NodeCoordType::try_from(s).unwrap_err();

        assert_eq!(
            value,
            NodeCoordTypeParseError::UnknownType("foo_node_coord".to_string())
        );
    }
}
