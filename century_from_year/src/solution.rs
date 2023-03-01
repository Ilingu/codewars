pub fn century(year: u32) -> u32 {
    let two_digits = year as f32 / 100.0;
    return if two_digits.fract() == 0.0 {
        two_digits as u32
    } else {
        two_digits as u32 + 1
    };
}
