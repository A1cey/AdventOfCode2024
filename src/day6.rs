// ....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...

// ....#.....
// ....XXXXX#
// ....X...X.
// ..#.X...X.
// ..XXXXX#X.
// ..X.X.X.X.
// .#XXXXXXX.
// .XXXXXXX#.
// #XXXXXXX..
// ......#X..

//41
#![allow(dead_code)]

use std::{collections::HashSet, fs};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Path,
    Visited,
    Obstacle,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
    fn set(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
}

#[derive(Clone)]
struct Room {
    room: Vec<Vec<State>>,
    width: usize,
    height: usize,
}

impl Room {
    fn create_from_grid(grid: Vec<Vec<State>>) -> Room {
        Room {
            width: grid.first().unwrap().len(),
            height: grid.len(),
            room: grid,
        }
    }

    fn set_visited(&mut self, position: &Position) {
        self.room[position.row][position.col] = State::Visited;
    }
}

pub fn run() {
    println!("Day 6:");

    let mut loops = 0;
    let mut position: Position = Position::new(0, 0);

    let room_grid = fs::read_to_string("src/input/input6.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => State::Path,
                    '#' => State::Obstacle,
                    _ => {
                        position.set(row, col);
                        State::Visited
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let room = Room::create_from_grid(room_grid);

    room.room.iter().enumerate().for_each(|(row, v)| {
        v.iter()
            .enumerate()
            .skip_while(|(col, _)| match room.room[row][*col] {
                State::Obstacle | State::Visited => true,
                State::Path => false,
            })
            .for_each(|(col, _)| {
                let mut virtual_room = room.clone();
                virtual_room.room[row][col] = State::Obstacle;

                let mut direction = Direction::Up;
                let mut visited_count = 1;
                let mut virtual_position = position;

                let mut positions = HashSet::<(Position, Direction)>::new();
                positions.insert((position, direction));

                loop {
                    match direction {
                        Direction::Left => {
                            if virtual_position.col != 0 {
                                match virtual_room.room[virtual_position.row]
                                    [virtual_position.col - 1]
                                {
                                    State::Path => {
                                        virtual_position
                                            .set(virtual_position.row, virtual_position.col - 1);
                                        visited_count += 1;
                                        virtual_room.set_visited(&virtual_position);
                                    }
                                    State::Obstacle => {
                                        direction = Direction::Up;
                                    }
                                    State::Visited => {
                                        virtual_position
                                            .set(virtual_position.row, virtual_position.col - 1);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        Direction::Right => {
                            if virtual_position.col != virtual_room.width - 1 {
                                match virtual_room.room[virtual_position.row]
                                    [virtual_position.col + 1]
                                {
                                    State::Path => {
                                        virtual_position
                                            .set(virtual_position.row, virtual_position.col + 1);
                                        visited_count += 1;
                                        virtual_room.set_visited(&virtual_position);
                                    }
                                    State::Obstacle => {
                                        direction = Direction::Down;
                                    }
                                    State::Visited => {
                                        virtual_position
                                            .set(virtual_position.row, virtual_position.col + 1);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        Direction::Down => {
                            if virtual_position.row != virtual_room.height - 1 {
                                match virtual_room.room[virtual_position.row + 1]
                                    [virtual_position.col]
                                {
                                    State::Path => {
                                        virtual_position
                                            .set(virtual_position.row + 1, virtual_position.col);
                                        visited_count += 1;
                                        virtual_room.set_visited(&virtual_position);
                                    }
                                    State::Obstacle => {
                                        direction = Direction::Left;
                                    }
                                    State::Visited => {
                                        virtual_position
                                            .set(virtual_position.row + 1, virtual_position.col);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        Direction::Up => {
                            if virtual_position.row != 0 {
                                match virtual_room.room[virtual_position.row - 1]
                                    [virtual_position.col]
                                {
                                    State::Path => {
                                        virtual_position
                                            .set(virtual_position.row - 1, virtual_position.col);
                                        visited_count += 1;
                                        virtual_room.set_visited(&virtual_position);
                                    }
                                    State::Obstacle => {
                                        direction = Direction::Right;
                                    }
                                    State::Visited => {
                                        virtual_position
                                            .set(virtual_position.row - 1, virtual_position.col);
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    if !positions.insert((virtual_position, direction)) {
                        loops += 1;
                        break;
                    }
                }

                if row == 0 && col == 0 {
                    println!("{visited_count} squares were visited.")
                };
            });
    });

    println!("There are {loops} loops possible.");
    println!();
}
