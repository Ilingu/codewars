pub fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|s| -> String {
            if s.len() >= 5 {
                s.chars().rev().collect()
            } else {
                s.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
