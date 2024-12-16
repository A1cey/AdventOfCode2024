use core::hash;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs, i32,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn new(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate::new(0, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Reindeer {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Reindeer {
    fn new(row: usize, col: usize, direction: Direction) -> Reindeer {
        Reindeer {
            row,
            col,
            direction,
        }
    }
}

impl Default for Reindeer {
    fn default() -> Self {
        Reindeer::new(0, 0, Direction::East)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

pub fn run() {
    println!("Day 16:");

    let input = fs::read_to_string("src/input/input16.txt").unwrap();

    let (reindeer, end, map) = parse_map(&input);

    // print_map(&map, &reindeer, &end);

    let lowest_cost = get_min_cost(&map, reindeer, &end);
    let best_paths = get_best_paths(&map, reindeer, &end);

    // print_paths(&map, &best_paths);

    let num_of_tiles = best_paths
        .iter()
        .flatten()
        .fold(HashSet::new(), |mut acc, coord| {
            acc.insert(coord);
            acc
        })
        .len();

    println!("Lowest cost: {lowest_cost}.");
    println!("{num_of_tiles} tiles are part of any best path.");
    println!();
}

fn parse_map(input: &str) -> (Reindeer, Coordinate, Vec<Vec<Tile>>) {
    let mut reindeer = Reindeer::default();
    let mut end = Coordinate::default();

    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Tile::Wall,
                    'S' => {
                        reindeer.row = row;
                        reindeer.col = col;
                        Tile::Empty
                    }
                    'E' => {
                        end = Coordinate::new(row, col);
                        Tile::Empty
                    }
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (reindeer, end, map)
}

fn print_map(map: &[Vec<Tile>], reindeer: &Reindeer, end: &Coordinate) {
    println!();
    map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, tile)| {
            match *tile {
                Tile::Wall => print!("#"),
                Tile::Empty if reindeer.col == col && reindeer.row == row => print!("R"),
                Tile::Empty if end.col == col && end.row == row => print!("E"),
                Tile::Empty => print!("."),
            };
        });
        println!();
    });
    println!();
}

fn print_paths(map: &[Vec<Tile>], paths: &[Vec<Coordinate>]) {
    paths.iter().for_each(|path| print_path(map, path));
}

fn print_path(map: &[Vec<Tile>], path: &[Coordinate]) {
    println!();
    map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, tile)| {
            match *tile {
                Tile::Wall => print!("#"),
                Tile::Empty if path.contains(&Coordinate::new(row, col)) => print!("O"),
                Tile::Empty => print!("."),
            };
        });
        println!();
    });
    println!();
}

fn get_min_cost(map: &[Vec<Tile>], reindeer: Reindeer, end: &Coordinate) -> i32 {
    // let costs = step(map, reindeer, end, 0, HashSet::new());
    // let costs = step_queue(map, *reindeer, end);
    // let min_cost = *costs.iter().min().unwrap();

    let min_cost = dijkstra(map, reindeer, end);
    min_cost
}

fn get_best_paths(map: &[Vec<Tile>], reindeer: Reindeer, end: &Coordinate) -> Vec<Vec<Coordinate>> {
    dijkstra_all_best_paths(map, reindeer, end)
}

