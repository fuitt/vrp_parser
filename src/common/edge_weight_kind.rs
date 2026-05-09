/// Represents the unified edge‑weight representation used by this library.
///
/// In the VRPLib specification, edge weights are described using two separate
/// fields: `EDGE_WEIGHT_TYPE` and `EDGE_WEIGHT_FORMAT`. For explicit
/// edge‑weight matrices (`EDGE_WEIGHT_TYPE = EXPLICIT`), the
/// `EDGE_WEIGHT_FORMAT` field determines how the matrix is stored, including
/// various compressed layouts. For coordinate‑based types such as `EUC_2D` or
/// `GEO`, the edge weights are computed from node coordinates and the
/// `EDGE_WEIGHT_FORMAT` field is not used.
///
/// `EdgeWeightKind` provides a unified abstraction that merges these concepts
/// into a single representation used internally by this library, allowing the
/// loader and instance builder to treat all supported formats consistently.
///
/// Currently, only the `LowerRow` format is supported.
///
/// - `LowerRow`: The lower triangular part of a symmetric matrix is listed
///   row by row, excluding the diagonal. This corresponds to VRPLib’s
///   `EDGE_WEIGHT_FORMAT = LOWER_ROW` when used with `EDGE_WEIGHT_TYPE = EXPLICIT`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EdgeWeightKind {
    LowerRow,
}
