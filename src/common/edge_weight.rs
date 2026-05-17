use crate::common::round::nint;

pub(crate) fn edge_weight_by_euc2d(p: (u64, u64), q: (u64, u64)) -> u64 {
    let dx = (p.0 as f64) - (q.0 as f64);
    let dy = (p.1 as f64) - (q.1 as f64);
    nint((dx * dx + dy * dy).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nint_truncates() {
        let p = (1, 2);
        let q = (3, 5);

        let value = edge_weight_by_euc2d(p, q);

        let expected = 4;
        assert_eq!(value, expected);
    }
}
