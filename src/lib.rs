//! # vrplib
//!
//! A library for loading VRP instance files and representing them as VRP instances.
//!
//! ## Supported formats
//! - **VRPLib**: use [`read_from_vrplib`] (integer weights) or [`read_from_vrplib_f64`]
//!   (floating-point weights)
//! - **Solomon**: use [`read_from_solomon_f64`] for CVRPTW instances
//!
//! ## Examples
//! ```no_run
//! // VRPLib (CVRP)
//! let instance = vrp_parser::read_from_vrplib("example.vrp").unwrap();
//!
//! // Solomon (CVRPTW)
//! let instance = vrp_parser::read_from_solomon_f64("example.txt").unwrap();
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
pub use solomon::reader::read_from_solomon_f64;
pub(crate) use util::Numeric;
pub use vrplib::reader::read_from_vrplib;
pub use vrplib::reader::read_from_vrplib_f64;
