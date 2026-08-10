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
pub(crate) mod common;
pub(crate) mod instance;
pub(crate) mod vrplib;

pub(crate) use common::EdgeWeightKind;
pub(crate) use common::Numeric;
pub use common::ProblemType;
pub use instance::VrpInstance;
pub(crate) use instance::VrpInstanceBuilder;
pub use vrplib::reader::LoadError;
pub use vrplib::reader::read_from_vrplib;
pub use vrplib::reader::read_from_vrplib_f64;
