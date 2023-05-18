// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use crate::solution::proper_fractions;

    #[test]
    fn sample_tests() {
        assert_eq!(
            proper_fractions(1),
            0,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(2),
            1,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(5),
            4,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(15),
            8,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(25),
            20,
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}
