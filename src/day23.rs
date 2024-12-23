use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    ops::Div,
    path::Path,
};

pub fn run() {
    println!("Day 23:");

    let matrix = get_diagonal_matrix(&fs::read_to_string("src/input/input23.txt").unwrap());
    let triple_connections = find_triples(&matrix);
    let count = triple_connections
        .iter()
        .filter(|triple| triple_contains_computer_starting_with_t(triple))
        .count();

    println!("{count} triples contain at least one computer starting with t.");
    println!();
}

fn get_diagonal_matrix(input: &str) -> Vec<Vec<bool>> {
    const CAPACITY: usize = 26 * 26;
    let mut matrix = vec![vec![]; CAPACITY];
    for i in 0..CAPACITY {
        matrix[i] = vec![false; i];
    }

    input.lines().for_each(|line| {
        let computers = line
            .split("-")
            .map(|s| get_integer_val(s))
            .collect::<Vec<_>>();


        let (a, b) = (
            computers[0].min(computers[1]),
            computers[0].max(computers[1]),
        );

        matrix[b][a] = true;
    });

    matrix
}

fn find_triples(matrix: &[Vec<bool>]) -> HashSet<(usize, usize, usize)> {
    let mut triples = HashSet::new();

    matrix.iter().enumerate().for_each(|(row, connections)| {
        connections
            .iter()
            .enumerate()
            .filter(|(_, connection)| **connection)
            .for_each(|(col, _)| {
                // check row
                matrix[row]
                    .iter()
                    .enumerate()
                    .filter(|(sec_col, sec_connection)| {
                        **sec_connection && *matrix[col].get(*sec_col).or(Some(&false)).unwrap()
                    })
                    .for_each(|(sec_col, _)| {
                        triples.insert(order(row, col, sec_col));
                    });

                // check col
                matrix[col]
                    .iter()
                    .enumerate()
                    .filter(|(sec_row, sec_connection)| {
                        **sec_connection && *matrix[row].get(*sec_row).or(Some(&false)).unwrap()
                    })
                    .for_each(|(sec_row, _)| {
                        triples.insert(order(row, col, sec_row));
                    });
            })
    });

    triples
}

fn order(a: usize, b: usize, c: usize) -> (usize, usize, usize) {
    let mut sorted = [a, b, c];
    sorted.sort();
    (sorted[0], sorted[1], sorted[2])
}

fn get_integer_val(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let first = (chars[0] as u8 - b'a') as usize;
    let second = (chars[1] as u8 - b'a') as usize;
    first * 26 + second
}

fn reverse_integer_val(num: usize) -> String {
    let first = (num / 26) as u8 + b'a';
    let second = (num % 26) as u8 + b'a';
    String::from_utf8(vec![first, second]).unwrap()
}

fn triple_contains_computer_starting_with_t(triple: &(usize, usize, usize)) -> bool {
    reverse_integer_val(triple.0).starts_with("t")
        || reverse_integer_val(triple.1).starts_with("t")
        || reverse_integer_val(triple.2).starts_with("t")
}
