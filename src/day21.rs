use std::fs;

#[derive(Debug)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Press,
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}
impl Coordinate {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

pub fn run() {
    println!("Day 21:");

    // find shortest path for all presses on numpad
    // convert into left right up down presses
    // convert into robot sequence 3x

    let input = fs::read_to_string("src/input/input21.txt").unwrap();
    let numeric_parts = get_numeric_parts(&input);

    let paths = get_numpad_paths(&input);
    let controller_1_paths = get_controller_path(&paths);
    let controller_2_paths = get_controller_path(&controller_1_paths);

    //print_path(&paths);
    //print_path(&controller_1_paths);
    //print_path(&controller_2_paths);

    let complexity = controller_2_paths
        .iter()
        .enumerate()
        .map(|(idx, path)| {
            
            path.len() * numeric_parts[idx]
        })
        .sum::<usize>();

    println!("Sum of the complexities: {complexity}");
    println!();
}

fn get_numeric_parts(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.strip_suffix("A").unwrap().parse().unwrap())
        .collect()
}

fn print_path(path: &[Vec<Action>]) {
   // println!("{:?}", path);
    println!(
        "{}",
        path[4].iter().fold(String::new(), |acc, action| acc
            + match action {
                Action::Up => "^",
                Action::Down => "v",
                Action::Left => "<",
                Action::Right => ">",
                Action::Press => "A",
            })
    );
}

fn get_path(start: &Coordinate, goal: &Coordinate) -> Vec<Action> {
    let dy = goal.row as isize - start.row as isize;
    let dx = goal.col as isize - start.col as isize;

    let mut path = vec![];

    // goal left  -> go vertical first
    // goal right -> go horizontal first
    if dx.is_negative() {
        if dy.is_negative() {
            for _ in 0..dy.abs() {
                path.push(Action::Up);
            }
        } else {
            for _ in 0..dy {
                path.push(Action::Down);
            }
        }

        for _ in 0..dx.abs() {
            path.push(Action::Left);
        }
    } else {
        for _ in 0..dx {
            path.push(Action::Right);
        }

        if dy.is_negative() {
            for _ in 0..dy.abs() {
                path.push(Action::Up);
            }
        } else {
            for _ in 0..dy {
                path.push(Action::Down);
            }
        }
    }

    path
}

fn get_numpad_paths(input: &str) -> Vec<Vec<Action>> {
    let mut start = get_coordinate_numpad(&'A');

    input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| {
                    let goal = get_coordinate_numpad(&c);
                    let mut path = get_path(&start, &goal);
                    start = goal;
                    path.push(Action::Press);
                    path
                })
                .collect()
        })
        .collect()
}

fn get_controller_path(actions: &[Vec<Action>]) -> Vec<Vec<Action>> {
    let mut start = get_coordinate_controller(&Action::Press);

    actions
        .iter()
        .map(|action_path| {
            action_path
                .iter()
                .flat_map(|action| {
                    let goal = get_coordinate_controller(&action);
                    let mut path = get_path(&start, &goal);
                    start = goal;
                    path.push(Action::Press);
                    path
                })
                .collect()
        })
        .collect()
}

fn get_coordinate_numpad(goal: &char) -> Coordinate {
    match *goal {
        'A' => Coordinate::new(3, 2),
        '0' => Coordinate::new(3, 1),
        '1' => Coordinate::new(2, 0),
        '2' => Coordinate::new(2, 1),
        '3' => Coordinate::new(2, 2),
        '4' => Coordinate::new(1, 0),
        '5' => Coordinate::new(1, 1),
        '6' => Coordinate::new(1, 2),
        '7' => Coordinate::new(0, 0),
        '8' => Coordinate::new(0, 1),
        '9' => Coordinate::new(0, 2),
        _ => unreachable!(),
    }
}

fn get_coordinate_controller(goal: &Action) -> Coordinate {
    match *goal {
        Action::Press => Coordinate::new(0, 2),
        Action::Up => Coordinate::new(0, 1),
        Action::Left => Coordinate::new(1, 0),
        Action::Down => Coordinate::new(1, 1),
        Action::Right => Coordinate::new(1, 2),
    }
}
