use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Sub,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Point {
        Point { row, col }
    }
}

pub fn run() {
    println!("Day 8:");

    let input = fs::read_to_string("src/input/input8.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let row_len: isize = lines[0].len().try_into().unwrap();
    let col_len: isize = lines.len().try_into().unwrap();

    let points_map = lines
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.ne(&'.'))
                .for_each(|(col, c)| {
                    acc.entry(c)
                        .or_insert_with(|| Vec::new())
                        .push(Point::new(row as isize, col as isize))
                });
            acc
        });

    println!("{}",points_map.iter().map(|(_,v)| v.len()).sum::<usize>());
        
    let unique_locations = points_map
        .iter()
        .filter(|(_, points)| points.len() > 1)
        .flat_map(|(_, points)| {
            points.iter().enumerate().flat_map(|(idx, a)| {
                points
                    .iter()
                    .enumerate()
                    .filter(move |(j, _)| *j > idx)
                    .flat_map(|(_, b)| find_antinodes(a, b))
            })
        })
        .filter(|point| {
            point.row >= 0 && point.row < row_len && point.col >= 0 && point.col < col_len
        })
        .collect::<HashSet<_>>()
        .len();

    println!("There are {unique_locations} unique locations.");
    println!();
}

fn find_antinodes<'a>(a: &Point, b: &Point) -> Vec<Point> {
    let distance_row = b.row - a.row;
    let distance_col = b.col - a.col;

    vec![
        Point::new(a.row - distance_row, a.col - distance_col),
        Point::new(b.row - distance_row, b.col - distance_col),
    ]
}
