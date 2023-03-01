#[cfg(test)]
mod tests {
    use crate::solution::name_shuffler;

    #[test]
    fn test_basic() {
        assert_eq!(name_shuffler("john McClane"), "McClane john");
        assert_eq!(name_shuffler("Mary jeggins"), "jeggins Mary");
        assert_eq!(name_shuffler("tom jerry"), "jerry tom");
    }
}
