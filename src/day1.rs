use std::{collections::HashMap, fs};

pub fn run() {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    fs::read_to_string("src/input1.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .for_each(|v| {
            left.push(v.clone().first().unwrap().parse().unwrap());
            right.push(v.clone().last().unwrap().parse().unwrap());
        });

    left.sort();
    right.sort();

    let result = left.iter().enumerate().fold(0, |acc, (idx, val)| {
        acc + (val - right.get(idx).unwrap()).abs()
    });

    println!("Total distance: {}", result);

    let mut occurences_r: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|num| {
        if occurences_r.contains_key(num) {
            occurences_r.insert(*num, occurences_r.get(num).unwrap() + 1);
        } else {
            occurences_r.insert(*num, 1);
        }
    });

    let similiarity_score = left.iter().fold(0, |acc, num| {
        acc + num * occurences_r.get(num).unwrap_or(&0)
    });

    println!("similiarity score: {}", similiarity_score);

}