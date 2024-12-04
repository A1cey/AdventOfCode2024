use std::fs;

enum Valid {
    True(i32, i32),
    False,
}

pub fn run() {
    println!("Day 3:");

    let input = fs::read_to_string("src/input3.txt").unwrap();

    let mut sum = 0;
    let mut idx = 0;
    let mut enabled = true;

    while idx < input.len() {
        if enabled {
            if (&input[idx..]).starts_with("don't()") {
                enabled = false;
                idx += "don't()".len();
            } else if (&input[idx..]).starts_with("mul(") {
                match is_valid_pattern(&input[idx..]) {
                    Valid::False => idx += 1,
                    Valid::True(a, b) => {
                        sum += a * b;
                        idx += format!("mul({},{})", a, b).len();
                    }
                }
            } else {
                idx += 1;
            }
        } else {
            if (&input[idx..]).starts_with("do()") {
                enabled = true;
                idx += "do()".len();
            } else {
                idx += 1;
            }
        }
    }

    println!("Result: {}", sum);
    println!();
}

fn is_valid_pattern(pattern: &str) -> Valid {
    if !pattern.starts_with("mul(") {
        return Valid::False;
    }

    let closing_paren = match pattern.find(")") {
        Some(idx) => idx,
        None => return Valid::False,
    };

    let subpattern = &pattern[4..closing_paren];

    let parts: Vec<&str> = subpattern.split(",").collect();

    if parts.len() != 2 {
        return Valid::False;
    }

    let first_num = parts[0];
    let second_num = parts[1];

    if first_num.len() > 3 || first_num.is_empty() || second_num.len() > 3 || second_num.is_empty()
    {
        return Valid::False;
    }

    if !first_num.chars().all(char::is_numeric) || !second_num.chars().all(char::is_numeric) {
        return Valid::False;
    }

    match (first_num.parse(), second_num.parse()) {
        (Ok(a), Ok(b)) => Valid::True(a, b),
        _ => Valid::False,
    }
}
