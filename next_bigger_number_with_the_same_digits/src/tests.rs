#[cfg(test)]
mod tests {
    use crate::solution::next_bigger_number;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected result (right).";

    #[test]
    fn sample_tests() {
        assert_eq!(None, next_bigger_number(9), "{ERR_MSG}");
        assert_eq!(Some(21), next_bigger_number(12), "{ERR_MSG}");
        assert_eq!(Some(531), next_bigger_number(513), "{ERR_MSG}");
        assert_eq!(Some(2071), next_bigger_number(2017), "{ERR_MSG}");
        assert_eq!(Some(441), next_bigger_number(414), "{ERR_MSG}");
        assert_eq!(Some(414), next_bigger_number(144), "{ERR_MSG}");
    }
}
