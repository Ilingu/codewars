#[cfg(test)]
mod tests {
    use crate::solution::decode_morse;

    #[test]
    fn test_hey_jude() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}
