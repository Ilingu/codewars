use solution::longest_palindrome;

pub mod solution;
pub mod tests;

fn main() {
    let s = format!(
        "{}{}{}",
        &"a".repeat(10_000),
        "bbaaacc",
        &"a".repeat(10_000)
    );
    println!("{}", longest_palindrome(s.as_str()))
}
