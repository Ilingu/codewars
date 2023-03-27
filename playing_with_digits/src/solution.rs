pub fn dig_pow(n: i64, p: i32) -> i64 {
    let sum = n
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, ch)| (ch.to_digit(10).unwrap() as f64).powf(p as f64 + i as f64))
        .sum::<f64>();

    if (sum / n as f64).fract() == 0.0 {
        sum as i64 / n
    } else {
        -1
    }
}
