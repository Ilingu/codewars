#[cfg(test)]
mod tests {
    use crate::solution::how_many_lightsabers_do_you_own;

    #[test]
    fn returns_zero_for_an_empty_string() {
        assert_eq!(how_many_lightsabers_do_you_own(""), 0);
    }
    #[test]
    fn returns_0_for_other_people() {
        assert_eq!(how_many_lightsabers_do_you_own("Adam"), 0);
    }
    #[test]
    fn returns_18_for_zach() {
        assert_eq!(how_many_lightsabers_do_you_own("Zach"), 18);
    }
    #[test]
    fn returns_0_when_zach_is_lowercased() {
        assert_eq!(how_many_lightsabers_do_you_own("zach"), 0);
    }
}
