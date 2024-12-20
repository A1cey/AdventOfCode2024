use std::collections::{HashMap, HashSet};

type Position = (i32, i32);

const DEPRECATED_CHEAT_DURATION: i32 = 2;
const LATEST_CHEAT_DURATION: i32 = 20;

#[derive(Debug)]
struct RaceSetup {
    obstacles: HashSet<Position>,
    start: Position,
    end: Position,
}

pub fn run() {
    println!("Day 20:");

    println!(
        "Part 1: {}",
        part1(&parse_input(include_str!("input/input20.txt")))
    );
    println!(
        "Part 2: {}",
        part2(&parse_input(include_str!("input/input20.txt")))
    );

    println!();
}

fn parse_input(input: &str) -> RaceSetup {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, tile)| match tile {
            '#' => {
                obstacles.insert((x as i32, y as i32));
            }
            'S' => {
                start = (x as i32, y as i32);
            }
            'E' => {
                end = (x as i32, y as i32);
            }
            _ => (),
        })
    });

    RaceSetup {
        obstacles,
        start,
        end,
    }
}

fn distance(position_a: &Position, position_b: &Position) -> i32 {
    (position_b.0 - position_a.0).abs() + (position_b.1 - position_a.1).abs()
}

fn count_deprecated_cheats(race_setup: &RaceSetup, saved: i32) -> usize {
    let mut path = HashMap::from([(race_setup.start, 0)]);
    let mut position = race_setup.start;

    while position != race_setup.end {
        for next_position in [
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ] {
            if !path.contains_key(&next_position) && !race_setup.obstacles.contains(&next_position)
            {
                path.insert(next_position, *path.get(&position).unwrap() + 1);
                position = next_position;
            }
        }
    }

    let mut count = 0;

    for (position, cost) in path.iter() {
        for next_position in [
            (position.0 - 2, position.1),
            (position.0 + 2, position.1),
            (position.0, position.1 - 2),
            (position.0, position.1 + 2),
        ] {
            if let Some(next_cost) = path.get(&next_position) {
                if next_cost - cost >= saved + DEPRECATED_CHEAT_DURATION {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part1(race_setup: &RaceSetup) -> usize {
    count_deprecated_cheats(race_setup, 100)
}

fn count_latest_cheats(race_setup: &RaceSetup, saved: i32) -> usize {
    let mut path = vec![race_setup.start];
    let mut visited = HashSet::from([race_setup.start]);
    let mut position = race_setup.start;

    while position != race_setup.end {
        for next_position in [
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ] {
            if !visited.contains(&next_position) && !race_setup.obstacles.contains(&next_position) {
                visited.insert(next_position);
                path.push(next_position);
                position = next_position;
            }
        }
    }

    let mut count = 0;

    for cheat_start_index in 0..path.len() {
        for cheat_end_index in cheat_start_index + 1..path.len() {
            let distance = distance(&path[cheat_start_index], &path[cheat_end_index]);

            if distance <= LATEST_CHEAT_DURATION
                && (cheat_end_index - cheat_start_index) as i32 >= saved + distance
            {
                count += 1;
            }
        }
    }

    count
}

fn part2(race_setup: &RaceSetup) -> usize {
    count_latest_cheats(race_setup, 100)
}
