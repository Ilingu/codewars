pub fn camel_case(str: &str) -> String {
    str.split_whitespace()
        .map(|s| s[..1].to_uppercase() + &s[1..])
        .collect()
}
