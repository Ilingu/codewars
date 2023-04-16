/*
    use itertools::Itertools;
    use itertools::MinMaxResult::{MinMax, OneElement};


    fn find_all(n: u8, k: u8) -> Option<(usize, u64, u64)> {
        if k > n || 9 * k < n { return None }
        let ls =
        (1..10)
        .combinations_with_replacement(k as usize)
        .filter(|e| e.iter().sum::<u8>() == n)
        .map(|e| e.iter().map(|x| x.to_string()).collect::<String>().parse::<u64>().unwrap() )
        .collect::<Vec<_>>();
        match ls.iter().minmax() {
            MinMax(x, y) => Some((ls.len(), *x, *y)),
            OneElement(x) => Some((ls.len(), *x, *x)),
            _ => None
        }
    }
*/

pub fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let all = generate_all(sum_dig as usize, digs as usize)
        .iter()
        .map(|d| {
            d.iter()
                .map(|d| d.to_string())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    // println!("{:?}", all);

    if all.is_empty() {
        None
    } else {
        Some((
            all.len(),
            *all.iter().min().unwrap(),
            *all.iter().max().unwrap(),
        ))
    }
}

fn generate_all(sum_dig: usize, digs: usize) -> Vec<Vec<usize>> {
    if digs < 2 {
        panic!()
    }

    if sum_dig as isize - 9 * (digs as isize - 1) >= 10 {
        return vec![];
    }

    let mut res: Vec<Vec<usize>> = vec![];
    for d in 1..=9 {
        let x = sum_dig - d;
        if d > x {
            break;
        }

        if digs == 2 {
            if x >= 10 {
                continue;
            }
            res.push(vec![d, x]);
        } else {
            let all_nums = generate_all(x, digs - 1);
            for digits in all_nums {
                let mut digits_check = digits.clone();
                digits_check.insert(0, d as usize);
                if is_sorted(&digits_check) {
                    res.push(digits_check)
                }
            }
        }
    }
    res
}

fn is_sorted(map: &Vec<usize>) -> bool {
    let mut sorted = map.clone();
    sorted.sort();
    &sorted == map
}

// pub fn find_all_bis(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
//     let (mut len, mut min, mut max) = (0, None, None);

//     for n in 10_usize.pow((digs - 1) as u32)..10_usize.pow(digs as u32) {
//         let digits = n
//             .to_string()
//             .chars()
//             .map(|c| c.to_digit(10).unwrap() as usize)
//             .collect::<Vec<_>>();

//         // if digits[..(digits.len() - 1)].iter().sum::<usize>() == sum_dig as usize {
//         //     println!("{:?}", digits[..(digits.len() - 1)].to_vec());
//         //     break;
//         // }

//         if digits.iter().sum::<usize>() == sum_dig as usize && is_sorted(&digits) {
//             len += 1;
//             min = Some(min.unwrap_or(n).min(n));
//             max = Some(max.unwrap_or(n).max(n));
//         }
//     }

//     if len == 0 {
//         None
//     } else {
//         Some((len, min.unwrap() as u64, max.unwrap() as u64))
//     }
// }

// pub fn generate_all_increasing_nums(digits_num: u8, idx: Vec<usize>) -> Vec<usize> {
//     if digits_num == 1 {
//         let mut res = vec![];
//         for j in 0..=9 {
//             let mut idx_check = idx.clone();
//             idx_check.push(j);

//             if is_sorted(&idx_check) && idx_check[0] != 0 {
//                 res.push(
//                     idx_check
//                         .iter()
//                         .map(|d| d.to_string())
//                         .collect::<String>()
//                         .parse::<usize>()
//                         .unwrap(),
//                 )
//             }
//         }
//         return res;
//     }

//     let mut res = vec![];
//     for i in 0..=9 {
//         let mut idx_child = idx.clone();
//         idx_child.push(i);

//         res.append(&mut generate_all_increasing_nums(digits_num - 1, idx_child));
//     }

//     res
// }
