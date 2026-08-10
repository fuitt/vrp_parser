use crate::Numeric;
use crate::ProblemType;

use super::Token;
use super::edge_weight_format::EdgeWeightFormat;
use super::edge_weight_type::EdgeWeightType;
use super::node_coord_type::NodeCoordType;

enum State {
    Header,
    EdgeWeightSection,
    NodeCoordSection,
    DemandSection,
    DepotSection,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct SectionData<T> {
    pub name: Option<String>,
    pub problem_type: Option<ProblemType>,
    pub dimension: Option<usize>,
    pub capacity: Option<T>,
    pub edge_weight_type: Option<EdgeWeightType>,
    pub edge_weight_format: Option<EdgeWeightFormat>,
    pub edge_weights: Vec<Vec<T>>,
    pub node_coord_type: Option<NodeCoordType>,
    pub node_coords: Vec<Vec<f64>>,
    pub demands: Vec<T>,
    pub depots: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("missing name")]
    MissingName,

    #[error("missing problem type")]
    MissingProblemType,

    #[error("missing dimension")]
    MissingDimension,

    #[error("missing edge weight type")]
    MissingEdgeWeightType,

    #[error("missing edge weight format")]
    MissingEdgeWeightFormat,

    #[error("missing node coord type")]
    MissingNodeCoordType,

    #[error("invalid problem type")]
    InvalidProblemType,

    #[error("invalid dimension")]
    InvalidDimension,

    #[error("invalid edge weight type")]
    InvalidEdgeWeightType,

    #[error("invalid edge weight format")]
    InvalidEdgeWeightFormat,

    #[error("invalid node coord type")]
    InvalidNodeCoordType,

    #[error("invalid capacity")]
    InvalidCapacity,

    #[error("invalid edge weight")]
    EdgeWeight,

    #[error("invalid node coord")]
    NodeCoord,

    #[error("invalid demand")]
    Demand,

    #[error("invalid depot")]
    Depot,
}

pub(crate) fn parse<T: Numeric>(tokens: &[Token]) -> Result<SectionData<T>, ParseError> {
    let section_data = parse_tokens(tokens)?;
    validate_format(&section_data)?;
    Ok(section_data)
}

fn parse_tokens<T: Numeric>(tokens: &[Token]) -> Result<SectionData<T>, ParseError> {
    let mut data = SectionData::default();

    let mut state = State::Header;
    let mut node_coord_index = 1;
    for token in tokens.iter() {
        match token {
            Token::Name(s) => data.name = Some(s.to_string()),
            Token::Comment(_) => {}
            Token::Type(s) => {
                data.problem_type = Some(
                    ProblemType::try_from(s.as_str())
                        .map_err(|_| ParseError::InvalidProblemType)?,
                );
            }
            Token::Dimension(s) => {
                data.dimension = Some(
                    s.parse::<usize>()
                        .map_err(|_| ParseError::InvalidDimension)?,
                );
            }
            Token::Capacity(s) => {
                data.capacity = Some(s.parse::<T>().map_err(|_| ParseError::InvalidCapacity)?);
            }
            Token::EdgeWeightType(s) => {
                data.edge_weight_type = Some(
                    EdgeWeightType::try_from(s.as_str())
                        .map_err(|_| ParseError::InvalidEdgeWeightType)?,
                );
            }
            Token::EdgeWeightFormat(s) => {
                data.edge_weight_format = Some(
                    EdgeWeightFormat::try_from(s.as_str())
                        .map_err(|_| ParseError::InvalidEdgeWeightFormat)?,
                );
            }
            Token::NodeCoordType(s) => {
                data.node_coord_type = Some(
                    NodeCoordType::try_from(s.as_str())
                        .map_err(|_| ParseError::InvalidNodeCoordType)?,
                );
            }
            Token::EdgeWeightSection => state = State::EdgeWeightSection,
            Token::NodeCoordSection => state = State::NodeCoordSection,
            Token::DemandSection => state = State::DemandSection,
            Token::DepotSection => state = State::DepotSection,
            Token::Data(d) => match state {
                State::Header => {}
                State::EdgeWeightSection => {
                    match d
                        .iter()
                        .map(|x| x.parse::<T>())
                        .collect::<Result<Vec<_>, _>>()
                    {
                        Ok(weights) => data.edge_weights.push(weights),
                        Err(_) => return Err(ParseError::EdgeWeight),
                    }
                }
                State::NodeCoordSection => {
                    if d.is_empty() {
                        return Err(ParseError::NodeCoord);
                    } else {
                        match d[0].parse::<usize>() {
                            Ok(val) if val == node_coord_index => {}
                            _ => return Err(ParseError::NodeCoord),
                        }
                        match d[1..]
                            .iter()
                            .map(|x| x.parse::<f64>())
                            .collect::<Result<Vec<_>, _>>()
                        {
                            Ok(coords) => data.node_coords.push(coords),
                            Err(_) => return Err(ParseError::NodeCoord),
                        }
                        node_coord_index += 1;
                    }
                }
                State::DemandSection => {
                    if d.len() != 2 {
                        return Err(ParseError::Demand);
                    }
                    let expected_index = data.demands.len() + 1;
                    match d[0].parse::<usize>() {
                        Ok(idx) if idx == expected_index => {}
                        _ => return Err(ParseError::Demand),
                    }
                    match d[1].parse::<T>() {
                        Ok(demand) => data.demands.push(demand),
                        Err(_) => return Err(ParseError::Demand),
                    }
                }
                State::DepotSection => {
                    if d.len() != 1 {
                        return Err(ParseError::Depot);
                    }
                    match d[0].parse::<usize>() {
                        Ok(depot) => data.depots.push(depot),
                        Err(_) => return Err(ParseError::Depot),
                    }
                }
            },
            Token::Eof => {}
        }
    }
    Ok(data)
}

