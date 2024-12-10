use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    const fn new(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
}

pub fn run() {
    println!("Day 10:");

    let mut trail_heads = Vec::new();

    let map = fs::read_to_string("src/input/input10.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let num = c.to_digit(10).unwrap();
                    if num == 0 {
                        trail_heads.push(Coordinate::new(row, col))
                    }
                    num
                })
                .collect()
        })
        .collect();

    let trails = trail_heads
        .into_iter()
        .map(|trail_head| step(0, trail_head, &map))
        .collect::<Vec<_>>();

    let score = trails
        .iter()
        .flat_map(|trails| {
            trails.iter().fold(HashSet::new(), |mut acc, coord| {
                acc.insert(coord);
                acc
            })
        })
        .count();

    let raiting = trails.iter().flatten().count();

    println!("The score of all trailheads is: {score}.");
    println!("The raiting of all trailheads is: {raiting}.");
    println!();
}

fn step(curr_incline: u32, coord: Coordinate, map: &Vec<Vec<u32>>) -> Vec<Coordinate> {
    if curr_incline == 9 {
        return vec![coord];
    }

    let directions = [
        (-1, 0), // Up
        (1, 0),  // Down
        (0, -1), // Left
        (0, 1),  // Right
    ];

    let mut trails = Vec::new();
    let row_len = map.len();
    let col_len = map[0].len();

    for (dx, dy) in directions {
        let new_row = coord.row as i32 + dx;
        let new_col = coord.col as i32 + dy;

        if new_row >= 0
            && new_row < row_len as i32
            && new_col >= 0
            && new_col < col_len as i32
            && map[new_row as usize][new_col as usize] == curr_incline + 1
        {
            trails.append(&mut step(
                curr_incline + 1,
                Coordinate::new(new_row as usize, new_col as usize),
                map,
            ));
        }
    }

    trails
}
