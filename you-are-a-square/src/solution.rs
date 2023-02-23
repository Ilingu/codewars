pub fn is_square(n: i64) -> bool {
    return (n as f64).sqrt().fract() == 0.0;
}