fn validate_format<T: Numeric>(section_data: &SectionData<T>) -> Result<(), ParseError> {
    section_data.name.as_ref().ok_or(ParseError::MissingName)?;
    section_data
        .problem_type
        .as_ref()
        .ok_or(ParseError::MissingProblemType)?;
    section_data
        .dimension
        .as_ref()
        .ok_or(ParseError::MissingDimension)?;
    section_data
        .edge_weight_type
        .as_ref()
        .ok_or(ParseError::MissingEdgeWeightType)?;

    let dimension = section_data.dimension.unwrap();
    let edge_weight_type = section_data.edge_weight_type.unwrap();

    validate_edge_weights(
        dimension,
        edge_weight_type,
        section_data.edge_weight_format,
        &section_data.edge_weights,
    )?;
    validate_node_coords(
        dimension,
        edge_weight_type,
        &section_data.node_coord_type,
        &section_data.node_coords,
    )?;
    validate_demands(dimension, &section_data.demands)?;
    validate_depots(dimension, &section_data.depots)?;
    Ok(())
}

fn validate_edge_weights<T>(
    dimension: usize,
    edge_weight_type: EdgeWeightType,
    edge_weight_format: Option<EdgeWeightFormat>,
    edge_weights: &[Vec<T>],
) -> Result<(), ParseError> {
    match edge_weight_type {
        EdgeWeightType::Explicit => {
            if let Some(fmt) = edge_weight_format {
                match fmt {
                    EdgeWeightFormat::LowerRow => {
                        if dimension - 1 == edge_weights.len()
                            && edge_weights
                                .iter()
                                .enumerate()
                                .all(|(i, weights)| (i + 1) == weights.len())
                        {
                            Ok(())
                        } else {
                            Err(ParseError::EdgeWeight)
                        }
                    }
                    EdgeWeightFormat::FullMatrix => {
                        if dimension == edge_weights.len()
                            && edge_weights
                                .iter()
                                .all(|weights| dimension == weights.len())
                        {
                            Ok(())
                        } else {
                            Err(ParseError::EdgeWeight)
                        }
                    }
                }
            } else {
                Err(ParseError::MissingEdgeWeightFormat)
            }
        }
        EdgeWeightType::Euc2D => Ok(()),
    }
}

fn validate_node_coords(
    dimension: usize,
    edge_weight_type: EdgeWeightType,
    node_coord_type: &Option<NodeCoordType>,
    node_coords: &[Vec<f64>],
) -> Result<(), ParseError> {
    match edge_weight_type {
        EdgeWeightType::Explicit => match node_coord_type {
            Some(_) => {}
            None => {
                if !node_coords.is_empty() {
                    return Err(ParseError::MissingNodeCoordType);
                }
            }
        },
        EdgeWeightType::Euc2D => {
            if !is_2d_coords(dimension, node_coords) {
                return Err(ParseError::NodeCoord);
            }
        }
    }

    if let Some(coord_type) = node_coord_type {
        match coord_type {
            NodeCoordType::TwodCoords => {
                if !is_2d_coords(dimension, node_coords) {
                    return Err(ParseError::NodeCoord);
                }
            }
        }
    }

    Ok(())
}

fn is_2d_coords(dimension: usize, node_coords: &[Vec<f64>]) -> bool {
    dimension == node_coords.len() && node_coords.iter().all(|coords| coords.len() == 2)
}

