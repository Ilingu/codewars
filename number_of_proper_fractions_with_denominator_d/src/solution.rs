/* What I know:
- each x^n has a linear function associated, that given the number in input output the number of proper fractions
- for exemple, 2^n has the linear function "f(x)=0.5x"; for 2^4=16: f(2^4) = 8; and there is exacly 8 proper fractions for n=2^4
- prime number are the building blocks of these linear functions; the slope of the function is always: (n-1)/n with n a prime number; for example the slope of 5^x is 4/5; the slope of 11^x is 10/11 (this only work for prime numbers!)
- for non prime number; we have to find the prime factor of that number and multiply each UNIQUE prime number with there slopes; for example: if I want the slop of 10^x: factor(10)=2*5; thus the slop is: (1/2)*(4/5)=0.4 which match the reality; for 15^x: factor(15)=3*5 thus the slop is: (2/3)*(4/5)=8/15 thus f(15) = 8/15 *15 = 8 (which match the example in this problem instruction); what I mean by "UNIQUE" is take for example: 1_000_000: factor(1_000_000)=2^6*5^6, his slop is not (1/2)^6*(4/5)^6 but just (1/2)*(4/5), so yes: 10^x, 100^x, 1_000_000^x and even 1_000_000_000^x have the exact same slop of 0.4
*/

pub fn proper_fractions(n: u64) -> u64 {
    if n == 1 {
        return 0;
    }

    let slop = factors_uniq(n)
        .iter()
        .map(|&x| (x as f64 - 1.0) / x as f64)
        .product::<f64>();
    (n as f64 * slop).round() as u64
}

fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };
    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    x
}

fn factors_uniq(x: u64) -> Vec<u64> {
    if x <= 1 {
        return vec![];
    };
    let mut lst: Vec<u64> = Vec::new();
    let mut curn = x;
    loop {
        let m = firstfac(curn);
        lst.push(m);
        if curn == m {
            break;
        }
        while curn % m == 0 {
            curn /= m;
        }
        if curn == 1 {
            break;
        }
    }
    lst
}

// pub fn proper_fractions_old(d: u64) -> u64 {
//     (1..d)
//         .into_iter()
//         .fold(0, |acc, n| if gcd(n, d) == 1 { acc + 1 } else { acc })
// }

// fn gcd(n: u64, d: u64) -> u64 {
//     let rem = n.rem_euclid(d);
//     match rem {
//         0 => d,
//         _ => gcd(d, rem),
//     }
// }
