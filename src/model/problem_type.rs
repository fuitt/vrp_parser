/// Represents the type of Vehicle Routing Problem (VRP).
///
/// - `Cvrp`: The Capacitated Vehicle Routing Problem, where each
///   customer has a demand and each vehicle has a capacity limit.
/// - `Cvrptw`: The Capacitated Vehicle Routing Problem with Time Windows,
///   where each customer additionally has a time window and a service time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProblemType {
    Cvrp,
    Cvrptw,
}
