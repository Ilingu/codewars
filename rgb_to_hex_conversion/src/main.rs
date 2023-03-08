pub mod tests;

fn main() {}

fn hex_substitute(hex: u8) -> String {
    match hex {
        10 => "A".to_string(),
        11 => "B".to_string(),
        12 => "C".to_string(),
        13 => "D".to_string(),
        14 => "E".to_string(),
        15 => "F".to_string(),
        _ => hex.to_string(),
    }
}

fn base10_to_hex(base10: u8) -> String {
    let mut converted_number: Vec<(usize, u8)> = vec![]; // (col, value)

    let mut col_id = 0;
    let mut last_quo = base10 as f32;
    loop {
        let quo = last_quo.div_euclid(16.0);
        let rem = last_quo.rem_euclid(16.0);

        if quo == 0.0 {
            converted_number.push((col_id, rem as u8));
            break;
        }
        converted_number.push((col_id, rem as u8));

        last_quo = quo;
        col_id += 1;
    }
    converted_number.reverse();

    (if converted_number.len() == 1 { "0" } else { "" }).to_string()
        + converted_number
            .iter()
            .map(|&x| hex_substitute(x.1))
            .collect::<String>()
            .as_str()
}

pub fn rgb2(r: i32, g: i32, b: i32) -> String {
    vec![r, g, b]
        .iter()
        .map(|&x| {
            if x < 0 {
                "00".to_string()
            } else if x > 255 {
                "FF".to_string()
            } else {
                base10_to_hex(x as u8)
            }
        })
        .collect()
}

pub fn rgb(r: i32, g: i32, b: i32) -> String {
    vec![r, g, b]
        .iter()
        .map(|&x| {
            format!("{:#04x}", x.min(255).max(0))
                .trim_start_matches("0x")
                .to_uppercase()
                .to_string()
        })
        .collect()
}
