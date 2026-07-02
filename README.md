# vrp_parser
A VRPLib parser for Rust.
Loads VRPLib-formatted files and constructs `VRPInstance` values.

## Example
```rust
// Load with integer (u64) numeric values
let instance = vrp_parser::read_from_vrplib("example.vrp")?;

// Load with floating-point (f64) numeric values
let instance = vrp_parser::read_from_vrplib_f64("example.vrp")?;
```

## Installation
```toml
vrp_parser = "0.1"
```

## Status
This is an early release (v0.1.1).

Currently supported VRPLib sections:
- NAME
- TYPE (CVRP only)
- COMMENT
- DIMENSION
- EDGE_WEIGHT_TYPE (EXPLICIT and EUC_2D only)
- EDGE_WEIGHT_FORMAT (LOWER_ROW only)
- EDGE_WEIGHT_SECTION
- NODE_COORD_TYPE (TWOD_COORDS only)
- NODE_COORD_SECTION
- CAPACITY
- DEMAND_SECTION
- DEPOT_SECTION

## License

Licensed under either of

 * Apache License, Version 2.0
 * MIT license

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
