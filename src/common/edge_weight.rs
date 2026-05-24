use crate::common::round::nint;

pub(crate) fn edge_weight_by_euc2d(p: (f64, f64), q: (f64, f64)) -> u64 {
    let dx = p.0 - q.0;
    let dy = p.1 - q.1;
    nint((dx * dx + dy * dy).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nint_truncates() {
        let p = (1.0, 2.0);
        let q = (3.0, 5.0);

        let value = edge_weight_by_euc2d(p, q);

        let expected = 4;
        assert_eq!(value, expected);
    }
}
