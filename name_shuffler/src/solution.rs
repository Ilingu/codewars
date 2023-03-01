pub fn name_shuffler(s: &str) -> String {
    format!(
        "{} {}",
        s.split(" ").nth(1).unwrap(),
        s.split(" ").nth(0).unwrap()
    )
}
