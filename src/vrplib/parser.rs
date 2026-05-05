use crate::ProblemType;

use super::NodeCoordType;
use super::Token;
use super::edge_weight_format::EdgeWeightFormat;
use super::edge_weight_type::EdgeWeightType;

enum State {
    Header,
    EdgeWeightSection,
    NodeCoordSection,
    DemandSection,
    DepotSection,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct SectionData {
    pub name: Option<String>,
    pub problem_type: Option<ProblemType>,
    pub dimension: Option<usize>,
    pub capacity: Option<u64>,
    pub edge_weight_type: Option<EdgeWeightType>,
    pub edge_weight_format: Option<EdgeWeightFormat>,
    pub edge_weights: Vec<Vec<u64>>,
    pub node_coord_type: Option<NodeCoordType>,
    pub node_coords: Vec<Vec<u64>>,
    pub demands: Vec<Vec<u64>>,
    pub depots: Vec<Vec<u64>>,
}

#[derive(Debug, PartialEq, thiserror::Error)]
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

    #[error("invalid edge weight")]
    EdgeWeight,

    #[error("invalid node coord")]
    NodeCoord,

    #[error("invalid demand")]
    Demand,

    #[error("invalid depot")]
    Depot,
}

pub(crate) fn parse(tokens: &[Token]) -> Result<SectionData, ParseError> {
    let section_data = parse_tokens(tokens);
    validate_format(&section_data)?;
    Ok(section_data)
}

fn parse_tokens(tokens: &[Token]) -> SectionData {
    let mut data = SectionData::default();

    let mut state = State::Header;
    for token in tokens.iter() {
        match token {
            Token::Name(s) => data.name = Some(s.to_string()),
            Token::Comment(_) => {}
            Token::Type(t) => data.problem_type = Some(*t),
            Token::Dimension(d) => data.dimension = Some(*d),
            Token::Capacity(c) => data.capacity = Some(*c),
            Token::EdgeWeightType(t) => data.edge_weight_type = Some(*t),
            Token::EdgeWeightFormat(f) => data.edge_weight_format = Some(*f),
            Token::NodeCoordType(t) => data.node_coord_type = Some(*t),
            Token::EdgeWeightSection => state = State::EdgeWeightSection,
            Token::NodeCoordSection => state = State::NodeCoordSection,
            Token::DemandSection => state = State::DemandSection,
            Token::DepotSection => state = State::DepotSection,
            Token::Data(d) => match state {
                State::Header => {}
                State::EdgeWeightSection => data.edge_weights.push(d.clone()),
                State::NodeCoordSection => data.node_coords.push(d.clone()),
                State::DemandSection => data.demands.push(d.clone()),
                State::DepotSection => data.depots.push(d.clone()),
            },
            Token::Eof => {}
        }
    }
    data
}

fn validate_format(section_data: &SectionData) -> Result<(), ParseError> {
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
    section_data
        .edge_weight_format
        .as_ref()
        .ok_or(ParseError::MissingEdgeWeightFormat)?;

    let dimension = section_data.dimension.unwrap();
    let edge_weight_type = section_data.edge_weight_type.unwrap();
    let edge_weight_format = section_data.edge_weight_format.unwrap();

    validate_edge_weights(
        dimension,
        edge_weight_type,
        edge_weight_format,
        &section_data.edge_weights,
    )?;
    validate_node_coords(
        dimension,
        &section_data.node_coord_type,
        &section_data.node_coords,
    )?;
    validate_demands(dimension, &section_data.demands)?;
    validate_depots(dimension, &section_data.depots)?;
    Ok(())
}

fn validate_edge_weights(
    dimension: usize,
    edge_weight_type: EdgeWeightType,
    edge_weight_format: EdgeWeightFormat,
    edge_weights: &[Vec<u64>],
) -> Result<(), ParseError> {
    match edge_weight_type {
        EdgeWeightType::Explicit => match edge_weight_format {
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
        },
    }
}

