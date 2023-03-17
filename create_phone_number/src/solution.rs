pub fn create_phone_number(num: &[u8]) -> String {
    let vec_to_str = |vec: &[u8]| vec.iter().map(|&id| id.to_string()).collect::<String>();
    format!(
        "({}) {}-{}",
        vec_to_str(&num[..3]),
        vec_to_str(&num[3..6]),
        vec_to_str(&num[6..])
    )
}
