pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq,
{
    let mut last_seen = vec![];
    for c in sequence.into_iter() {
        if last_seen.len() == 0 || c != last_seen[0] {
            last_seen.insert(0, c);
        }
    }
    last_seen.reverse();
    last_seen
}
