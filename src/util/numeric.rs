use std::str::FromStr;

pub(crate) trait Numeric: FromStr + Default + Copy {}

impl Numeric for u64 {}

impl Numeric for f64 {}
