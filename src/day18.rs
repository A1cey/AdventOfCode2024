use colored::Colorize;

use std::{
    collections::{HashSet, VecDeque},
    fs, i32,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn new(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
}

pub fn run() {
    println!("Day 18:");

    const WIDTH: usize = 71;
    const HEIGHT: usize = 71;

    let coords = fs::read_to_string("src/input/input18.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (col, row) = line.split_once(",").unwrap();
            Coordinate::new(row.parse().unwrap(), col.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let steps = shortest_path_bfs(coords.iter().take(1024).collect(), WIDTH, HEIGHT);

    // let first_blocking_byte = first_blocking_byte_brute_force(&coords, WIDTH, HEIGHT);
    let first_blocking_byte = first_blocking_byte_binary_search(&coords, WIDTH, HEIGHT);

    println!("{steps} steps needed.");
    println!(
        "The Byte with the coordinates {},{} will block the path first.",
        first_blocking_byte.col, first_blocking_byte.row
    );
    println!();
}

fn shortest_path_bfs(coords: HashSet<&Coordinate>, width: usize, height: usize) -> i32 {
    let goal = Coordinate::new(height - 1, width - 1);
    let start = Coordinate::new(0, 0);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back((0, start));

    while let Some((steps, coord)) = queue.pop_front() {
        if coord == goal {
            return steps;
        }

        let neighbors = [
            coord
                .col
                .checked_sub(1)
                .map(|col| Coordinate::new(coord.row, col)),
            (coord.col + 1 < width).then(|| Coordinate::new(coord.row, coord.col + 1)),
            coord
                .row
                .checked_sub(1)
                .map(|row| Coordinate::new(row, coord.col)),
            (coord.row + 1 < height).then(|| Coordinate::new(coord.row + 1, coord.col)),
        ];

        neighbors.into_iter().flatten().for_each(|neighbor| {
            if !coords.contains(&neighbor) && visited.insert(neighbor) {
                queue.push_back((steps + 1, neighbor));
            }
        });
    }
    -1
}

fn first_blocking_byte_brute_force(
    coords: &[Coordinate],
    width: usize,
    height: usize,
) -> &Coordinate {
    for i in 1025..coords.len() {
        if -1 == shortest_path_bfs(coords.iter().take(i).collect(), width, height) {
            return &coords[i - 1];
        }
    }

    unreachable!()
}

fn first_blocking_byte_binary_search(
    coords: &[Coordinate],
    width: usize,
    height: usize,
) -> &Coordinate {
    let mut low = 1024;
    let mut high = coords.len();
    let mut result = 1025;

    while low <= high {
        let mid = (low + high) / 2;

        if -1 == shortest_path_bfs(coords.iter().take(mid).collect(), width, height) {
            high = mid - 1;
        } else {
            low = mid + 1;
            result = mid;
        }
    }

    &coords[result]
}
