use crate::EdgeWeightKind;
use crate::Numeric;
use crate::ProblemType;
use crate::solomon::parser::SolomonData;
use crate::util::edge_weight::{
    edge_weight_by_euc2d, edge_weight_by_euc2d_f64, euclidean_distance,
};
use crate::util::matrix::expand_lower_row;
use crate::vrplib::parser::SectionData;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VrpInstanceBuilder<T> {
    name: String,
    problem_type: ProblemType,
    edge_weight_kind: EdgeWeightKind,
    dimension: usize,
    depots: Vec<usize>,
    capacity: Option<T>,
    demands: Option<Vec<T>>,
    node_coords: Option<Vec<(f64, f64)>>,
    edge_weights: Option<Vec<Vec<T>>>,
    time_windows: Option<Vec<(f64, f64)>>,
    service_times: Option<Vec<f64>>,
    vehicle_count: Option<usize>,
}

/// Represents a fully constructed Vehicle Routing Problem (VRP) instance
/// loaded from a VRPLib file.
///
/// A `VrpInstance<T>` contains all data required to describe a VRP, including
/// problem metadata, node information, distance or cost matrices, and
/// problem‑specific attributes such as vehicle capacity and customer demands.
/// The type parameter `T` determines the numeric representation used for values
/// such as edge weights and demands. Common choices include `u64` for standard
/// VRPLib instances.
///
/// This structure is created only after successful parsing and validation of
/// a VRPLib file. All fields therefore represent a semantically consistent
/// instance. Optional fields are present only for problem types that require
/// them (e.g., capacity and demands for [`ProblemType::Cvrp`]).
///
/// Edge weights are stored as a fully expanded matrix, regardless of the
/// original VRPLib representation.
///
/// # Type Parameters
/// - `T`: The numeric type representing values such as edge weights and demands.
///   Typically:
///   - `u64`: The standard VRPLib-compliant integer representation.
///   - `f64`: A floating-point representation for fractional weights and demands.
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
/// A `VrpInstance` is immutable after construction and can be used directly
/// by solvers, heuristics, or analysis tools.
#[derive(Debug, Clone, PartialEq)]
pub struct VrpInstance<T> {
    name: String,
    problem_type: ProblemType,
    dimension: usize,
    depots: Vec<usize>,
    edge_weights: Vec<Vec<T>>,
    capacity: Option<T>,
    demands: Option<Vec<T>>,
    node_coords: Option<Vec<(f64, f64)>>,
    time_windows: Option<Vec<(f64, f64)>>,
    service_times: Option<Vec<f64>>,
    vehicle_count: Option<usize>,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
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
    #[error("missing time windows")]
    MissingTimeWindows,
    #[error("missing service times")]
    MissingServiceTimes,
    #[error("invalid demands length")]
    InvalidDemandsLength,
}

impl<T: Numeric> VrpInstanceBuilder<T> {
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
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        }
    }

    pub fn depots(mut self, depots: Vec<usize>) -> Self {
        self.depots = depots;
        self
    }

    pub fn capacity(mut self, capacity: Option<T>) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn demands(mut self, demands: Option<Vec<T>>) -> Self {
        self.demands = demands;
        self
    }

    pub fn node_coords(mut self, node_coords: Option<Vec<(f64, f64)>>) -> Self {
        self.node_coords = node_coords;
        self
    }

    pub fn edge_weights(mut self, edge_weights: Option<Vec<Vec<T>>>) -> Self {
        self.edge_weights = edge_weights;
        self
    }

    pub fn time_windows(mut self, time_windows: Option<Vec<(f64, f64)>>) -> Self {
        self.time_windows = time_windows;
        self
    }

    pub fn service_times(mut self, service_times: Option<Vec<f64>>) -> Self {
        self.service_times = service_times;
        self
    }

    pub fn vehicle_count(mut self, vehicle_count: Option<usize>) -> Self {
        self.vehicle_count = vehicle_count;
        self
    }

    pub(crate) fn make_from_vrplib(section_data: SectionData<T>) -> Self {
        let name = section_data.name.unwrap();
        let edge_weight_kind = EdgeWeightKind::new(
            section_data.edge_weight_type.unwrap(),
            section_data.edge_weight_format,
        )
        .unwrap();
        let depots = section_data.depots;
        let demands = section_data.demands;
        let node_coords = section_data
            .node_coords
            .iter()
            .map(|coords| (coords[0], coords[1]))
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

    fn validate_demands(&self) -> Result<(), ValidationError> {
        if let Some(demands) = &self.demands
            && demands.len() != self.dimension
        {
            return Err(ValidationError::InvalidDemandsLength);
        }
        Ok(())
    }
}

