use std::{convert::TryFrom, num::ParseIntError};

use super::{EdgeWeightFormat, EdgeWeightFormatParseError};
use super::{EdgeWeightType, EdgeWeightTypeParseError};
use super::{NodeCoordType, NodeCoordTypeParseError};
use super::{ProblemType, ProblemTypeParseError};

#[derive(Debug, PartialEq)]
pub enum Token {
    Name(String),
    Comment(String),
    Type(ProblemType),
    Dimension(usize),
    EdgeWeightType(EdgeWeightType),
    EdgeWeightFormat(EdgeWeightFormat),
    NodeCoordTtype(NodeCoordType),
    Capacity(u64),
    EdgeWeightSection,
    NodeCoordSection,
    DemandSection,
    DepotSection,
    EOF,
    Data(Vec<u64>),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TokenParseError {
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Invalid value: {0}")]
    InvalidData(String),

    #[error("Invalid value: {0}")]
    InvalidProblemType(#[from] ProblemTypeParseError),

    #[error("Invalid value: {0}")]
    InvalidDimension(#[from] ParseIntError),

    #[error("Invalid value: {0}")]
    InvalidEdgeWeightType(#[from] EdgeWeightTypeParseError),

    #[error("Invalid value: {0}")]
    InvalidEdgeWeightFormat(#[from] EdgeWeightFormatParseError),

    #[error("Invalid value: {0}")]
    InvalidNodeCoordType(#[from] NodeCoordTypeParseError),

    #[error("Invalid value: {0}")]
    InvalidCapacity(ParseIntError),
}

impl TryFrom<&str> for Token {
    type Error = TokenParseError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        if line.contains(':') {
            let (key, value) = line.split_once(':').unwrap();
            let key = key.trim();
            let value = value.trim();

            if key == "NAME" {
                return Ok(Self::Name(value.to_string()));
            }
            if key == "COMMENT" {
                return Ok(Self::Comment(value.to_string()));
            }
            if key == "TYPE" {
                let prob_type = ProblemType::try_from(value);
                return match prob_type {
                    Ok(_) => Ok(Token::Type(prob_type.unwrap())),
                    Err(_) => Err(TokenParseError::InvalidProblemType(prob_type.unwrap_err())),
                };
            }
            if key == "DIMENSION" {
                let d = value.parse::<usize>();
                return match d {
                    Ok(_) => Ok(Token::Dimension(d.unwrap())),
                    Err(_) => Err(TokenParseError::InvalidDimension(d.unwrap_err())),
                };
            }
            if key == "EDGE_WEIGHT_TYPE" {
                let weight_type = EdgeWeightType::try_from(value);
                return match weight_type {
                    Ok(_) => Ok(Token::EdgeWeightType(weight_type.unwrap())),
                    Err(_) => Err(TokenParseError::InvalidEdgeWeightType(
                        weight_type.unwrap_err(),
                    )),
                };
            }
            if key == "EDGE_WEIGHT_FORMAT" {
                let weight_format = EdgeWeightFormat::try_from(value);
                return match weight_format {
                    Ok(_) => Ok(Token::EdgeWeightFormat(weight_format.unwrap())),
                    Err(_) => Err(TokenParseError::InvalidEdgeWeightFormat(
                        weight_format.unwrap_err(),
                    )),
                };
            }
            if key == "NODE_COORD_TYPE" {
                let coord_type = NodeCoordType::try_from(value);
                return match coord_type {
                    Ok(_) => Ok(Token::NodeCoordTtype(coord_type.unwrap())),
                    Err(_) => Err(TokenParseError::InvalidNodeCoordType(
                        coord_type.unwrap_err(),
                    )),
                };
            }
            if key == "CAPACITY" {
                let d = value.parse::<u64>();
                return match d {
                    Ok(_) => Ok(Token::Capacity(d.unwrap())),
                    Err(_) => Err(TokenParseError::InvalidCapacity(d.unwrap_err())),
                };
            }
            Err(TokenParseError::InvalidKey(key.to_string()))
        } else {
            let line = line.trim();
            if line == "EDGE_WEIGHT_SECTION" {
                return Ok(Token::EdgeWeightSection);
            }
            if line == "NODE_COORD_SECTION" {
                return Ok(Token::NodeCoordSection);
            }
            if line == "DEMAND_SECTION" {
                return Ok(Token::DemandSection);
            }
            if line == "DEPOT_SECTION" {
                return Ok(Token::DepotSection);
            }
            if line == "-1" || line == "EOF" {
                return Ok(Token::EOF);
            }
            let data: Result<Vec<_>, _> =
                line.split_whitespace().map(|x| x.parse::<u64>()).collect();
            match data {
                Ok(_) => Ok(Token::Data(data.unwrap())),
                Err(_) => Err(TokenParseError::InvalidData(line.to_string())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        let s = "NAME : instance 001\n";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::Name("instance 001".to_string()));
    }

    #[test]
    fn test_parse_comment() {
        let s = "COMMENT : This is a test instance. \n";

        let value = Token::try_from(s).unwrap();

        assert_eq!(
            value,
            Token::Comment("This is a test instance.".to_string())
        );
    }

    #[test]
    fn test_parse_type() {
        let s = "TYPE : CVRP";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::Type(ProblemType::CVRP));
    }

    #[test]
    fn test_parse_type_fails() {
        let s = "TYPE : foo_vrp";

        let value = Token::try_from(s).unwrap_err();

        assert_eq!(
            value,
            TokenParseError::InvalidProblemType(ProblemTypeParseError::UnknownType(
                "foo_vrp".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_dimension() {
        let s = "DIMENSION : 123";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::Dimension(123));
    }

    #[test]
    fn test_parse_edge_weight_type() {
        let s = "EDGE_WEIGHT_TYPE : EXPLICIT";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::EdgeWeightType(EdgeWeightType::Explicit));
    }

    #[test]
    fn test_parse_edge_weight_format() {
        let s = "EDGE_WEIGHT_FORMAT : LOWER_ROW";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::EdgeWeightFormat(EdgeWeightFormat::LowerRow));
    }

    #[test]
    fn test_parse_node_coord_type() {
        let s = "NODE_COORD_TYPE : TWOD_COORDS";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::NodeCoordTtype(NodeCoordType::TwodCoords));
    }

    #[test]
    fn test_parse_capacity() {
        let s = "CAPACITY : 45";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::Capacity(45));
    }

    #[test]
    fn test_parse_edge_weight_section() {
        let s = "EDGE_WEIGHT_SECTION";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::EdgeWeightSection);
    }

    #[test]
    fn test_parse_node_coord_section() {
        let s = "NODE_COORD_SECTION";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::NodeCoordSection);
    }

    #[test]
    fn test_parse_demand_section() {
        let s = "DEMAND_SECTION";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::DemandSection);
    }

    #[test]
    fn test_parse_depot_section() {
        let s = "DEPOT_SECTION";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::DepotSection);
    }

    #[test]
    fn test_parse_eof_num() {
        let s = "-1";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::EOF);
    }

    #[test]
    fn test_parse_eof_string() {
        let s = "EOF";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::EOF);
    }

    #[test]
    fn test_parse_data() {
        let s = "  1 2 3 4 5 6 7 8 9 10\n";

        let value = Token::try_from(s).unwrap();

        assert_eq!(value, Token::Data(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    fn test_parse_data_fails() {
        let s = "  1 2 3 4 ABC 6 7 8 9 10\n";

        let value = Token::try_from(s).unwrap_err();

        assert_eq!(
            value,
            TokenParseError::InvalidData("1 2 3 4 ABC 6 7 8 9 10".to_string())
        );
    }
}
