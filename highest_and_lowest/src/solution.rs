pub fn high_and_low(numbers: &str) -> String {
    let n = numbers
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    format!("{} {}", n.clone().max().unwrap(), n.min().unwrap())
}
