# vrp_parser
A VRP instance file parser for Rust.
Loads VRPLib and Solomon-formatted files and constructs `VrpInstance` values.

## Supported formats

| Format  | Function                  | Problem type |
|---------|---------------------------|--------------|
| VRPLib  | `read_from_vrplib`        | CVRP (`u64` weights) |
| VRPLib  | `read_from_vrplib_f64`    | CVRP (`f64` weights) |
| Solomon | `read_from_solomon_f64`   | CVRPTW (`f64` weights, exact Euclidean distances) |

## Examples

```rust
// VRPLib — integer weights
let instance = vrp_parser::read_from_vrplib("example.vrp")?;

// VRPLib — floating-point weights
let instance = vrp_parser::read_from_vrplib_f64("example.vrp")?;

// Solomon — CVRPTW with exact Euclidean distances
let instance = vrp_parser::read_from_solomon_f64("example.txt")?;
```

## Installation

```toml
vrp_parser = "0.2"
```

## Supported VRPLib sections

- NAME
- TYPE (CVRP only)
- COMMENT
- DIMENSION
- EDGE_WEIGHT_TYPE (EXPLICIT and EUC_2D)
- EDGE_WEIGHT_FORMAT (LOWER_ROW and FULL_MATRIX)
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
