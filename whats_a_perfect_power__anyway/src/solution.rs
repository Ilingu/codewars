pub fn is_perfect_power(n: u64) -> Option<(u64, u32)> {
    let (mut base, mut result): (usize, f64) = (2, (n as f64).powf(0.5) as f64);
    while result.fract() != 0.0 {
        if (result - 1.0).abs() <= 1e-3 {
            return None;
        }

        base += 1;
        result = (n as f64).powf(1.0 / base as f64) as f64;
        if (result.round() - result).abs() <= 1e-9 {
            result = result.round()
        }
    }

    Some((result as u64, base as u32))
}