fn validate_demands<T>(dimension: usize, demands: &[T]) -> Result<(), ParseError> {
    if demands.len() == dimension {
        Ok(())
    } else {
        Err(ParseError::Demand)
    }
}

fn validate_depots(dimension: usize, depots: &[usize]) -> Result<(), ParseError> {
    if depots.iter().all(|&depot| 1 <= depot && depot <= dimension) {
        Ok(())
    } else {
        Err(ParseError::Depot)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_tokens_succeeds() {
        let sut = vec![
            Token::Name("This is a name.".to_string()),
            Token::Comment("This is a comment.".to_string()),
            Token::Type("CVRP".to_string()),
            Token::Dimension("3".to_string()),
            Token::EdgeWeightType("EXPLICIT".to_string()),
            Token::EdgeWeightFormat("LOWER_ROW".to_string()),
            Token::NodeCoordType("TWOD_COORDS".to_string()),
            Token::Capacity("2".to_string()),
            Token::EdgeWeightSection,
            Token::Data(vec!["4".to_string()]),
            Token::Data(vec!["5".to_string(), "6".to_string()]),
            Token::NodeCoordSection,
            Token::Data(vec!["1".to_string(), "0".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "7".to_string(), "8".to_string()]),
            Token::Data(vec!["3".to_string(), "9".to_string(), "10".to_string()]),
            Token::DemandSection,
            Token::Data(vec!["1".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "11".to_string()]),
            Token::Data(vec!["3".to_string(), "12".to_string()]),
            Token::DepotSection,
            Token::Data(vec!["1".to_string()]),
        ];

        let value: SectionData<u64> = parse_tokens(&sut).unwrap();

        let expected = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_parse_tokens_fails_when_edge_weight_section_is_invalid() {
        let sut = vec![
            Token::Name("This is a name.".to_string()),
            Token::Comment("This is a comment.".to_string()),
            Token::Type("CVRP".to_string()),
            Token::Dimension("3".to_string()),
            Token::EdgeWeightType("EXPLICIT".to_string()),
            Token::EdgeWeightFormat("LOWER_ROW".to_string()),
            Token::NodeCoordType("TWOD_COORDS".to_string()),
            Token::Capacity("2".to_string()),
            Token::EdgeWeightSection,
            Token::Data(vec!["A".to_string()]), // not a number
            Token::Data(vec!["5".to_string(), "6".to_string()]),
            Token::NodeCoordSection,
            Token::Data(vec!["1".to_string(), "0".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "7".to_string(), "8".to_string()]),
            Token::Data(vec!["3".to_string(), "9".to_string(), "10".to_string()]),
            Token::DemandSection,
            Token::Data(vec!["1".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "11".to_string()]),
            Token::Data(vec!["3".to_string(), "12".to_string()]),
            Token::DepotSection,
            Token::Data(vec!["1".to_string()]),
        ];

        let value: Result<SectionData<u64>, _> = parse_tokens(&sut);
        let value = value.unwrap_err();

        let expected = ParseError::EdgeWeight;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_parse_tokens_fails_when_edge_node_coord_section_is_invalid() {
        let sut = vec![
            Token::Name("This is a name.".to_string()),
            Token::Comment("This is a comment.".to_string()),
            Token::Type("CVRP".to_string()),
            Token::Dimension("3".to_string()),
            Token::EdgeWeightType("EXPLICIT".to_string()),
            Token::EdgeWeightFormat("LOWER_ROW".to_string()),
            Token::NodeCoordType("TWOD_COORDS".to_string()),
            Token::Capacity("2".to_string()),
            Token::EdgeWeightSection,
            Token::Data(vec!["4".to_string()]),
            Token::Data(vec!["5".to_string(), "6".to_string()]),
            Token::NodeCoordSection,
            Token::Data(vec!["1".to_string(), "ABC".to_string(), "0".to_string()]), // not a number
            Token::Data(vec!["2".to_string(), "7".to_string(), "8".to_string()]),
            Token::Data(vec!["3".to_string(), "9".to_string(), "10".to_string()]),
            Token::DemandSection,
            Token::Data(vec!["1".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "11".to_string()]),
            Token::Data(vec!["3".to_string(), "12".to_string()]),
            Token::DepotSection,
            Token::Data(vec!["1".to_string()]),
        ];

        let value: Result<SectionData<u64>, _> = parse_tokens(&sut);
        let value = value.unwrap_err();

        let expected = ParseError::NodeCoord;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_parse_tokens_fails_when_edge_node_demand_section_is_invalid() {
        let sut = vec![
            Token::Name("This is a name.".to_string()),
            Token::Comment("This is a comment.".to_string()),
            Token::Type("CVRP".to_string()),
            Token::Dimension("3".to_string()),
            Token::EdgeWeightType("EXPLICIT".to_string()),
            Token::EdgeWeightFormat("LOWER_ROW".to_string()),
            Token::NodeCoordType("TWOD_COORDS".to_string()),
            Token::Capacity("2".to_string()),
            Token::EdgeWeightSection,
            Token::Data(vec!["4".to_string()]),
            Token::Data(vec!["5".to_string(), "6".to_string()]),
            Token::NodeCoordSection,
            Token::Data(vec!["1".to_string(), "0".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "7".to_string(), "8".to_string()]),
            Token::Data(vec!["3".to_string(), "9".to_string(), "10".to_string()]),
            Token::DemandSection,
            Token::Data(vec!["1".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "1A".to_string()]), // not a number
            Token::Data(vec!["3".to_string(), "12".to_string()]),
            Token::DepotSection,
            Token::Data(vec!["1".to_string()]),
        ];

        let value: Result<SectionData<u64>, _> = parse_tokens(&sut);
        let value = value.unwrap_err();

        let expected = ParseError::Demand;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_parse_tokens_fails_when_edge_node_depot_section_is_invalid() {
        let sut = vec![
            Token::Name("This is a name.".to_string()),
            Token::Comment("This is a comment.".to_string()),
            Token::Type("CVRP".to_string()),
            Token::Dimension("3".to_string()),
            Token::EdgeWeightType("EXPLICIT".to_string()),
            Token::EdgeWeightFormat("LOWER_ROW".to_string()),
            Token::NodeCoordType("TWOD_COORDS".to_string()),
            Token::Capacity("2".to_string()),
            Token::EdgeWeightSection,
            Token::Data(vec!["4".to_string()]),
            Token::Data(vec!["5".to_string(), "6".to_string()]),
            Token::NodeCoordSection,
            Token::Data(vec!["1".to_string(), "0".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "7".to_string(), "8".to_string()]),
            Token::Data(vec!["3".to_string(), "9".to_string(), "10".to_string()]),
            Token::DemandSection,
            Token::Data(vec!["1".to_string(), "0".to_string()]),
            Token::Data(vec!["2".to_string(), "1".to_string()]),
            Token::Data(vec!["3".to_string(), "12".to_string()]),
            Token::DepotSection,
            Token::Data(vec!["A1".to_string()]), // not a number
        ];

        let value: Result<SectionData<u64>, _> = parse_tokens(&sut);
        let value = value.unwrap_err();

        let expected = ParseError::Depot;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_succeeds() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Ok(());
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_name() {
        let sut = SectionData::<u64> {
            name: None,
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingName);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_problem_type() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: None,
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingProblemType);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_dimension() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: None,
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingDimension);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_edge_weight_type() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: None,
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingEdgeWeightType);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_edge_weight_format() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: None,
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingEdgeWeightFormat);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_edge_weights_are_invalid() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4, 5], vec![6]], // not lower row
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::EdgeWeight);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_node_coords_are_invalid() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![
                vec![0.0, 0.0, 0.0],
                vec![7.0, 8.0, 0.0],
                vec![9.0, 10.0, 0.0],
            ], // 3D coords
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::NodeCoord);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_node_coords_type_not_given() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: None,
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingNodeCoordType);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_node_coords_are_invalid_euc2d() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Euc2D),
            edge_weight_format: None,
            node_coord_type: None,
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![
                vec![0.0, 0.0, 0.0],
                vec![7.0, 8.0, 0.0],
                vec![9.0, 10.0, 0.0],
            ], // 3D coords
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::NodeCoord);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_succeeds_full_matrix() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::FullMatrix),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![0, 4, 5], vec![4, 0, 6], vec![5, 6, 0]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        assert_eq!(value, Ok(()));
    }

    #[test]
    fn test_validate_format_fails_if_full_matrix_edge_weights_are_invalid() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::FullMatrix),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![0, 4, 5], vec![4, 0, 6]], // missing one row
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![1],
        };

        let value = validate_format(&sut);

        assert_eq!(value, Err(ParseError::EdgeWeight));
    }

    #[test]
    fn test_validate_format_fails_if_demands_are_invalid() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11], // wrong count (2 instead of 3)
            depots: vec![1],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::Demand);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_depots_are_invalid() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![0.0, 0.0], vec![7.0, 8.0], vec![9.0, 10.0]],
            demands: vec![0, 11, 12],
            depots: vec![0], // out of range (depots are 1-indexed)
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::Depot);
        assert_eq!(value, expected);
    }
}
