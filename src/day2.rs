use std::fs;

pub fn run() {
    println!("Day 2:");

    let mut ans1 = 0;
    let mut ans2 = 0;

    for line in fs::read_to_string("src/input/input2.txt")
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
    {
        let mut values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_save(&mut values) {
            ans1 += 1;
            ans2 += 1;
            continue;
        }

        for i in 0..values.len() {
            let mut vec = values.clone();
            vec.remove(i);

            if is_save(&mut vec) {
                ans2 += 1;
                break;
            }
        }
    }
    println!("{} reports are safe.", ans1);
    println!("{} reports are now safe.", ans2);
    println!();
}

fn is_save(values: &mut Vec<i32>) -> bool {
    if values.first() > values.iter().nth(1) {
        values.reverse();
    }

    values
        .iter()
        .is_sorted_by(|a, b| a < b && *b - *a >= 1 && *b - *a <= 3)
}
