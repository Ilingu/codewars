use std::{cmp::Ordering, collections::HashMap};

pub fn simplify(mut polynomial: &str) -> String {
    polynomial = polynomial.trim().trim_matches('+');
    let mut variables_families = HashMap::new();

    let mut last_family_idx = 0;
    let mut add_family = |family: &str| {
        let is_minus = family.contains("-");
        let raw_amount = family
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap_or(1);
        let amount = if !is_minus { raw_amount } else { -raw_amount };

        let mut sorted_vec = family
            .replace("+", "")
            .replace("-", "")
            .chars()
            .filter(|ch| ch.is_ascii_alphabetic())
            .collect::<Vec<_>>();
        sorted_vec.sort();

        let sorted_family: String = sorted_vec.iter().map(|x| x.to_string()).collect();

        variables_families
            .entry(sorted_family)
            .and_modify(|counter| *counter += amount)
            .or_insert(amount);
    };

    for (i, ch) in polynomial.chars().enumerate() {
        if (ch == '+' || ch == '-') && i != 0 {
            let family = &polynomial[last_family_idx..i];
            add_family(family);
            last_family_idx = i;
        }
    }
    add_family(&polynomial[last_family_idx..]);

    let mut sorted_families = vec![];
    for (family, amount) in variables_families.clone() {
        if amount == 0 {
            continue;
        }
        sorted_families.push((family, amount));
    }
    sorted_families.sort_by(|a, b| {
        if a.0.len().cmp(&b.0.len()) == Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            a.0.len().cmp(&b.0.len())
        }
    });

    let mut simplified_expr = String::new();
    for (family, amount) in sorted_families {
        let mut str_amount = amount.to_string();
        if amount > 0 {
            str_amount = "+".to_string() + str_amount.as_str()
        }
        if amount == 1 || amount == -1 {
            str_amount = str_amount.replace("1", "");
        }

        simplified_expr.push_str(format!("{str_amount}{family}").as_str())
    }

    simplified_expr.trim_matches('+').to_string()
}
