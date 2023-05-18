use crate::solution::proper_fractions;

pub mod solution;
pub mod tests;

fn main() {
    println!("{}", proper_fractions(709438050));
}

/* What I know:
- each x^n has a linear function associated, that given the number in input output the number of proper fractions
- for exemple, 2^n has the linear function "f(x)=0.5x"; for 2^4=16: f(2^4) = 8; and there is exacly 8 proper fractions for n=2^4
- prime number are the building blocks of these linear functions; the slope of the function is always: (n-1)/n with n a prime number; for example the slope of 5^x is 4/5; the slope of 11^x is 10/11 (this only work for prime numbers!)
- for non prime number; we have to find the prime factor of that number and multiply each UNIQUE prime number with there slopes; for example: if I want the slop of 10^x: factor(10)=2*5; thus the slop is: (1/2)*(4/5)=0.4 which match the reality; for 15^x: factor(15)=3*5 thus the slop is: (2/3)*(4/5)=8/15 thus f(15) = 8/15 *15 = 8 (which match the example in this problem instruction); what I mean by "UNIQUE" is take for example: 1_000_000: factor(1_000_000)=2^6*5^6, his slop is not (1/2)^6*(4/5)^6 but just (1/2)*(4/5), so yes: 10^x, 100^x, 1_000_000^x and even 1_000_000_000^x have the exact same slop of 0.4
*/
