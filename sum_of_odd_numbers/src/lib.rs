fn row_sum_odd_numbers(n: i64) -> i64 {
    let k = (1..n).sum::<i64>();
    (k..(k + n)).map(|x| 2 * x + 1).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(row_sum_odd_numbers(1), 1);
        assert_eq!(row_sum_odd_numbers(42), 74088);
    }
}
