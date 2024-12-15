use std::{collections::VecDeque, default, fs};

#[derive(Debug)]
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
    LeftBox,
    RightBox,
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

    movements
        .iter()
        .for_each(|direction| step(&mut warehouse, &mut robot, direction));

    let coordinates_sum_1 = warehouse
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &tile)| tile == Tile::Box)
                .map(move |(col, _)| row * 100 + col)
        })
        .sum::<usize>();

    // part 2
    // (warehouse, robot) = get_big_warehouse(warehouse_data);

    // movements.iter().for_each(|direction| {
    //     step_2(&mut warehouse, &mut robot, direction);
    // });

    // let middle = warehouse[0].len() / 2;

    // let coordinates_sum_2 = warehouse
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(row, line)| {
    //         line.iter()
    //             .enumerate()
    //             .filter(|(_, &tile)| tile == Tile::RightBox || tile == Tile::LeftBox)
    //             .map(move |(col, tile)| {
    //                 if col < middle {
    //                     match tile {
    //                         Tile::RightBox => 0,
    //                         Tile::LeftBox => row * 100 + col,
    //                         _ => panic!("Cannot happen due to filter"),
    //                     }
    //                 } else {
    //                     match tile {
    //                         Tile::RightBox => row * 100 + col,
    //                         Tile::LeftBox => 0,
    //                         _ => panic!("Cannot happen due to filter"),
    //                     }
    //                 }
    //             })
    //     })
    //     .sum::<usize>();

    println!("Sum of all boxes GPS coordinates: {coordinates_sum_1}");
    // println!("Sum of all boxes GPS coordinates: {coordinates_sum_2}");
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

fn step(warehouse: &mut Vec<Vec<Tile>>, robot: &mut Robot, direction: &Direction) {
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
                    _ => (),
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
                    _ => (),
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
                    _ => (),
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
                    _ => (),
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
}

fn get_big_warehouse(data: &str) -> (Vec<Vec<Tile>>, Robot) {
    let mut robot_row = 0;

    let warehouse = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => vec![Tile::Wall; 2],
                    '.' => vec![Tile::Empty; 2],
                    'O' => vec![Tile::LeftBox, Tile::RightBox],
                    '@' => {
                        robot_row = row;
                        vec![Tile::Robot, Tile::Empty]
                    }
                    _ => panic!("This should not be a possibility"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let robot_col = warehouse[robot_row]
        .iter()
        .position(|&tile| tile == Tile::Robot)
        .unwrap();

    (warehouse, Robot::new(robot_row, robot_col))
}

fn step_2(warehouse: &mut Vec<Vec<Tile>>, robot: &mut Robot, direction: &Direction) {
    let mut row = robot.row;
    let mut col = robot.col;
    let mut movement_possible = false;

    match direction {
        Direction::Left => {
            while col > 0 {
                col -= 1;

                match warehouse[row][col] {
                    Tile::RightBox | Tile::LeftBox => {
                        continue;
                    }
                    Tile::Empty => {
                        movement_possible = true;
                        break;
                    }
                    Tile::Wall => {
                        break;
                    }
                    _ => (),
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
                    Tile::RightBox | Tile::LeftBox => {
                        continue;
                    }
                    Tile::Empty => {
                        movement_possible = true;
                        break;
                    }
                    Tile::Wall => {
                        break;
                    }
                    _ => (),
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
            let mut boxes_to_check = VecDeque::<usize>::new();
            boxes_to_check.push_back(col);

            'outer: while row > 0 {
                row -= 1;

                let mut queue = boxes_to_check.drain(..).collect::<VecDeque<_>>();

                while !queue.is_empty() {
                    col = queue.pop_front().unwrap();

                    match warehouse[row][col] {
                        Tile::RightBox => {
                            boxes_to_check.push_back(col);
                            boxes_to_check.push_back(col - 1);
                        }
                        Tile::LeftBox => {
                            boxes_to_check.push_back(col);
                            boxes_to_check.push_back(col + 1);
                        }
                        Tile::Empty => {
                            movement_possible = true;
                        }
                        Tile::Wall => {
                            movement_possible = false;
                            break 'outer;
                        }
                        _ => (),
                    }
                }
            }

            if movement_possible {
                let end_row = row + 1;
                row = robot.row;
                col = robot.col;

                let mut cols_to_consider = VecDeque::<usize>::new();
                cols_to_consider.push_back(col);

                while row > end_row {
                    while !cols_to_consider.is_empty() {
                        col = cols_to_consider.pop_front().unwrap();

                        match warehouse[row - 1][col] {
                            Tile::RightBox => {
                                cols_to_consider.push_back(col);
                                cols_to_consider.push_back(col - 1);
                            }
                            Tile::LeftBox => {
                                cols_to_consider.push_back(col);
                                cols_to_consider.push_back(col + 1);
                            }
                            Tile::Empty => (),
                            _ => panic!("There should not be a wall or robot."),
                        }

                        warehouse[row - 1][col] = warehouse[row][col];
                        warehouse[row][col] = Tile::Empty;
                    }
                    row -= 1;
                }

                robot.row -= 1;
            }
        }
        Direction::Down => {
            let mut boxes_to_check = VecDeque::<usize>::new();
            boxes_to_check.push_back(col);

            'outer: while row < warehouse.len() - 1 {
                row += 1;

                let mut queue = boxes_to_check.drain(..).collect::<VecDeque<_>>();

                while !queue.is_empty() {
                    col = queue.pop_front().unwrap();

                    match warehouse[row][col] {
                        Tile::RightBox => {
                            boxes_to_check.push_back(col);
                            boxes_to_check.push_back(col - 1);
                        }
                        Tile::LeftBox => {
                            boxes_to_check.push_back(col);
                            boxes_to_check.push_back(col + 1);
                        }
                        Tile::Empty => {
                            movement_possible = true;
                        }
                        Tile::Wall => {
                            movement_possible = false;
                            break 'outer;
                        }
                        _ => (),
                    }
                }
            }

            if movement_possible {
                let end_row = row - 1;
                row = robot.row;
                col = robot.col;

                let mut cols_to_consider = VecDeque::<usize>::new();
                cols_to_consider.push_back(col);

                while row < end_row {
                    while !cols_to_consider.is_empty() {
                        col = cols_to_consider.pop_front().unwrap();

                        match warehouse[row + 1][col] {
                            Tile::RightBox => {
                                cols_to_consider.push_back(col);
                                cols_to_consider.push_back(col - 1);
                            }
                            Tile::LeftBox => {
                                cols_to_consider.push_back(col);
                                cols_to_consider.push_back(col + 1);
                            }
                            Tile::Empty => (),
                            _ => panic!("There should not be a wall or robot."),
                        }

                        warehouse[row + 1][col] = warehouse[row][col];
                        warehouse[row][col] = Tile::Empty;
                    }
                    row += 1;
                }

                robot.row += 1;
            }
        }
    }
}
