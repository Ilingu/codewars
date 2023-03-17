pub fn simplify(polynomial: &str) -> String {
    let family_scan: Vec<_> = polynomial
        .split("+")
        .into_iter()
        .map(|x| x.split("-").into_iter().collect::<Vec<_>>())
        .collect();

    println!("{:?}", family_scan);
    "".to_string()
}
