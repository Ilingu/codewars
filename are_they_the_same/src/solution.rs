pub fn comp(a: Vec<i64>, mut b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for el in a {
        if !b.contains(&el.pow(2)) {
            return false;
        }
        b.remove(b.iter().position(|&x| x == el.pow(2)).unwrap());
    }
    true
}
