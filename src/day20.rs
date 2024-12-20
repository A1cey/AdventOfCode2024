use std::{collections::VecDeque, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

pub fn run() {
    println!("Day 20:");

    let input = fs::read_to_string("src/input/input20.txt").unwrap();

    let mut start = Coordinate::new(0, 0);
    let mut end = Coordinate::new(0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'E' => {
                        end = Coordinate::new(row, col);
                        Tile::End
                    }
                    'S' => {
                        start = Coordinate::new(row, col);
                        Tile::Start
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<_>>();

    let path = find_path(&map, start, map[0].len(), map.len());

    let shortcuts = find_best_shortcuts(&path);

    println!("There are {} shortcust that save 100 ps.", shortcuts.len());

    println!();
}

fn find_path(map: &[Vec<Tile>], start: Coordinate, width: usize, height: usize) -> Vec<Coordinate> {
    let mut path = vec![start];

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        let next_tiles = [
            pos.row
                .checked_sub(1)
                .map(|row| Coordinate::new(row, pos.col)),
            (pos.row + 1 < width - 1).then(|| Coordinate::new(pos.row + 1, pos.col)),
            pos.col
                .checked_sub(1)
                .map(|col| Coordinate::new(pos.row, col)),
            (pos.col + 1 < height - 1).then(|| Coordinate::new(pos.row, pos.col + 1)),
        ];

        for next_tile in next_tiles.into_iter().flatten() {
            if map[next_tile.row][next_tile.col] == Tile::Empty && !path.contains(&next_tile) {
                path.push(next_tile);
                queue.push_back(next_tile);
            }
        }
    }

    path
}

fn find_best_shortcuts(path: &[Coordinate]) -> Vec<(Coordinate, Coordinate)> {
    let mut shortcuts = Vec::new();

    for i in 0..path.len() - 101 {
        let begin = path[i];

        for j in i + 100..path.len() {
            let end = path[j];

            if calculate_distance(begin, end) < 3 {
                shortcuts.push((begin, end));
            }
        }
    }

    shortcuts
}

fn calculate_distance(start: Coordinate, end: Coordinate) -> usize {
    (start.row as isize - end.row as isize).abs() as usize
        + (start.col as isize - end.col as isize).abs() as usize
}
