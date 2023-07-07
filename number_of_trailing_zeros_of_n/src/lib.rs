pub fn zeros(n: u64) -> u64 {
    let k_max = (n as f64).log(5.0).floor() as u32;
    (1..=k_max)
        .map(|k| ((n as f64) / 5_f64.powi(k as i32)).floor() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}