fn step(
    map: &[Vec<Tile>],
    reindeer: &Reindeer,
    end: &Coordinate,
    curr_cost: i32,
    destinations_visited: HashSet<(Coordinate, Direction)>,
) -> Vec<i32> {
    // print_map(map, reindeer, end);
    // println!("curr_cost: {curr_cost}");

    if reindeer.row == end.row && reindeer.col == end.col {
        return vec![curr_cost];
    }

    let row = reindeer.row;
    let col = reindeer.col;

    let mut costs = vec![];

    let (front, left, right) = match reindeer.direction {
        Direction::East => (
            (&map[row][col - 1], 0, -1, Direction::East),
            (&map[row + 1][col], 1, 0, Direction::South),
            (&map[row - 1][col], -1, 0, Direction::North),
        ),
        Direction::West => (
            (&map[row][col + 1], 0, 1, Direction::West),
            (&map[row - 1][col], -1, 0, Direction::North),
            (&map[row + 1][col], 1, 0, Direction::South),
        ),
        Direction::North => (
            (&map[row - 1][col], -1, 0, Direction::North),
            (&map[row][col - 1], 0, -1, Direction::East),
            (&map[row][col + 1], 0, 1, Direction::West),
        ),
        Direction::South => (
            (&map[row + 1][col], 1, 0, Direction::South),
            (&map[row][col + 1], 0, 1, Direction::West),
            (&map[row][col - 1], 0, -1, Direction::East),
        ),
    };

    for next_step in [front, left, right] {
        if *next_step.0 == Tile::Empty {
            let next_coord = Coordinate::new(
                (reindeer.row as isize + next_step.1) as usize,
                (reindeer.col as isize + next_step.2) as usize,
            );

            if destinations_visited.contains(&(next_coord, reindeer.direction)) {
                continue;
            }

            let mut updatete_destinations_visited = destinations_visited.clone();
            updatete_destinations_visited.insert((next_coord, reindeer.direction));

            let new_reindeer = Reindeer::new(next_coord.row, next_coord.col, next_step.3);

            let new_cost = curr_cost
                + if next_step.3 != reindeer.direction {
                    1001
                } else {
                    1
                };

            costs.append(&mut step(
                map,
                &new_reindeer,
                end,
                new_cost,
                updatete_destinations_visited,
            ));
        }
    }

    costs
}

fn step_queue(map: &[Vec<Tile>], reindeer: Reindeer, end: &Coordinate) -> Vec<i32> {
    // print_map(map, reindeer, end);
    // println!("curr_cost: {curr_cost}");
    let mut costs = vec![];
    let destinations_visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_front((reindeer, 0, destinations_visited));

    while let Some((curr_reindeer, curr_cost, curr_destinations_visited)) = queue.pop_front() {
        let row = curr_reindeer.row;
        let col = curr_reindeer.col;

        if row == end.row && col == end.col {
            costs.push(curr_cost);
        }

        let (front, left, right) = match curr_reindeer.direction {
            Direction::East => (
                (&map[row][col - 1], 0, -1, Direction::East),
                (&map[row + 1][col], 1, 0, Direction::South),
                (&map[row - 1][col], -1, 0, Direction::North),
            ),
            Direction::West => (
                (&map[row][col + 1], 0, 1, Direction::West),
                (&map[row - 1][col], -1, 0, Direction::North),
                (&map[row + 1][col], 1, 0, Direction::South),
            ),
            Direction::North => (
                (&map[row - 1][col], -1, 0, Direction::North),
                (&map[row][col - 1], 0, -1, Direction::East),
                (&map[row][col + 1], 0, 1, Direction::West),
            ),
            Direction::South => (
                (&map[row + 1][col], 1, 0, Direction::South),
                (&map[row][col + 1], 0, 1, Direction::West),
                (&map[row][col - 1], 0, -1, Direction::East),
            ),
        };

        for next_step in [front, left, right] {
            if *next_step.0 == Tile::Empty {
                let next_coord = Coordinate::new(
                    (row as isize + next_step.1) as usize,
                    (col as isize + next_step.2) as usize,
                );

                if curr_destinations_visited.contains(&(next_coord, curr_reindeer.direction)) {
                    continue;
                }

                let mut updatete_destinations_visited = curr_destinations_visited.clone();
                updatete_destinations_visited.insert((next_coord, curr_reindeer.direction));

                let new_reindeer = Reindeer::new(next_coord.row, next_coord.col, next_step.3);

                let new_cost = curr_cost
                    + if next_step.3 != curr_reindeer.direction {
                        1001
                    } else {
                        1
                    };

                queue.push_back((new_reindeer, new_cost, updatete_destinations_visited));
            }
        }
    }

    costs
}

