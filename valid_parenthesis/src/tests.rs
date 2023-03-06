#[cfg(test)]
mod tests {
    use crate::valid_parentheses;

    #[test]
    fn sample_tests() {
        do_test("()", true);
        do_test("())", false);
        do_test("", true);
        do_test("(}{)", true);
    }

    fn do_test(s: &str, exp: bool) {
        let actual = valid_parentheses(s);
        assert_eq!(
            actual, exp,
            "\nFor the input \"{}\", your result ({}) did not match the expected output ({})",
            s, actual, exp
        );
    }
}
