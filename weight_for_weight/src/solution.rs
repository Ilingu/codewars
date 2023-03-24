// use std::cmp::Ordering;

pub fn order_weight(s: &str) -> String {
    let mut parsed_weight = s
        .trim()
        .split_whitespace()
        .filter(|&x| x != "")
        .map(|num| {
            (
                num.parse::<usize>().unwrap(),
                num.chars()
                    .map(|digit| digit.to_string().parse::<usize>().unwrap())
                    .sum::<usize>(),
            )
        })
        .collect::<Vec<_>>();

    parsed_weight.sort_by(|a, b| {
        if a.1 != b.1 {
            return a.1.cmp(&b.1);
        } else {
            return a.0.to_string().cmp(&b.0.to_string());
        }
    });

    parsed_weight
        .iter()
        .map(|(ord_num, _)| ord_num.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
