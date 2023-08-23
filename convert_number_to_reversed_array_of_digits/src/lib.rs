fn digitize(mut n: u64) -> Vec<u8> {
    let has_leading_zero = n % 10 == 0;
    let mut reversed = 0;
    while n > 0 {
        reversed *= 10;
        let rem = n % 10;
        n = (n - rem) / 10;

        reversed += rem;
    }

    let mut reversed_split = vec![];
    while reversed > 0 {
        reversed_split.insert(0, (reversed % 10) as u8);
        reversed /= 10;
    }
    if has_leading_zero {
        reversed_split.insert(0, 0)
    }
    reversed_split
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(348597), vec![7, 9, 5, 8, 4, 3]);
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
        assert_eq!(digitize(1885408990), vec![0, 9, 9, 8, 0, 4, 5, 8, 8, 1]);
    }
}
