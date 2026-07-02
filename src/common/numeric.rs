use std::str::FromStr;

pub(crate) trait Numeric: FromStr + Default + Copy {
    fn as_usize(&self) -> usize;
    fn from_u64(val: u64) -> Self;
}

impl Numeric for u64 {
    fn as_usize(&self) -> usize {
        *self as usize
    }

    fn from_u64(val: u64) -> Self {
        val
    }
}

impl Numeric for f64 {
    fn as_usize(&self) -> usize {
        *self as usize
    }

    fn from_u64(val: u64) -> Self {
        val as f64
    }
}
