use std::{default, fs};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Robot,
    Box,
}

struct Robot {
    row: usize,
    col: usize,
}

impl Robot {
    const fn new(row: usize, col: usize) -> Robot {
        Robot { row, col }
    }
}

pub fn run() {
    println!("Day 15:");

    let data = fs::read_to_string("src/input/input15.txt").unwrap();

    let (warehouse_data, movement_data) = data.split_once("\r\n\r\n").unwrap();

    let (mut warehouse, mut robot) = get_warehouse(warehouse_data);

    let movements = get_movements(movement_data);

    // warehouse.iter().for_each(|row| {
    //     row.iter().for_each(|tile| match tile {
    //         Tile::Box => {
    //             print!("O");
    //         }
    //         Tile::Empty => {
    //             print!(".")
    //         }
    //         Tile::Wall => {
    //             print!("#")
    //         }
    //         Tile::Robot => print!("@"),
    //     });
    //     println!();
    // });

    movements.iter().for_each(|direction| {
        let mut row = robot.row;
        let mut col = robot.col;
        let mut movement_possible = false;

        match direction {
            Direction::Left => {
                while col > 0 {
                    col -= 1;

                    match warehouse[row][col] {
                        Tile::Box => {
                            continue;
                        }
                        Tile::Empty => {
                            movement_possible = true;
                            break;
                        }
                        Tile::Wall => {
                            break;
                        }
                        Tile::Robot => panic!("This should not be a possibility"),
                    }
                }

                if movement_possible {
                    while col < robot.col {
                        col += 1;
                        warehouse[row][col - 1] = warehouse[row][col];
                        warehouse[row][col] = Tile::Empty;
                    }

                    robot.col -= 1;
                }
            }
            Direction::Right => {
                while col < warehouse[0].len() - 1 {
                    col += 1;

                    match warehouse[row][col] {
                        Tile::Box => {
                            continue;
                        }
                        Tile::Empty => {
                            movement_possible = true;
                            break;
                        }
                        Tile::Wall => {
                            break;
                        }
                        Tile::Robot => panic!("This should not be a possibility"),
                    }
                }

                if movement_possible {
                    while col > robot.col {
                        col -= 1;
                        warehouse[row][col + 1] = warehouse[row][col];
                        warehouse[row][col] = Tile::Empty;
                    }

                    robot.col += 1;
                }
            }
            Direction::Up => {
                while row > 0 {
                    row -= 1;

                    match warehouse[row][col] {
                        Tile::Box => {
                            continue;
                        }
                        Tile::Empty => {
                            movement_possible = true;
                            break;
                        }
                        Tile::Wall => {
                            break;
                        }
                        Tile::Robot => panic!("This should not be a possibility"),
                    }
                }

                if movement_possible {
                    while row < robot.row {
                        row += 1;
                        warehouse[row - 1][col] = warehouse[row][col];
                        warehouse[row][col] = Tile::Empty;
                    }

                    robot.row -= 1;
                }
            }
            Direction::Down => {
                while row < warehouse.len() - 1 {
                    row += 1;

                    match warehouse[row][col] {
                        Tile::Box => {
                            continue;
                        }
                        Tile::Empty => {
                            movement_possible = true;
                            break;
                        }
                        Tile::Wall => {
                            break;
                        }
                        Tile::Robot => panic!("This should not be a possibility"),
                    }
                }

                if movement_possible {
                    while row > robot.row {
                        row -= 1;
                        warehouse[row + 1][col] = warehouse[row][col];
                        warehouse[row][col] = Tile::Empty;
                    }

                    robot.row += 1;
                }
            }
        }
        // println!();
        // println!(
        //     "{}",
        //     match direction {
        //         Direction::Left => '<',
        //         Direction::Right => '>',
        //         Direction::Up => '^',
        //         Direction::Down => 'v',
        //     }
        // );
        // warehouse.iter().for_each(|row| {
        //     row.iter().for_each(|tile| match tile {
        //         Tile::Box => {
        //             print!("O");
        //         }
        //         Tile::Empty => {
        //             print!(".")
        //         }
        //         Tile::Wall => {
        //             print!("#")
        //         }
        //         Tile::Robot => print!("@"),
        //     });
        //     println!();
        // });
    });

    // warehouse.iter().for_each(|row| {
    //     row.iter().for_each(|tile| match tile {
    //         Tile::Box => {
    //             print!("O");
    //         }
    //         Tile::Empty => {
    //             print!(".")
    //         }
    //         Tile::Wall => {
    //             print!("#")
    //         }
    //         Tile::Robot => print!("@"),
    //     });
    //     println!();
    // });

    let coordinates_sum = warehouse
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &tile)| tile == Tile::Box)
                .map(move |(col, _)| row * 100 + col)
        })
        .sum::<usize>();

    println!("Sum of all boxes GPS coordinates: {coordinates_sum}");
    println!();
}

fn get_warehouse(data: &str) -> (Vec<Vec<Tile>>, Robot) {
    let mut robot = Robot::new(0, 0);

    let warehouse = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => {
                        robot = Robot::new(row, col);
                        Tile::Robot
                    }
                    _ => panic!("This should not be a possibility"),
                })
                .collect()
        })
        .collect();

    (warehouse, robot)
}

fn get_movements(data: &str) -> Vec<Direction> {
    data.chars()
        .filter(|c| ['<', '>', '^', 'v'].contains(c))
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("This should not be a possibility"),
        })
        .collect()
}