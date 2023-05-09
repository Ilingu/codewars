use itertools::Itertools;

pub fn next_bigger_number(n: u64) -> Option<u64> {
    let digits = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    let len = digits.len();

    for i in 0..len {
        let sub_digits = &digits[len - 1 - i..];
        let permutation = sub_digits
            .into_iter()
            .permutations(sub_digits.len())
            .map(|x| x.iter().map(|x| **x).collect::<Vec<_>>());

        let mut result = vec![];
        for mut p in permutation {
            let mut new_num = digits[..len - 1 - i].to_vec();
            new_num.append(&mut p);

            let num = digits_to_num(&new_num);
            if num > n {
                result.push(num);
            }
        }
        if !result.is_empty() {
            result.sort();
            return Some(result[0]);
        }
    }
    None
}

fn digits_to_num(digits: &Vec<u64>) -> u64 {
    digits
        .iter()
        .map(|x| x.to_string())
        .join("")
        .parse::<u64>()
        .unwrap()
}

/*
fn digits_to_num<T: Num + std::str::FromStr + std::fmt::Display>(digits: &Vec<T>) -> T
where
    <T as FromStr>::Err: Debug,
{
    digits
        .iter()
        .map(|x| x.to_string())
        .join("")
        .parse::<T>()
        .unwrap()
}
*/
