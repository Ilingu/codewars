pub fn order(sentence: &str) -> String {
    let mut word_by_id = sentence
        .split_whitespace()
        .map(|w| {
            (
                w.chars()
                    .find(|c| c.is_digit(10))
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
                w,
            )
        })
        .collect::<Vec<_>>();
    word_by_id.sort_by(|(a, _), (b, _)| a.cmp(b));

    word_by_id
        .iter()
        .map(|&(_, w)| w)
        .collect::<Vec<_>>()
        .join(" ")
}
