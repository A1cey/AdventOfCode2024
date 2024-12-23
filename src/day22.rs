use std::{collections::HashMap, fs, ops::BitXor};

pub fn run() {
    println!("Day 22:");

    let initial_secret_nums = fs::read_to_string("src/input/input22.txt")
        .unwrap()
        .lines()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    let secret_nums = initial_secret_nums
        .iter()
        .map(|&start| generate_nth_secret_num(start, 2000))
        .collect::<Vec<_>>();
    
    let sequences = initial_secret_nums.iter()
        .map(|start| get_n_secrets(*start, 2000))
        .map(|secrets|get_secret_first_digits(&secrets))
        .map(|secrets| get_changes(&secrets)).collect::<Vec<_>>();
    
    let combined = combine_changes(&sequences);
    
    let highest_value = get_highest_value(&combined);    

    println!(
        "Sum of 2000th secret numbers: {}",
        secret_nums.iter().sum::<usize>()
    );
    println!(
        "Highest number of bananas: {highest_value}"
    );

    println!();
}

fn generate_nth_secret_num(start: usize, n: usize) -> usize {
    let mut secret_num = start;

    for _ in 0..n {
        secret_num = generate_next_secret_num(secret_num);
    }

    secret_num
}

fn generate_next_secret_num(prev: usize) -> usize {
    let secret_num_start = prune(mix(prev, prev * 64));
    let secret_num_middle = prune(mix(secret_num_start, secret_num_start / 32));
    let secret_num_final = prune(mix(secret_num_middle, secret_num_middle * 2048));

    secret_num_final
}

fn mix(secret_num: usize, val: usize) -> usize {
    val.bitxor(secret_num)
}

fn prune(secret_num: usize) -> usize {
    secret_num % 16777216
}

fn get_n_secrets(start: usize, n: usize) -> Vec<usize> {
    let mut secrets = vec![start];

    for _ in 0..n {
        secrets.push(generate_next_secret_num(*secrets.last().unwrap()));
    }

    secrets
}

fn get_secret_first_digits(secrets: &[usize]) -> Vec<usize> {
    secrets.iter().map(|secret| secret % 10).collect()
}

fn get_changes(secrets: &[usize]) -> HashMap<(isize, isize, isize, isize), usize> {
    let changes = secrets
        .windows(2)
        .map(|vals| vals[1] as isize - vals[0] as isize)
        .collect::<Vec<_>>();

    changes
        .windows(4)
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, block)| {
            let seq = (block[0], block[1], block[2], block[3]);

            if !acc.contains_key(&seq) {
                acc.insert(seq, secrets[idx + 4]);
            }

            acc
        })
}

fn combine_changes(changes: &[HashMap<(isize, isize, isize, isize),usize>]) -> HashMap<(isize, isize, isize, isize),Vec<usize>> {
    let mut map = HashMap::new();
    
    changes.iter().for_each(|mapping| 
        mapping.iter().for_each(|(key, val)|
            map.entry(*key).or_insert(vec![]).push(*val)
        ));
       
    map
}

fn get_highest_value(changes:  &HashMap<(isize, isize, isize, isize),Vec<usize>>) -> usize {
    changes.iter().map(|(_,vals)| vals.iter().sum()).max().unwrap()
}