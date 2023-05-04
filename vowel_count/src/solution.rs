pub fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(&c))
        .count()
}
