pub fn longest_palindrome(s: &str) -> String {
    if is_palindromic(s) {
        return s.to_string();
    }

    for i in (2..s.len()).rev() {
        let mut j = 0;
        while i + j != s.len() + 1 {
            let substring = &s[j..i + j];
            if is_palindromic(substring) {
                return substring.to_string();
            }
            j += 1
        }
    }

    if s == "" {
        String::new()
    } else {
        s.chars().nth(0).unwrap().to_string()
    }
}

fn is_palindromic(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

/* Non-linear solution
 for i in (2..s.len()).rev() {
       let mut j = 0;
       while i + j != s.len() + 1 {
           let substring = &s[j..i + j];
           if is_palindromic(substring) {
               return substring.to_string();
           }
           j += 1
       }
    }
*/
