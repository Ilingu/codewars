pub fn dbl_linear(n: u32) -> Vec<u32> {
    let mut u = vec![1];
    let mut idx = 0;
    while idx < n as usize {
        let (y, z) = (2 * u[idx] + 1, 3 * u[idx] + 1);
        if !u.contains(&y) {
            u.push(y);
        }
        if !u.contains(&z) {
            u.push(z);
        }
        idx += 1;
        u.sort();
    }
    u
}