impl VrpInstanceBuilder<f64> {
    pub(crate) fn make_from_solomon(data: SolomonData) -> Self {
        let dimension = data.node_coords.len();
        Self::new(
            data.name,
            ProblemType::Cvrptw,
            dimension,
            EdgeWeightKind::Euc2DExact,
        )
        .depots(vec![0])
        .capacity(Some(data.capacity))
        .demands(Some(data.demands))
        .node_coords(Some(data.node_coords))
        .time_windows(Some(data.time_windows))
        .service_times(Some(data.service_times))
        .vehicle_count(Some(data.vehicle_count))
    }
}

impl VrpInstanceBuilder<u64> {
    pub fn build(mut self) -> Result<VrpInstance<u64>, ValidationError> {
        let edge_weights = match self.edge_weight_kind {
            EdgeWeightKind::LowerRow => {
                self.edge_weights
                    .as_ref()
                    .ok_or(ValidationError::MissingEdgeWeight)?;
                expand_lower_row(self.edge_weights.as_ref().unwrap())
            }
            EdgeWeightKind::FullMatrix => self
                .edge_weights
                .take()
                .ok_or(ValidationError::MissingEdgeWeight)?,
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
            EdgeWeightKind::Euc2DExact => unreachable!("Euc2DExact produces f64 weights"),
        };

        if self.depots.is_empty() {
            return Err(ValidationError::MissingDepots);
        }

        match self.problem_type {
            ProblemType::Cvrp => {
                self.capacity
                    .as_ref()
                    .ok_or(ValidationError::MissingCapacity)?;
                self.demands
                    .as_ref()
                    .ok_or(ValidationError::MissingDemands)?;

                self.validate_demands()?;

                Ok(VrpInstance::new(
                    self.name,
                    self.problem_type,
                    self.dimension,
                    self.depots,
                    edge_weights,
                    self.capacity,
                    self.demands,
                    self.node_coords,
                    None,
                    None,
                    None,
                ))
            }
            ProblemType::Cvrptw => unreachable!("CVRPTW instances use f64"),
        }
    }
}

impl VrpInstanceBuilder<f64> {
    pub fn build(mut self) -> Result<VrpInstance<f64>, ValidationError> {
        let edge_weights = match self.edge_weight_kind {
            EdgeWeightKind::LowerRow => {
                self.edge_weights
                    .as_ref()
                    .ok_or(ValidationError::MissingEdgeWeight)?;
                expand_lower_row(self.edge_weights.as_ref().unwrap())
            }
            EdgeWeightKind::FullMatrix => self
                .edge_weights
                .take()
                .ok_or(ValidationError::MissingEdgeWeight)?,
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
                        .map(|&q| edge_weight_by_euc2d_f64(p, q))
                        .collect()
                })
                .collect(),
            EdgeWeightKind::Euc2DExact => self
                .node_coords
                .as_ref()
                .ok_or(ValidationError::MissingNodeCoords)?
                .iter()
                .map(|&p| {
                    self.node_coords
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|&q| euclidean_distance(p, q))
                        .collect()
                })
                .collect(),
        };

        if self.depots.is_empty() {
            return Err(ValidationError::MissingDepots);
        }

        match self.problem_type {
            ProblemType::Cvrp => {
                self.capacity
                    .as_ref()
                    .ok_or(ValidationError::MissingCapacity)?;
                self.demands
                    .as_ref()
                    .ok_or(ValidationError::MissingDemands)?;

                self.validate_demands()?;

                Ok(VrpInstance::new(
                    self.name,
                    self.problem_type,
                    self.dimension,
                    self.depots,
                    edge_weights,
                    self.capacity,
                    self.demands,
                    self.node_coords,
                    None,
                    None,
                    None,
                ))
            }
            ProblemType::Cvrptw => {
                self.capacity
                    .as_ref()
                    .ok_or(ValidationError::MissingCapacity)?;
                self.demands
                    .as_ref()
                    .ok_or(ValidationError::MissingDemands)?;
                self.time_windows
                    .as_ref()
                    .ok_or(ValidationError::MissingTimeWindows)?;
                self.service_times
                    .as_ref()
                    .ok_or(ValidationError::MissingServiceTimes)?;

                self.validate_demands()?;

                Ok(VrpInstance::new(
                    self.name,
                    self.problem_type,
                    self.dimension,
                    self.depots,
                    edge_weights,
                    self.capacity,
                    self.demands,
                    self.node_coords,
                    self.time_windows,
                    self.service_times,
                    self.vehicle_count,
                ))
            }
        }
    }
}

