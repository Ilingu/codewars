pub fn validate_pin(pin: &str) -> bool {
    pin.chars().all(|c| c.is_ascii_digit()) && (pin.len() == 4 || pin.len() == 6)
}
