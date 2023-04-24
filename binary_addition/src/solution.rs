pub fn add_binary(a: u64, b: u64) -> String {
    let mut binary_sum = String::new();
    let mut q = a + b;
    while q != 0 {
        binary_sum.push_str((q % 2).to_string().as_str());
        q = q / 2;
    }
    binary_sum.chars().rev().collect()
}
