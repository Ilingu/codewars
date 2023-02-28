pub fn reverse_seq(n: u32) -> Vec<u32> {
    let mut out = vec![];
    for i in (1..=n).rev() {
        out.push(i);
    }
    out
}
