#[cfg(test)]
mod tests {
    use crate::solution::reverse_seq;

    #[test]
    fn sample_test() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }
}