fn dijkstra(map: &[Vec<Tile>], reindeer: Reindeer, end: &Coordinate) -> i32 {
    let mut destinations_visited = HashSet::new();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, reindeer)));

    while let Some(Reverse((curr_cost, curr_reindeer))) = heap.pop() {
        let row = curr_reindeer.row;
        let col = curr_reindeer.col;

        if row == end.row && col == end.col {
            return curr_cost;
        }

        if !destinations_visited.insert((row, col, curr_reindeer.direction)) {
            continue;
        }

        let (front, left, right) = match curr_reindeer.direction {
            Direction::East => (
                (&map[row][col - 1], 0, -1, Direction::East),
                (&map[row + 1][col], 1, 0, Direction::South),
                (&map[row - 1][col], -1, 0, Direction::North),
            ),
            Direction::West => (
                (&map[row][col + 1], 0, 1, Direction::West),
                (&map[row - 1][col], -1, 0, Direction::North),
                (&map[row + 1][col], 1, 0, Direction::South),
            ),
            Direction::North => (
                (&map[row - 1][col], -1, 0, Direction::North),
                (&map[row][col - 1], 0, -1, Direction::East),
                (&map[row][col + 1], 0, 1, Direction::West),
            ),
            Direction::South => (
                (&map[row + 1][col], 1, 0, Direction::South),
                (&map[row][col + 1], 0, 1, Direction::West),
                (&map[row][col - 1], 0, -1, Direction::East),
            ),
        };

        for next_step in [front, left, right] {
            if *next_step.0 == Tile::Empty {
                let new_reindeer = Reindeer::new(
                    (row as isize + next_step.1) as usize,
                    (col as isize + next_step.2) as usize,
                    next_step.3,
                );

                let new_cost = curr_cost
                    + if next_step.3 != curr_reindeer.direction {
                        1001
                    } else {
                        1
                    };

                heap.push(Reverse((new_cost, new_reindeer)));
            }
        }
    }

    i32::MAX
}

fn dijkstra_all_best_paths(
    map: &[Vec<Tile>],
    reindeer: Reindeer,
    end: &Coordinate,
) -> Vec<Vec<Coordinate>> {
    //let mut destinations_visited = HashSet::new();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((
        0,
        reindeer,
        vec![Coordinate::new(reindeer.row, reindeer.col)],
    )));

    let mut costs = HashMap::new();
    costs.insert((reindeer.row, reindeer.col, reindeer.direction), 0);

    let mut best_paths = Vec::new();
    let mut min_cost = i32::MAX;

    while let Some(Reverse((curr_cost, curr_reindeer, path))) = heap.pop() {
        let row = curr_reindeer.row;
        let col = curr_reindeer.col;

        if curr_cost > min_cost {
            continue;
        }

        if row == end.row && col == end.col {
            if curr_cost < min_cost {
                min_cost = curr_cost;
                best_paths.clear();
            }

            best_paths.push(path);
            continue;
        }

        // if !destinations_visited.insert((row, col, curr_reindeer.direction)) {
        //     continue;
        // }

        let (front, left, right) = match curr_reindeer.direction {
            Direction::East => (
                (&map[row][col - 1], 0, -1, Direction::East),
                (&map[row + 1][col], 1, 0, Direction::South),
                (&map[row - 1][col], -1, 0, Direction::North),
            ),
            Direction::West => (
                (&map[row][col + 1], 0, 1, Direction::West),
                (&map[row - 1][col], -1, 0, Direction::North),
                (&map[row + 1][col], 1, 0, Direction::South),
            ),
            Direction::North => (
                (&map[row - 1][col], -1, 0, Direction::North),
                (&map[row][col - 1], 0, -1, Direction::East),
                (&map[row][col + 1], 0, 1, Direction::West),
            ),
            Direction::South => (
                (&map[row + 1][col], 1, 0, Direction::South),
                (&map[row][col + 1], 0, 1, Direction::West),
                (&map[row][col - 1], 0, -1, Direction::East),
            ),
        };

        for next_step in [front, left, right] {
            if *next_step.0 == Tile::Empty {
                let next_coord = Coordinate::new(
                    (row as isize + next_step.1) as usize,
                    (col as isize + next_step.2) as usize,
                );

                let new_reindeer = Reindeer::new(next_coord.row, next_coord.col, next_step.3);

                let new_cost = curr_cost
                    + if next_step.3 != curr_reindeer.direction {
                        1001
                    } else {
                        1
                    };

                let key = (next_coord.row, next_coord.col, next_step.3);
                if !costs.contains_key(&key) || new_cost <= *costs.get(&key).unwrap() {
                    costs.insert(key, new_cost);

                    let mut new_path = path.clone();
                    new_path.push(next_coord);

                    heap.push(Reverse((new_cost, new_reindeer, new_path)));
                }
            }
        }
    }

    best_paths
}
