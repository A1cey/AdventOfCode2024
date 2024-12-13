use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

pub fn run() {
    println!("Day 13:");

    let params = fs::read_to_string("src/input/input13.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|input| parse_input(input))
        .collect::<Vec<_>>();

    let min_tokens = params
        .iter()
        .map(|&((dx_a, dy_a), (dx_b, dy_b), (goal_x, goal_y))| {
            solved_equations(dx_a, dy_a, dx_b, dy_b, goal_x, goal_y)
        })
        .sum::<i64>();

    let min_tokens_2 = params
        .iter()
        .map(|&((dx_a, dy_a), (dx_b, dy_b), (goal_x, goal_y))| {
            solved_equations(
                dx_a,
                dy_a,
                dx_b,
                dy_b,
                goal_x + 10000000000000,
                goal_y + 10000000000000,
            )
        })
        .sum::<i64>();

    println!("Minimum number of tokens: {min_tokens}");
    println!("Minimum number of tokens after multiplying: {min_tokens_2}");

    let min_tokens = solve1(fs::read_to_string("src/input/input13.txt")
        .unwrap().as_str(), 0);
     let min_tokens_2 = solve1(fs::read_to_string("src/input/input13.txt")
        .unwrap().as_str(), 10000000000000);

    println!("Minimum number of tokens: {min_tokens}");
    println!("Minimum number of tokens after multiplying: {min_tokens_2}");
    println!();
}

fn solved_equations(
    dx_a: i64,
    dy_a: i64,
    dx_b: i64,
    dy_b: i64,
    goal_x: i64,
    goal_y: i64,
) -> i64 {
    // solution of solving the equations:
    let b = (goal_y * dx_a - goal_x * dx_b) / (dy_b * dx_a - dy_a * dx_b);
    let a = (goal_x - b * dy_a) / dx_a;

    if (dx_a * a + dy_a * b, dx_b * a + dy_b * b) != (goal_x, goal_y) {
        return 0;
    }

    a * 3 + b
}

fn solve1(input: &str, C: isize) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    for chunk in lines.chunks(4) {
        let mut iter = chunk.iter();
        let cons1 = iter.next().unwrap();
        let cons2 = iter.next().unwrap();
        let prize = iter.next().unwrap();

        let mut split = cons1.split(": ");
        let mut split = split.nth(1).unwrap().split(", ");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        let a0 = a.split("+").nth(1).unwrap().parse::<isize>().unwrap();
        let a1 = b.split("+").nth(1).unwrap().parse::<isize>().unwrap();

        let mut split = cons2.split(": ");
        let mut split = split.nth(1).unwrap().split(", ");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        let b0 = a.split("+").nth(1).unwrap().parse::<isize>().unwrap();
        let b1 = b.split("+").nth(1).unwrap().parse::<isize>().unwrap();

        let mut split = prize.split(": ");
        let mut split = split.nth(1).unwrap().split(", ");
        let x = split.next().unwrap();
        let y = split.next().unwrap();

        let x = x.split("=").nth(1).unwrap().parse::<isize>().unwrap() + C;
        let y = y.split("=").nth(1).unwrap().parse::<isize>().unwrap() + C;

        let denom = a0 * b1 - a1 * b0;
        let num1 = a0 * y - a1 * x;
        let num2 = b1 * x - b0 * y;

        if denom == 0 {
            continue;
        }
        if num1 % denom != 0 || num2 % denom != 0 {
            continue;
        }

        let i = num1 / denom;
        let j = num2 / denom;

        if i < 0 || j < 0 {
            continue;
        }

        sum += i + 3 * j;

        let _ = iter.next();
    }
    sum as usize
}

fn dijkstra(dx_a: i64, dy_a: i64, dx_b: i64, dy_b: i64, goal: (i64, i64)) -> Option<i64> {
    const COST_A: i64 = 3;
    const COST_B: i64 = 1;

    let mut queue = BinaryHeap::<Reverse<(i64, i64, i64)>>::new();
    let mut min_costs = HashMap::<(i64, i64), i64>::new();

    queue.push(Reverse((0, 0, 0)));
    min_costs.insert((0, 0), 0);

    while let Some(Reverse((cost, x, y))) = queue.pop() {
        if (x, y) == goal {
            return Some(cost);
        }

        if Some(&cost) != min_costs.get(&(x, y)) {
            continue;
        }

        // Button A
        let (new_x, new_y) = (x + dx_a, y + dy_a);
        let new_cost = cost + COST_A;

        if new_x <= goal.0
            && new_y <= goal.1
            && min_costs
                .get(&(new_x, new_y))
                .map_or(true, |&min_cost| min_cost > new_cost)
        {
            min_costs.insert((new_x, new_y), new_cost);
            queue.push(Reverse((new_cost, new_x, new_y)));
        }

        // Button B
        let (new_x, new_y) = (x + dx_b, y + dy_b);
        let new_cost = cost + COST_B;

        if new_x <= goal.0
            && new_y <= goal.1
            && min_costs
                .get(&(new_x, new_y))
                .map_or(true, |&min_cost| min_cost > new_cost)
        {
            min_costs.insert((new_x, new_y), new_cost);
            queue.push(Reverse((new_cost, new_x, new_y)));
        }
    }

    None
}

fn parse_input(input: &str) -> ((i64, i64), (i64, i64), (i64, i64)) {
    let (a, b_and_goal) = input.split_once("\r\n").unwrap();
    let (b, goal) = b_and_goal.split_once("\r\n").unwrap();

    let (dx_a_str, dy_a_str) = a.split_once(", Y+").unwrap();
    let (dx_b_str, dy_b_str) = b.split_once(", Y+").unwrap();
    let (goal_x_str, goal_y_str) = goal.split_once(", Y=").unwrap();

    let dx_a = dx_a_str.split_once("X+").unwrap().1.parse().unwrap();
    let dy_a = dy_a_str.parse().unwrap();
    let dx_b = dx_b_str.split_once("X+").unwrap().1.parse().unwrap();
    let dy_b = dy_b_str.parse().unwrap();
    let goal_x = goal_x_str.split_once("X=").unwrap().1.parse().unwrap();
    let goal_y = goal_y_str.parse().unwrap();

    ((dx_a, dy_a), (dx_b, dy_b), (goal_x, goal_y))
    // let v = input
    //     .split(|c: char| !c.is_ascii_digit())
    //     .filter(|w| !w.is_empty())
    //     .map(|w| w.parse().unwrap())
    //     .collect::<Vec<_>>();

    // ((v[0], v[1]), (v[2], v[3]), (v[4], v[5]))
}
