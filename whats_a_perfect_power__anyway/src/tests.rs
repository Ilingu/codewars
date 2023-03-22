// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use crate::solution::is_perfect_power;

    #[test]
    fn basic_examples() {
        assert_eq!(is_perfect_power(4), Some((2, 2)), "4 = 2^2");
        assert_eq!(is_perfect_power(9), Some((3, 2)), "9 = 3^2");
        assert_eq!(is_perfect_power(5), None, "5 is not a perfect power");
    }

    #[test]
    fn first_perfect_powers() {
        let pp = &[
            4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 81, 100, 121, 125, 128, 144, 169, 196, 216, 225,
            243, 256, 289, 324, 343, 361, 400, 441, 484,
        ];
        for p in pp {
            if is_perfect_power(*p) == None {
                assert!(false, "{} wasn't recognized as a perfect power", p)
            } else {
                assert!(true)
            }
        }
    }
}
