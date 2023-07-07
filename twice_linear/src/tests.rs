#[cfg(test)]
mod tests {
    use crate::solution::dbl_linear;

    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
    }
}
