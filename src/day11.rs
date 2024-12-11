use std::{collections::HashMap, fs};

pub fn run() {
    println!("Day 11:");

    let stones = fs::read_to_string("src/input/input11.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let num_of_stones1 = part1(&stones);
    let num_of_stones2 = part2(&stones);

    println!("There are {num_of_stones1} stones after 25 iterations.");
    println!("There are {num_of_stones2} stones after 75 iterations.");
    println!()
}

fn part1(initial_stones: &[String]) -> usize {
    let mut stones = initial_stones
        .iter()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = stones
            .iter()
            .flat_map(|&num| {
                if num == 0 {
                    vec![1]
                } else if num.to_string().len() % 2 == 0 {
                    let num_str = num.to_string();
                    let mid = num_str.len() / 2;
                    let (a, b) = (&num_str[..mid], &num_str[mid..]);
                    vec![a.parse().unwrap(), b.parse().unwrap()]
                } else {
                    vec![num * 2024]
                }
            })
            .collect();
    }

    stones.len()
}

fn part2(initial_stones: &[String]) -> usize {
    let mut stones = initial_stones.iter().fold(HashMap::new(), |mut acc, num| {
        let num = num.parse::<usize>().unwrap();
        *acc.entry(num).or_insert(0) += 1;

        acc
    });

    for _ in 0..75 {
        stones = stones
            .iter()
            .fold(HashMap::new(), |mut acc, (&num, &count)| {
                if num == 0 {
                    *acc.entry(1).or_insert(0) += count;
                } else if num.to_string().len() % 2 == 0 {
                    let num_str = num.to_string();
                    let mid = num_str.len() / 2;
                    let (a, b) = (&num_str[..mid], &num_str[mid..]);
                    *acc.entry(a.parse().unwrap()).or_insert(0) += count;
                    *acc.entry(b.parse().unwrap()).or_insert(0) += count;
                } else {
                    *acc.entry(num * 2024).or_insert(0) += count;
                }

                acc
            });
    }

    stones.iter().map(|(_, &count)| count).sum()
}
