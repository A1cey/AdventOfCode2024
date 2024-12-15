use std::{
    cmp::Ordering,
    fs::{self, File},
    io::Write,
    path::Path,
};

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

enum Quadrant {
    UpperLeft,
    LowerLeft,
    UpperRight,
    LowerRight,
    Middle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Occuppation {
    Robot,
    Empty,
}

impl Robot {
    const fn new(pos_x: i32, pos_y: i32, vel_x: i32, vel_y: i32) -> Robot {
        Robot {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        }
    }

    fn create_robot(data: &str) -> Robot {
        let (pos, vel) = data.split_once(" ").unwrap();

        let (pos_x, pos_y) = pos.strip_prefix("p=").unwrap().split_once(",").unwrap();
        let (vel_x, vel_y) = vel.strip_prefix("v=").unwrap().split_once(",").unwrap();

        Robot::new(
            pos_x.parse().unwrap(),
            pos_y.parse().unwrap(),
            vel_x.parse().unwrap(),
            vel_y.parse().unwrap(),
        )
    }
}

pub fn run() {
    println!("Day 14:");

    const ROOM_WIDTH: i32 = 101;
    const ROOM_HEIGHT: i32 = 103;
    const SIMULATION_COUNT: i32 = 100;
    const UPPER_BOUND: i32 = 10000;
    const CLUSTER_SIZE: i32 = 25;

    let robots = fs::read_to_string("src/input/input14.txt")
        .unwrap()
        .lines()
        .map(|line| Robot::create_robot(line))
        .collect::<Vec<_>>();

    find_christmas_tree(
        robots.clone(),
        UPPER_BOUND,
        CLUSTER_SIZE,
        ROOM_WIDTH,
        ROOM_HEIGHT,
    );

    let simulated_robots = robots
        .into_iter()
        .map(|robot| simulate_movement(SIMULATION_COUNT, robot, ROOM_WIDTH, ROOM_HEIGHT))
        .collect::<Vec<_>>();

    // simulated_robots
    //     .iter()
    //     .for_each(|robot| println!("{:?}", robot));

    let safety_factor = count_robots_in_quadrants(&simulated_robots, ROOM_WIDTH, ROOM_HEIGHT)
        .iter()
        .fold(1, |acc, num| acc * num);

    println!("Safety factor: {safety_factor}");
    println!();
}

fn simulate_movement(n: i32, mut robot: Robot, room_width: i32, room_height: i32) -> Robot {
    for _ in 0..n {
        let mut new_x = robot.pos_x + robot.vel_x;
        let mut new_y = robot.pos_y + robot.vel_y;

        if new_x >= room_width {
            new_x -= room_width;
        } else if new_x < 0 {
            new_x = room_width + new_x; // addition because x is negative
        }

        if new_y >= room_height {
            new_y -= room_height;
        } else if new_y < 0 {
            new_y = room_height + new_y;
        }

        robot.pos_x = new_x;
        robot.pos_y = new_y;
    }

    robot
}

fn count_robots_in_quadrants(robots: &[Robot], room_width: i32, room_height: i32) -> [i32; 4] {
    let mut quadrants = [0, 0, 0, 0];

    robots
        .iter()
        .for_each(|robot| match get_quadrant(robot, room_width, room_height) {
            Quadrant::UpperLeft => quadrants[0] += 1,
            Quadrant::UpperRight => quadrants[1] += 1,
            Quadrant::LowerLeft => quadrants[2] += 1,
            Quadrant::LowerRight => quadrants[3] += 1,
            Quadrant::Middle => (),
        });

    quadrants
}

fn get_quadrant(robot: &Robot, room_width: i32, room_height: i32) -> Quadrant {
    let x = robot.pos_x;
    let y = robot.pos_y;

    match x.cmp(&((room_width - 1) / 2)) {
        Ordering::Less => match y.cmp(&((room_height - 1) / 2)) {
            Ordering::Less => Quadrant::UpperLeft,
            Ordering::Greater => Quadrant::LowerLeft,
            Ordering::Equal => Quadrant::Middle,
        },
        Ordering::Greater => match y.cmp(&((room_height - 1) / 2)) {
            Ordering::Less => Quadrant::UpperRight,
            Ordering::Greater => Quadrant::LowerRight,
            Ordering::Equal => Quadrant::Middle,
        },
        Ordering::Equal => Quadrant::Middle,
    }
}

fn find_christmas_tree(
    mut robots: Vec<Robot>,
    upper_bound: i32,
    cluster_size: i32,
    room_width: i32,
    room_height: i32,
) {
    for sec in 1..=upper_bound {
        robots = robots
            .into_iter()
            .map(|robot| simulate_movement(1, robot, room_width, room_height))
            .collect();

        // manual labor
        //     let path_str = format!("src/day14_output/{}", sec);
        //     let path = Path::new(path_str.as_str());
        //     let mut file = File::create(&path).unwrap();

        //     print_robots(&robots, room_width, room_height, &mut file);

        // search for clusters
        let mut room = vec![Occuppation::Empty; room_width as usize * room_height as usize];

        robots.iter().for_each(|robot| {
            room[(robot.pos_y * room_width + robot.pos_x) as usize] = Occuppation::Robot
        });

        let mut max_cluster = 0;

        for y in 0..room_height - 5 {
            for x in 0..room_width - 5 {
                let mut cluster = 0;

                for dy in 0..5 {
                    for dx in 0..5 {
                        if room[((dy + y) * room_width + dx + x) as usize] == Occuppation::Robot {
                            cluster += 1;
                        }
                    }
                }

                max_cluster = max_cluster.max(cluster);
            }
        }

        if max_cluster >= cluster_size {
            let path_str = format!("src/day14_output/{}", sec);
            let path = Path::new(path_str.as_str());
            let mut file = File::create(&path).unwrap();

            print_robots(&robots, room_width, room_height, &mut file);
        }
    }
}

fn print_robots(robots: &[Robot], room_width: i32, room_height: i32, file: &mut File) {
    let mut room = vec![vec![" "; room_width as usize]; room_height as usize];

    robots
        .iter()
        .for_each(|robot| room[robot.pos_y as usize][robot.pos_x as usize] = "O");

    room.iter()
        .for_each(|row| writeln!(file, "{}", row.concat()).unwrap());
}