fn validate_node_coords(
    dimension: usize,
    node_coord_type: &Option<NodeCoordType>,
    node_coords: &[Vec<u64>],
) -> Result<(), ParseError> {
    match node_coord_type {
        Some(coord_type) => match coord_type {
            NodeCoordType::TwodCoords => {
                if dimension == node_coords.len()
                    && node_coords
                        .iter()
                        .enumerate()
                        .all(|(i, coords)| coords.len() == 3 && i + 1 == (coords[0] as usize))
                {
                    Ok(())
                } else {
                    Err(ParseError::NodeCoord)
                }
            }
        },
        None => {
            if node_coords.is_empty() {
                Ok(())
            } else {
                Err(ParseError::NodeCoord)
            }
        }
    }
}

fn validate_demands(dimension: usize, demands: &[Vec<u64>]) -> Result<(), ParseError> {
    if dimension == demands.len()
        && demands
            .iter()
            .enumerate()
            .all(|(i, demand)| demand.len() == 2 && i + 1 == (demand[0] as usize))
    {
        Ok(())
    } else {
        Err(ParseError::Demand)
    }
}

fn validate_depots(dimension: usize, depots: &[Vec<u64>]) -> Result<(), ParseError> {
    if depots
        .iter()
        .all(|depot| depot.len() == 1 && 1 <= depot[0] && depot[0] <= (dimension as u64))
    {
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
            Token::Type(ProblemType::CVRP),
            Token::Dimension(3),
            Token::EdgeWeightType(EdgeWeightType::Explicit),
            Token::EdgeWeightFormat(EdgeWeightFormat::LowerRow),
            Token::NodeCoordType(NodeCoordType::TwodCoords),
            Token::Capacity(2),
            Token::EdgeWeightSection,
            Token::Data(vec![4]),
            Token::Data(vec![5, 6]),
            Token::NodeCoordSection,
            Token::Data(vec![1, 0, 0]),
            Token::Data(vec![2, 7, 8]),
            Token::Data(vec![3, 9, 10]),
            Token::DemandSection,
            Token::Data(vec![1, 0]),
            Token::Data(vec![2, 11]),
            Token::Data(vec![3, 12]),
            Token::DepotSection,
            Token::Data(vec![1]),
        ];

        let value = parse_tokens(&sut);

        let expected = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_succeeds() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Ok(());
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_name() {
        let sut = SectionData {
            name: None,
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingName);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_problem_type() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: None,
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingProblemType);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_dimension() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: None,
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingDimension);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_edge_weight_type() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: None,
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingEdgeWeightType);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_missing_edge_weight_format() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: None,
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::MissingEdgeWeightFormat);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_edge_weights_are_invalid() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4, 5], vec![6]], // not lower row
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::EdgeWeight);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_node_coords_are_invalid() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0, 0], vec![2, 7, 8, 0], vec![3, 9, 10, 0]], // 3D coords
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::NodeCoord);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_demands_are_invalid() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12, 0]], // len != 2
            depots: vec![vec![1]],
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::Demand);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_validate_format_fails_if_depots_are_invalid() {
        let sut = SectionData {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::CVRP),
            dimension: Some(3),
            edge_weight_type: Some(EdgeWeightType::Explicit),
            edge_weight_format: Some(EdgeWeightFormat::LowerRow),
            node_coord_type: Some(NodeCoordType::TwodCoords),
            capacity: Some(2),
            edge_weights: vec![vec![4], vec![5, 6]],
            node_coords: vec![vec![1, 0, 0], vec![2, 7, 8], vec![3, 9, 10]],
            demands: vec![vec![1, 0], vec![2, 11], vec![3, 12]],
            depots: vec![vec![]], // is empty
        };

        let value = validate_format(&sut);

        let expected = Err(ParseError::Depot);
        assert_eq!(value, expected);
    }
}
