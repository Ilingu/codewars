#[derive(PartialEq)]
enum Braces {
    Brackets,
    CurlyBraces,
    Parenthesis,
}

pub fn valid_braces(s: &str) -> bool {
    let mut queue: Vec<Braces> = vec![];
    for ch in s.chars() {
        match ch {
            '(' => queue.push(Braces::Parenthesis),
            '{' => queue.push(Braces::CurlyBraces),
            '[' => queue.push(Braces::Brackets),

            ')' => {
                if queue.len() == 0 || queue[queue.len() - 1] != Braces::Parenthesis {
                    return false;
                }
                queue.pop();
            }
            '}' => {
                if queue.len() == 0 || queue[queue.len() - 1] != Braces::CurlyBraces {
                    return false;
                }
                queue.pop();
            }
            ']' => {
                if queue.len() == 0 || queue[queue.len() - 1] != Braces::Brackets {
                    return false;
                }
                queue.pop();
            }
            _ => {
                return false;
            }
        }
    }
    queue.is_empty()
}
