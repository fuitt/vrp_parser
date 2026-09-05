pub(crate) fn expand_lower_row<T: Default + Copy>(lower_row: &[Vec<T>]) -> Vec<Vec<T>> {
    let size = lower_row.len() + 1;
    let mut ret = vec![vec![Default::default(); size]; size];

    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            if i < j {
                ret[i][j] = lower_row[j - 1][i];
            }
            if i > j {
                ret[i][j] = lower_row[i - 1][j];
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_lower_row() {
        let sut = vec![vec![1], vec![2, 3]];

        let value = expand_lower_row(&sut);

        let expected = vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 3, 0]];
        assert_eq!(value, expected);
    }
}
