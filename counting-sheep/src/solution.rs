pub fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|b| b == &&true).count() as u8
}
