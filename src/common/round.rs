pub(crate) fn nint(x: f64) -> u64 {
    (x + 0.5).floor() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nint_truncates() {
        let sut = 0.4;

        let value = nint(sut);

        let expected = 0;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_nint_rounds_up() {
        let sut = 0.6;

        let value = nint(sut);

        let expected = 1;
        assert_eq!(value, expected);
    }

    #[test]
    fn test_nint_boundary_value() {
        let sut = 0.5;

        let value = nint(sut);

        let expected = 1;
        assert_eq!(value, expected);
    }
}
