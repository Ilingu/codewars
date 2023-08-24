use std::{collections::HashSet, vec};

use itertools::Itertools;

fn permutational_primes(n_max: u32, k_perms: u32) -> (u32, u32, u32) {
    let primes = sieve_of_eratosthenes(n_max as usize);
    let mut valid_permutation_prime = vec![];

    let mut seen_prime_perms: HashSet<usize> = HashSet::new();
    for prime in &primes {
        if seen_prime_perms.contains(prime) {
            continue;
        }

        let permutations = prime
            .to_string()
            .chars()
            .permutations(prime.to_string().len())
            .filter_map(|perm| {
                let perm_num = perm
                    .iter()
                    .map(|c| c.to_string())
                    .join("")
                    .parse::<usize>()
                    .unwrap();
                if (perm_num as f64).log10() as usize + 1 == perm.len() {
                    Some(perm_num)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let primes_perm = permutations
            .into_iter()
            .filter(|perm| perm < &(n_max as usize) && primes.contains(perm))
            .unique()
            .collect::<Vec<_>>();
        seen_prime_perms.extend(&primes_perm);

        if primes_perm.len() - 1 == k_perms as usize {
            valid_permutation_prime.push(*prime);
        }
    }

    if valid_permutation_prime.is_empty() {
        (0, 0, 0)
    } else {
        (
            valid_permutation_prime.len() as u32,
            *valid_permutation_prime.first().unwrap() as u32,
            *valid_permutation_prime.last().unwrap() as u32,
        )
    }
}

fn sieve_of_eratosthenes(n_max: usize) -> Vec<usize> {
    let mut list = vec![false; n_max - 1];
    let mut primes = vec![2];

    let mut p = 2;
    loop {
        for k in 1..=(n_max / p) {
            let list_index = k * p - 2;
            list[list_index] = true;
        }

        let next_p = list.iter().position(|marked| !marked).map(|np| np + 2);
        match next_p {
            Some(np) => {
                if np > (n_max as f64).sqrt() as usize {
                    let mut remaining_primes = list
                        .iter()
                        .enumerate()
                        .filter(|(_, &marked)| !marked)
                        .map(|(i, _)| i + 2)
                        .collect::<Vec<_>>();
                    primes.append(&mut remaining_primes);
                    break;
                }

                primes.push(np);
                p = np;
            }
            None => break,
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, time::Instant};

    use super::*;

    fn dotest(n: u32, k: u32, expected: (u32, u32, u32)) {
        let actual = permutational_primes(n, k);
        assert!(
            actual == expected,
            "With n = {n}, k = {k}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(1000, 3, (3, 149, 379));
        dotest(1000, 2, (9, 113, 389));
        dotest(1000, 1, (34, 13, 797));
        dotest(18077, 1, (211, 13, 17807));
    }

    #[test]
    fn numlen_test() {
        const NUM: f64 = 3.14e20;

        fn mathematically(n: f64) -> usize {
            n.log10() as usize + 1
        }
        fn with_string(n: f64) -> usize {
            n.to_string().len()
        }

        fn do_bench<T: Fn(f64) -> usize>(cb: T) -> u128 {
            (0..10).fold(0, |acc, _| {
                let now = Instant::now();
                for _ in 0..1_000_000 {
                    assert_eq!(cb(NUM), 21);
                }
                acc + now.elapsed().as_millis()
            }) / 10
        }

        let mathematically_elapsed = do_bench(mathematically);
        let with_string_elapsed = do_bench(with_string);

        println!("mathematically_elapsed: {mathematically_elapsed}ms");
        println!("with_string_elapsed: {with_string_elapsed}ms");
    }

    #[test]
    fn soe_bench() {
        //   slowest
        fn soe_hashset(n: usize) {
            let mut list: HashSet<usize> = HashSet::from_iter(2..=n);
            let mut p = 2;

            loop {
                for i in 2..=(n / p) {
                    list.remove(&(p * i));
                }

                let next_p = list.iter().filter(|&&x| x > p).min();
                match next_p {
                    Some(&np) => {
                        if np > (n as f64).sqrt() as usize {
                            let _ = list.iter().copied().collect::<Vec<_>>();
                            break;
                        }
                        p = np
                    }
                    None => {
                        let _ = list.iter().copied().collect::<Vec<_>>();
                        break;
                    }
                }
            }
        }
        fn soe_vec1(n_max: usize) {
            let mut list = vec![false; n_max - 1];
            let mut primes = vec![2];

            let mut p = 2;
            loop {
                for k in 1..=(n_max / p) {
                    let list_index = k * p - 2;
                    list[list_index] = true;
                }

                let next_p = list.iter().position(|marked| !marked).map(|np| np + 2);
                match next_p {
                    Some(np) => {
                        if np > (n_max as f64).sqrt() as usize {
                            let mut remaining_primes = list
                                .iter()
                                .enumerate()
                                .filter(|(_, &marked)| !marked)
                                .map(|(i, _)| i + 2)
                                .collect::<Vec<_>>();
                            primes.append(&mut remaining_primes);
                            break;
                        }

                        primes.push(np);
                        p = np;
                    }
                    None => break,
                }
            }
        }
        fn soe_vec2(n_max: usize) {
            let mut list = vec![false; n_max - 1];
            let mut primes = vec![2];

            let mut p = 2;
            loop {
                for k in 1..=(n_max / p) {
                    let list_index = k * p - 2;
                    list[list_index] = true;
                }

                let next_p = list[(p - 1)..]
                    .iter()
                    .position(|marked| !marked)
                    .map(|np| np + p + 1);
                match next_p {
                    Some(np) => {
                        if np > (n_max as f64).sqrt() as usize {
                            let mut remaining_primes = list
                                .iter()
                                .enumerate()
                                .filter(|(_, &marked)| !marked)
                                .map(|(i, _)| i + 2)
                                .collect::<Vec<_>>();
                            primes.append(&mut remaining_primes);
                            break;
                        }

                        primes.push(np);
                        p = np;
                    }
                    None => break,
                }
            }
        }

        fn do_bench<T: Fn(usize)>(cb: T) -> u128 {
            (0..10).fold(0, |acc, _| {
                let now = Instant::now();
                cb(100_000_000);
                acc + now.elapsed().as_millis()
            }) / 10
        }

        let soe_vec1_elapsed = do_bench(soe_vec1);
        let soe_vec2_elapsed = do_bench(soe_vec2);

        println!("soe_vec1_elapsed: {soe_vec1_elapsed}ms");
        println!("soe_vec2_elapsed: {soe_vec2_elapsed}ms");
    }
}
