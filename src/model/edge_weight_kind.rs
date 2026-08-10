use crate::vrplib::edge_weight_format::EdgeWeightFormat;
use crate::vrplib::edge_weight_type::EdgeWeightType;

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
/// Currently, only the `LowerRow` and `Euc2D` format is supported.
///
/// - `LowerRow`: The lower triangular part of a symmetric matrix is listed
///   row by row, excluding the diagonal. This corresponds to VRPLib's
///   `EDGE_WEIGHT_FORMAT = LOWER_ROW` when used with `EDGE_WEIGHT_TYPE = EXPLICIT`.
/// - `Euc2D`: The edge weights are computed as the Euclidean distance between
///   2D coordinates. This corresponds to VRPLib's `EDGE_WEIGHT_TYPE = EUC_2D` with no
///   `EDGE_WEIGHT_FORMAT`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeWeightKind {
    LowerRow,
    FullMatrix,
    Euc2D,
}

impl EdgeWeightKind {
    pub(crate) fn new(
        edge_weight_type: EdgeWeightType,
        edge_weight_format: Option<EdgeWeightFormat>,
    ) -> Option<Self> {
        match edge_weight_type {
            EdgeWeightType::Explicit => {
                if let Some(fmt) = edge_weight_format {
                    match fmt {
                        EdgeWeightFormat::LowerRow => return Some(EdgeWeightKind::LowerRow),
                        EdgeWeightFormat::FullMatrix => return Some(EdgeWeightKind::FullMatrix),
                    }
                }
            }
            EdgeWeightType::Euc2D => {
                if edge_weight_format.is_none() {
                    return Some(EdgeWeightKind::Euc2D);
                }
            }
        }
        None
    }
}
