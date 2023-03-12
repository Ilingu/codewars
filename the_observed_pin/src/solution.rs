const ADJACENT: [[&str; 5]; 10] = [
    ["0", "8", "", "", ""],
    ["1", "2", "4", "", ""],
    ["1", "2", "3", "5", ""],
    ["2", "3", "6", "", ""],
    ["1", "4", "5", "7", ""],
    ["2", "4", "5", "6", "8"],
    ["3", "5", "6", "9", ""],
    ["4", "7", "8", "", ""],
    ["0", "5", "7", "8", "9"],
    ["6", "8", "9", "", ""],
];

pub fn get_pins(observed: &str) -> Vec<String> {
    let all_digits: Vec<_> = observed
        .chars()
        .map(|x| {
            ADJACENT[x.to_string().parse::<usize>().unwrap()]
                .iter()
                .filter(|y| y != &&"")
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    recursive_brute_force(all_digits)
}

fn recursive_brute_force(all_digits: Vec<Vec<String>>) -> Vec<String> {
    if all_digits.len() == 1 {
        return all_digits[0].to_vec();
    }

    let mut result: Vec<String> = vec![];
    for digit in &all_digits[0] {
        let child_combinaison = recursive_brute_force(all_digits[1..].to_vec());
        for child in child_combinaison {
            result.push(format!("{digit}{child}"))
        }
    }

    result
}
