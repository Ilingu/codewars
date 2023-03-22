use crate::MORSE_CODE;

pub fn decode_morse(encoded: &str) -> String {
    let mut out = String::new();
    for word in encoded.split("   ") {
        for ch in word.split_whitespace() {
            out.push_str(MORSE_CODE.get(ch).unwrap())
        }
        out.push(' ')
    }
    out.trim().to_string()
}
