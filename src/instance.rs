use crate::EdgeWeightKind;
use crate::ProblemType;
use crate::common::edge_weight::edge_weight_by_euc2d;
use crate::common::matrix::expand_lower_row;
use crate::vrplib::parser::SectionData;

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

/// Represents a fully constructed Vehicle Routing Problem (VRP) instance
/// loaded from a VRPLib file.
///
/// A `VRPInstance` contains all data required to describe a VRP, including
/// problem metadata, node information, distance or cost matrices, and
/// problem‑specific attributes such as vehicle capacity and customer demands.
///
/// This structure is created only after successful parsing and validation of
/// a VRPLib file. All fields therefore represent a semantically consistent
/// instance. Optional fields are present only for problem types that require
/// them (e.g., capacity and demands for [`ProblemType::CVRP`]).
///
/// Edge weights are stored as a fully expanded matrix, regardless of the
/// original VRPLib representation.
///
/// # Fields
/// - `name`: The instance name as specified in the VRPLib file.
/// - `problem_type`: The VRP variant (see [`ProblemType`]).
/// - `dimension`: The number of nodes in the instance.
/// - `depots`: Indices of depot nodes.
/// - `edge_weights`: A fully expanded distance or cost matrix.
/// - `capacity`: Vehicle capacity (if applicable).
/// - `demands`: Customer demands for each node (if applicable).
/// - `node_coords`: Node coordinates (if provided in the VRPLib file).
///
/// A `VRPInstance` is immutable after construction and can be used directly
/// by solvers, heuristics, or analysis tools.
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
    #[error("missing node coords")]
    MissingNodeCoords,
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
            EdgeWeightKind::Euc2D => self
                .node_coords
                .as_ref()
                .ok_or(ValidationError::MissingNodeCoords)?
                .iter()
                .map(|&p| {
                    self.node_coords
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|&q| edge_weight_by_euc2d(p, q))
                        .collect()
                })
                .collect(),
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
        let edge_weight_kind = EdgeWeightKind::new(
            section_data.edge_weight_type.unwrap(),
            section_data.edge_weight_format,
        )
        .unwrap();
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

    /// Returns the name of the instance as specified in the VRPLib file
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the problem type of this instance.
    ///
    /// See [`ProblemType`] for supported variants.
    pub fn problem_type(&self) -> ProblemType {
        self.problem_type
    }

    /// Returns the number of nodes in the instance (`DIMENSION` in VRPLib).
    pub fn dimension(&self) -> usize {
        self.dimension
    }
    /// Returns the list of node coordinates, if provided.
    ///
    /// Coordinate‑based VRPLib instances (e.g., `EUC_2D`, `GEO`) include
    /// coordinates for each node. For explicit edge‑weight matrices, this
    /// field is `None`.
    pub fn node_coords(&self) -> &Option<Vec<(u64, u64)>> {
        &self.node_coords
    }

    /// Returns the demand value for each node, if applicable.
    ///
    /// This field is present for problem types that require customer demands,
    /// such as [`ProblemType::CVRP`]. For other problem types, it is `None`.
    pub fn demands(&self) -> &Option<Vec<u64>> {
        &self.demands
    }

    /// Returns the vehicle capacity, if defined for this instance.
    ///
    /// Capacity is required for capacitated VRP variants such as
    /// [`ProblemType::CVRP`]. For problem types without capacity constraints,
    /// this field is `None`.
    pub fn capacity(&self) -> &Option<u64> {
        &self.capacity
    }

    /// Returns the indices of depot nodes.
    ///
    /// VRPLib allows multiple depots.
    /// The indices refer to node positions in the instance.
    pub fn depots(&self) -> &[usize] {
        &self.depots
    }

    /// Returns the fully expanded edge‑weight matrix.
    ///
    /// Regardless of the original VRPLib representation (explicit matrix,
    /// compressed format, or coordinate‑based type), this method returns a
    /// complete `dimension × dimension` matrix of edge weights.
    pub fn edge_weights(&self) -> &[Vec<u64>] {
        &self.edge_weights
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::vrplib::edge_weight_format::EdgeWeightFormat;
    use crate::vrplib::edge_weight_type::EdgeWeightType;
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
            capacity: None,
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0, 0), (7, 8), (9, 10)]),
            edge_weights: None, // not given
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingEdgeWeight;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_node_coords() {
        let sut = VRPInstanceBuilder {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::CVRP,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::Euc2D,
            depots: vec![1],
            capacity: None,
            demands: Some(vec![0, 11, 12]),
            node_coords: None, // not given
            edge_weights: None,
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingNodeCoords;
        assert_eq!(value, expected);
    }
}
