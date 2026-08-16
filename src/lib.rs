//! # vrplib
//!
//! A library for loading VRPLib-formatted files and representing them as VRP instances.
//!
//! ## Features
//! - Loading VRPLib files
//! - Lexical and syntactic parsing
//! - Instance validation
//!
//! ## Example
//! ```no_run
//! let instance = vrp_parser::read_from_vrplib("example.vrp").unwrap();
//! ```
pub(crate) mod error;
pub(crate) mod instance;
pub(crate) mod model;
pub(crate) mod solomon;
pub(crate) mod util;
pub(crate) mod vrplib;

pub use error::LoadError;
pub use instance::VrpInstance;
pub(crate) use instance::VrpInstanceBuilder;
pub(crate) use model::EdgeWeightKind;
pub use model::ProblemType;
pub use solomon::parser::SolomonParseError;
pub use solomon::reader::read_from_solomon;
pub(crate) use util::Numeric;
pub use vrplib::error::VrplibError;
pub use vrplib::reader::read_from_vrplib;
pub use vrplib::reader::read_from_vrplib_f64;