impl<T> VrpInstance<T> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        name: String,
        problem_type: ProblemType,
        dimension: usize,
        depots: Vec<usize>,
        edge_weights: Vec<Vec<T>>,
        capacity: Option<T>,
        demands: Option<Vec<T>>,
        node_coords: Option<Vec<(f64, f64)>>,
        time_windows: Option<Vec<(f64, f64)>>,
        service_times: Option<Vec<f64>>,
        vehicle_count: Option<usize>,
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
            time_windows,
            service_times,
            vehicle_count,
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
    pub fn node_coords(&self) -> &Option<Vec<(f64, f64)>> {
        &self.node_coords
    }

    /// Returns the coordinates for the given node, if coordinates are defined.
    pub fn get_node_coord(&self, node: usize) -> Option<&(f64, f64)> {
        self.node_coords.as_ref()?.get(node)
    }

    /// Returns the demand value for each node, if applicable.
    ///
    /// This field is present for problem types that require customer demands,
    /// such as [`ProblemType::Cvrp`]. For other problem types, it is `None`.
    pub fn demands(&self) -> &Option<Vec<T>> {
        &self.demands
    }

    /// Returns the demand for the given node, if demands are defined.
    pub fn get_demand(&self, node: usize) -> Option<&T> {
        self.demands.as_ref()?.get(node)
    }

    /// Returns the vehicle capacity, if defined for this instance.
    ///
    /// Capacity is required for capacitated VRP variants such as
    /// [`ProblemType::Cvrp`]. For problem types without capacity constraints,
    /// this field is `None`.
    pub fn capacity(&self) -> &Option<T> {
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
    pub fn edge_weights(&self) -> &[Vec<T>] {
        &self.edge_weights
    }

    /// Returns the edge weight between two nodes, or `None` if either index is out of bounds.
    pub fn get_edge_weight(&self, from: usize, to: usize) -> Option<&T> {
        self.edge_weights.get(from)?.get(to)
    }

    /// Returns the time windows `(ready_time, due_date)` for each node, if defined.
    ///
    /// Present for [`ProblemType::Cvrptw`] instances (e.g., Solomon format).
    /// `None` for problem types without time windows.
    pub fn time_windows(&self) -> &Option<Vec<(f64, f64)>> {
        &self.time_windows
    }

    /// Returns the time window for a specific node, or `None` if not defined or out of bounds.
    pub fn get_time_window(&self, node: usize) -> Option<&(f64, f64)> {
        self.time_windows.as_ref()?.get(node)
    }

    /// Returns the service time for each node, if defined.
    ///
    /// Present for [`ProblemType::Cvrptw`] instances. `None` otherwise.
    pub fn service_times(&self) -> &Option<Vec<f64>> {
        &self.service_times
    }

    /// Returns the service time for a specific node, or `None` if not defined or out of bounds.
    pub fn get_service_time(&self, node: usize) -> Option<&f64> {
        self.service_times.as_ref()?.get(node)
    }

    /// Returns the number of available vehicles, if specified by the instance format.
    ///
    /// Solomon instances include a vehicle count. VRPLib instances do not, so this is `None`.
    pub fn vehicle_count(&self) -> Option<usize> {
        self.vehicle_count
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::vrplib::types::edge_weight_format::EdgeWeightFormat;
    use crate::vrplib::types::edge_weight_type::EdgeWeightType;
    use crate::vrplib::types::node_coord_type::NodeCoordType;

    fn build_instance_with_edge_weights() -> VrpInstance<u64> {
        VrpInstance {
            name: "test".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            depots: vec![0],
            capacity: None,
            demands: None,
            node_coords: None,
            edge_weights: vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]],
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        }
    }

    #[test]
    fn test_get_edge_weight_returns_weight_for_valid_nodes() {
        let sut = build_instance_with_edge_weights();
        assert_eq!(sut.get_edge_weight(0, 2), Some(&2));
    }

    #[test]
    fn test_get_edge_weight_returns_none_for_out_of_bounds_from() {
        let sut = build_instance_with_edge_weights();
        assert_eq!(sut.get_edge_weight(99, 0), None);
    }

    #[test]
    fn test_get_edge_weight_returns_none_for_out_of_bounds_to() {
        let sut = build_instance_with_edge_weights();
        assert_eq!(sut.get_edge_weight(0, 99), None);
    }

    fn build_instance_with_coords() -> VrpInstance<u64> {
        VrpInstance {
            name: "test".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            depots: vec![0],
            capacity: Some(10),
            demands: Some(vec![0, 5, 8]),
            node_coords: Some(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]),
            edge_weights: vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]],
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        }
    }

    #[test]
    fn test_get_node_coord_returns_coord_for_valid_node() {
        let sut = build_instance_with_coords();
        assert_eq!(sut.get_node_coord(1), Some(&(3.0, 4.0)));
    }

    #[test]
    fn test_get_node_coord_returns_none_for_out_of_bounds_node() {
        let sut = build_instance_with_coords();
        assert_eq!(sut.get_node_coord(99), None);
    }

    #[test]
    fn test_get_node_coord_returns_none_when_coords_absent() {
        let sut = VrpInstance::<u64> {
            name: "test".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            depots: vec![0],
            capacity: None,
            demands: None,
            node_coords: None,
            edge_weights: vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]],
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };
        assert_eq!(sut.get_node_coord(0), None);
    }

    fn build_instance_with_demands() -> VrpInstance<u64> {
        VrpInstance {
            name: "test".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            depots: vec![0],
            capacity: Some(10),
            demands: Some(vec![0, 5, 8]),
            node_coords: None,
            edge_weights: vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]],
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        }
    }

    #[test]
    fn test_get_demand_returns_demand_for_valid_node() {
        let sut = build_instance_with_demands();
        assert_eq!(sut.get_demand(1), Some(&5));
    }

    #[test]
    fn test_get_demand_returns_none_for_out_of_bounds_node() {
        let sut = build_instance_with_demands();
        assert_eq!(sut.get_demand(99), None);
    }

    #[test]
    fn test_get_demand_returns_none_when_demands_absent() {
        let sut = VrpInstance {
            name: "test".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            depots: vec![0],
            capacity: None,
            demands: None,
            node_coords: None,
            edge_weights: vec![vec![0u64, 1, 2], vec![1, 0, 3], vec![2, 3, 0]],
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };
        assert_eq!(sut.get_demand(0), None);
    }

    #[test]
    fn test_make_from_vrplib() {
        let sut = SectionData::<u64> {
            name: Some("This is a name.".to_string()),
            problem_type: Some(ProblemType::Cvrp),
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

        let value = VrpInstanceBuilder::make_from_vrplib(sut);

        let expected = VrpInstanceBuilder::<u64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: Some(2),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_succeeds() {
        let sut = VrpInstanceBuilder::<u64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: Some(2),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };

        let value = sut.build().unwrap();

        let expected = VrpInstance {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            depots: vec![1],
            capacity: Some(2u64),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: vec![vec![0, 4, 5], vec![4, 0, 6], vec![5, 6, 0]],
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_depots() {
        let sut = VrpInstanceBuilder::<u64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![], // is empty
            capacity: Some(2),
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingDepots;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_demands() {
        let sut = VrpInstanceBuilder::<u64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: Some(2),
            demands: None, // not given
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingDemands;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_capacity() {
        let sut = VrpInstanceBuilder::<u64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: None, // not given
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: Some(vec![vec![4], vec![5, 6]]),
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingCapacity;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_edge_weights() {
        let sut = VrpInstanceBuilder::<u64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: None,
            demands: Some(vec![0, 11, 12]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: None, // not given
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingEdgeWeight;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_fails_if_missing_node_coords() {
        let sut = VrpInstanceBuilder::<u64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::Euc2D,
            depots: vec![1],
            capacity: None,
            demands: Some(vec![0, 11, 12]),
            node_coords: None, // not given
            edge_weights: None,
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };

        let value = sut.build().unwrap_err();

        let expected = ValidationError::MissingNodeCoords;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_build_f64_succeeds() {
        let sut = VrpInstanceBuilder::<f64> {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            edge_weight_kind: EdgeWeightKind::LowerRow,
            depots: vec![1],
            capacity: Some(2.0),
            demands: Some(vec![0.0, 11.0, 12.0]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: Some(vec![vec![4.0], vec![5.0, 6.0]]),
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };

        let value = sut.build().unwrap();

        let expected = VrpInstance {
            name: "This is a name.".to_string(),
            problem_type: ProblemType::Cvrp,
            dimension: 3,
            depots: vec![1],
            capacity: Some(2.0f64),
            demands: Some(vec![0.0, 11.0, 12.0]),
            node_coords: Some(vec![(0.0, 0.0), (7.0, 8.0), (9.0, 10.0)]),
            edge_weights: vec![
                vec![0.0, 4.0, 5.0],
                vec![4.0, 0.0, 6.0],
                vec![5.0, 6.0, 0.0],
            ],
            time_windows: None,
            service_times: None,
            vehicle_count: None,
        };
        assert_eq!(value, expected);
    }
}
