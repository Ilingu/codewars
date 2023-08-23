fn reverse_words(str: &str) -> String {
    println!("{:?}", str.split(' ').collect::<Vec<_>>());
    println!("{:?}", str.split_whitespace().collect::<Vec<_>>());

    let mut rev_words = str.to_owned();
    let mut curr_word_start = 0;
    for (i, c) in str.chars().enumerate() {
        if i == str.len() - 1 {
            let word = &rev_words[curr_word_start..];
            rev_words = format!(
                "{}{}",
                &rev_words[..curr_word_start],
                word.chars().rev().collect::<String>(),
            );
        } else if c == ' ' {
            let word = &rev_words[curr_word_start..i];
            rev_words = format!(
                "{}{}{}",
                &rev_words[..curr_word_start],
                word.chars().rev().collect::<String>(),
                &rev_words[i..]
            );
            curr_word_start = i + 1;
        }
    }
    rev_words
}

#[cfg(test)]
mod tests {
    use crate::reverse_words;

    #[test]
    fn sample_test() {
        assert_eq!(
            reverse_words("The quick brown fox jumps over the lazy dog."),
            "ehT kciuq nworb xof spmuj revo eht yzal .god"
        );
        assert_eq!(reverse_words("apple"), "elppa");
        assert_eq!(reverse_words("a b c d"), "a b c d");
        assert_eq!(
            reverse_words("double  spaced  words"),
            "elbuod  decaps  sdrow"
        );
        assert_eq!(
            reverse_words("double spaced  words   triple"),
            "elbuod decaps  sdrow   elpirt"
        );
    }
}
