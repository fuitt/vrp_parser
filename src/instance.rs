use crate::EdgeWeightKind;
use crate::ProblemType;
use crate::common::matrix::expand_lower_row;
use crate::vrplib::{
    edge_weight_format::EdgeWeightFormat, edge_weight_type::EdgeWeightType, parser::SectionData,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VRPInstanceBuilder {
    name: String,
    problem_type: ProblemType,
    edge_weight_kind: EdgeWeightKind,
    dimension: usize,
    depots: Vec<usize>,
    capacity: Option<u64>,
    demands: Option<Vec<u64>>,
    node_coords: Option<Vec<(u64, u64)>>,
    edge_weights: Option<Vec<Vec<u64>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VRPInstance {
    name: String,
    problem_type: ProblemType,
    dimension: usize,
    depots: Vec<usize>,
    edge_weights: Vec<Vec<u64>>,
    capacity: Option<u64>,
    demands: Option<Vec<u64>>,
    node_coords: Option<Vec<(u64, u64)>>,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ValidationError {
    #[error("missing capacity")]
    MissingCapacity,
    #[error("missing demands")]
    MissingDemands,
    #[error("missing depots")]
    MissingDepots,
    #[error("missing edge weight")]
    MissingEdgeWeight,
    #[error("invalid demands length")]
    InvalidDemandsLength,
}

impl VRPInstanceBuilder {
    pub fn new(
        name: String,
        problem_type: ProblemType,
        dimension: usize,
        edge_weight_kind: EdgeWeightKind,
    ) -> Self {
        Self {
            name,
            problem_type,
            edge_weight_kind,
            dimension,
            depots: vec![],
            capacity: None,
            demands: None,
            node_coords: None,
            edge_weights: None,
        }
    }

    pub fn depots(mut self, depots: Vec<usize>) -> Self {
        self.depots = depots;
        self
    }

    pub fn capacity(mut self, capacity: Option<u64>) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn demands(mut self, demands: Option<Vec<u64>>) -> Self {
        self.demands = demands;
        self
    }

    pub fn node_coords(mut self, node_coords: Option<Vec<(u64, u64)>>) -> Self {
        self.node_coords = node_coords;
        self
    }
    pub fn edge_weights(mut self, edge_weights: Option<Vec<Vec<u64>>>) -> Self {
        self.edge_weights = edge_weights;
        self
    }

    pub fn build(self) -> Result<VRPInstance, ValidationError> {
        let edge_weights = match self.edge_weight_kind {
            EdgeWeightKind::LowerRow => {
                self.edge_weights
                    .as_ref()
                    .ok_or(ValidationError::MissingEdgeWeight)?;
                expand_lower_row(self.edge_weights.as_ref().unwrap())
            }
        };

        if self.depots.is_empty() {
            return Err(ValidationError::MissingDepots);
        }

        match self.problem_type {
            ProblemType::CVRP => {
                self.capacity
                    .as_ref()
                    .ok_or(ValidationError::MissingCapacity)?;
                self.demands
                    .as_ref()
                    .ok_or(ValidationError::MissingDemands)?;

                self.validate_demands()?;

                Ok(VRPInstance::new(
                    self.name,
                    self.problem_type,
                    self.dimension,
                    self.depots,
                    edge_weights,
                    self.capacity,
                    self.demands,
                    self.node_coords,
                ))
            }
        }
    }

    fn validate_demands(&self) -> Result<(), ValidationError> {
        if let Some(demands) = &self.demands
            && demands.len() != self.dimension
        {
            return Err(ValidationError::InvalidDemandsLength);
        }
        Ok(())
    }

    pub(crate) fn make_from_vrplib(section_data: SectionData) -> Self {
        let name = section_data.name.unwrap();
        let edge_weight_kind = match (
            section_data.edge_weight_type.unwrap(),
            section_data.edge_weight_format.unwrap(),
        ) {
            (EdgeWeightType::Explicit, EdgeWeightFormat::LowerRow) => EdgeWeightKind::LowerRow,
        };
        let depots = section_data
            .depots
            .iter()
            .map(|depot| depot[0] as usize)
            .collect();
        let demands = section_data
            .demands
            .iter()
            .map(|demand| demand[1])
            .collect();
        let node_coords = section_data
            .node_coords
            .iter()
            .map(|coords| (coords[1], coords[2]))
            .collect();

        Self::new(
            name,
            section_data.problem_type.unwrap(),
            section_data.dimension.unwrap(),
            edge_weight_kind,
        )
        .depots(depots)
        .capacity(section_data.capacity)
        .demands(Some(demands))
        .node_coords(Some(node_coords))
        .edge_weights(Some(section_data.edge_weights))
    }
}

impl VRPInstance {
    #[allow(clippy::too_many_arguments)]
    fn new(
        name: String,
        problem_type: ProblemType,
        dimension: usize,
        depots: Vec<usize>,
        edge_weights: Vec<Vec<u64>>,
        capacity: Option<u64>,
        demands: Option<Vec<u64>>,
        node_coords: Option<Vec<(u64, u64)>>,
    ) -> Self {
        Self {
            name,
            problem_type,
            dimension,
            depots,
            edge_weights,
            capacity,
            demands,
            node_coords,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn problem_type(&self) -> ProblemType {
        self.problem_type
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn node_coords(&self) -> &Option<Vec<(u64, u64)>> {
        &self.node_coords
    }

    pub fn demands(&self) -> &Option<Vec<u64>> {
        &self.demands
    }

    pub fn capacity(&self) -> &Option<u64> {
        &self.capacity
    }

    pub fn depots(&self) -> &[usize] {
        &self.depots
    }

    pub fn edge_weights(&self) -> &[Vec<u64>] {
        &self.edge_weights
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::vrplib::node_coord_type::NodeCoordType;

    #[test]
    fn test_make_from_vrplib() {
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

        let value = VRPInstanceBuilder::make_from_vrplib(sut);

        let expected = VRPInstanceBuilder {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: Some(2),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_succeeds() {
        let sut = VRPInstanceBuilder {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: Some(2),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
        };

        let value = sut.build().unwrap();

        let expected = VRPInstance {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            depots: vec![1],
            capacity: Some(2),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: vec![vec![0, 4, 5], vec![4, 0, 6], vec![5, 6, 0]],
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_depots() {
        let sut = VRPInstanceBuilder {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![], // is empty
            capacity: Some(2),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingDepots;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_demands() {
        let sut = VRPInstanceBuilder {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: Some(2),
            demands: None, // not given
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingDemands;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_capacity() {
        let sut = VRPInstanceBuilder {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: None, // not given
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingCapacity;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_edge_weights() {
        let sut = VRPInstanceBuilder {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: None, // not given
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: None,
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingEdgeWeight;
        assert_eq!(value, expected);
    }
}
