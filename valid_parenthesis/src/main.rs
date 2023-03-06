pub mod tests;

fn main() {}

pub fn valid_parentheses(s: &str) -> bool {
    let mut state = 0;
    for ch in s.chars() {
        match ch {
            '(' => state += 1,
            ')' => {
                if state == 0 {
                    return false;
                }
                state -= 1
            }
            _ => {}
        }
    }
    state == 0
}
