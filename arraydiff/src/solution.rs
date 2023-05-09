pub fn array_diff<T: PartialEq + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.iter().filter(|&e| !b.contains(&e)).map(|&x| x).collect()
}
