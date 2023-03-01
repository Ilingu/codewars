#[cfg(test)]
mod tests {
    use crate::solution::solution;

    #[test]
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
    }
}
