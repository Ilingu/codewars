pub mod tests;

fn main() {}

pub fn is_valid_ip(ip: &str) -> bool {
    let digits = ip.split(".");
    if digits.count() != 4 {
        return false;
    }
    for digit in ip.split(".") {
        if digit != "0" && digit.starts_with("0") {
            return false;
        }
        match digit.parse::<usize>() {
            Ok(d) => {
                if d > 255 {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    true
}
