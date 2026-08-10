/// Represents the type of Vehicle Routing Problem (VRP) described
/// by a VRPLib instance.
///
/// VRPLib supports multiple problem classes (e.g., CVRP, VRPTW),
/// but this library currently implements only the Capacitated VRP.
///
/// - `CVRP`: The Capacitated Vehicle Routing Problem, where each
///   customer has a demand and each vehicle has a capacity limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProblemType {
    CVRP,
}
