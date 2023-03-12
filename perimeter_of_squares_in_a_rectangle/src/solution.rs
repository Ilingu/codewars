pub fn perimeter(n: u64) -> u64 {
    4 * fibo_sum(n)
}

fn fibo_sum(nth: u64) -> u64 {
    let mut fibo_sequence: Vec<u64> = vec![0, 1];
    let mut i = 2;
    while i != nth + 2 {
        fibo_sequence.push(fibo_sequence[(i - 1) as usize] + fibo_sequence[(i - 2) as usize]);
        i += 1
    }
    fibo_sequence.iter().sum()
}
