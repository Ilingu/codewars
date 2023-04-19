use std::{cmp::Ordering, collections::HashSet, hash::Hash};

pub fn decompose(n: i64) -> Option<Vec<i64>> {
    let goal = (n * n) as u64;

    let mut squared_solutions: Vec<(Vec<u64>, u64)> = vec![];
    let min = solve_cubic(1.0 / 3.0, 1.0 / 2.0, 1.0 / 6.0, -(goal as f64)).ceil() as u64;
    for (i, start_value) in ((n as u64 - 1)..(n as u64)).rev().enumerate() {
        squared_solutions.push((vec![(start_value.pow(2)) as u64], n.abs() as u64 - i as u64));
    }

    let mut all_solutions = vec![];
    while !squared_solutions.is_empty() {
        let (mut squared_solution, mut cn) = squared_solutions.remove(0);

        cn -= 1;
        if cn == 0 {
            continue;
        }

        let sum = squared_solution.iter().map(|&x| x as u64).sum::<u64>();
        let cnsquare = (cn * cn) as u64;
        if cnsquare > *squared_solution.last().unwrap() {
            continue;
        }

        let new_sum = cnsquare + sum;
        if new_sum < goal {
            squared_solution.push(cnsquare);

            let (max, min) = (
                ((goal - new_sum) as f64).sqrt().floor() as u64 + 1,
                solve_cubic(
                    1.0 / 3.0,
                    1.0 / 2.0,
                    1.0 / 6.0,
                    new_sum as f64 - goal as f64,
                )
                .ceil() as u64,
            );
            for ncn in (min..=max).rev() {
                squared_solutions.push((squared_solution.clone(), ncn));
            }
        } else if new_sum == goal {
            squared_solution.push(cnsquare);
            let solution = squared_solution
                .iter()
                .map(|&x| (x as f64).sqrt() as i64)
                .rev()
                .collect::<Vec<_>>();

            if has_unique_elements(solution.clone()) {
                all_solutions.push(solution.clone());
                if all_solutions.len() == 25 {
                    break;
                }
            }
        } else
        /* new_sum > goal */
        {
            if n - cn as i64 == 1 {
                let (max, min) = (
                    ((goal - cnsquare) as f64).sqrt().floor() as u64 + 1,
                    solve_cubic(
                        1.0 / 3.0,
                        1.0 / 2.0,
                        1.0 / 6.0,
                        cnsquare as f64 - goal as f64,
                    )
                    .ceil() as u64,
                );

                for ncn in (min..=max).rev() {
                    squared_solutions.push((squared_solution.clone(), ncn));
                }
            } else {
                squared_solutions.push((squared_solution, cn));
            }
        }
    }

    if all_solutions.is_empty() {
        return None;
    }

    all_solutions.sort_by(|a, b| {
        for (i, v) in a.iter().rev().enumerate() {
            match b.iter().rev().nth(i) {
                Some(w) => {
                    if v > w {
                        return Ordering::Less;
                    } else if v < w {
                        return Ordering::Greater;
                    }
                }
                None => return Ordering::Less,
            }
        }

        if a.len() == b.len() {
            return Ordering::Equal;
        }
        Ordering::Greater
    });
    Some(all_solutions[0].clone())
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let p = -b / (3.0 * a);
    let q = p.powi(3) + (b * c - 3.0 * a * d) / (6.0 * a.powi(2));
    let r = c / (3.0 * a);

    (q + (q.powi(2) + (r - p.powi(2)).powi(3)).sqrt()).cbrt()
        + (q - (q.powi(2) + (r - p.powi(2)).powi(3)).sqrt()).cbrt()
        + p
}
