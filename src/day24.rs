use std::{
    collections::{HashMap, VecDeque},
    fs,
    ops::{BitAnd, BitOr, BitXor},
};

pub fn run() {
    println!("Day 24:");
    let input = fs::read_to_string("src/input/input24.txt").unwrap();

    let (wire_vals, mut operations) = parse(&input);
    
    let correct_res = correct_result(&wire_vals);
    let swaps = find_swaps(&wire_vals, &mut operations, correct_res);
    
    let result = simulate(wire_vals, &mut operations);
    let z_num = get_result(&result, "z");
    
    compare(&correct_res, &z_num);
    
    println!("Decimal output: {z_num}");
   // println!("Swapped wires: {swaps}");
    println!();
}

fn compare(lhs: &isize, rhs: &isize)  {
    let a = (0..64).rev().map(|n| lhs >> n & 1).collect::<Vec<_>>();
    let b = (0..64).rev().map(|n| rhs >> n & 1).collect::<Vec<_>>();
    
    let c = a.iter().enumerate().map(|(idx, bit)| b[idx] == *bit).map(|b| match b {
        true => 1,
        false => 0
    }).collect::<Vec<_>>();
    
    println!("{:?}", c);
}

fn find_swaps(
    wire_vals: &HashMap<&str, i32>,
    operations: &mut VecDeque<((&str, &str, &str), fn(i32, i32) -> i32)>,
    correct_res: isize
)  {
    println!("correct result: {correct_res}");

    
    
}

fn get_result(result: &HashMap<&str, i32>, res_prefix: &str) -> isize {
    let mut z_results = result
        .iter()
        .filter(|(id, _)| id.starts_with(res_prefix))
        .collect::<Vec<_>>();

    z_results.sort();

    z_results
        .iter()
        .map(|(_, num)| **num)
        .enumerate()
        .fold(0, |acc, (idx, num)| {
            acc + 2_isize.pow(idx as u32) * num as isize
        })
}

fn simulate<'a>(
    mut wire_vals: HashMap<&'a str, i32>,
    operations: &mut VecDeque<((&'a str, &'a str, &'a str), fn(i32, i32) -> i32)>,
) -> HashMap<&'a str, i32> {
    while let Some(((a, b, res), op)) = operations.pop_front() {
        if let Some(val_a) = wire_vals.get(a) {
            if let Some(val_b) = wire_vals.get(b) {
                wire_vals.insert(res, op(*val_a, *val_b));
                continue;
            }
        }

        operations.push_back(((a, b, res), op));
    }

    wire_vals
}

fn parse(
    input: &str,
) -> (
    HashMap<&str, i32>,
    VecDeque<((&str, &str, &str), fn(i32, i32) -> i32)>,
) {
    let (initial, connections) = input.split_once("\r\n\r\n").unwrap();

    let wire_vals = initial.lines().fold(HashMap::new(), |mut acc, line| {
        let (id, val) = line.split_once(": ").unwrap();
        acc.insert(id, val.parse().unwrap());
        acc
    });

    let operations = connections
        .lines()
        .map(|line| {
            let (op, res) = line.split_once(" -> ").unwrap();

            let args = op.split(" ").collect::<Vec<_>>();
            let fun = match args[1] {
                "AND" => and,
                "OR" => or,
                "XOR" => xor,
                _ => unreachable!(),
            };

            ((args[0], args[2], res), fun)
        })
        .collect();

    (wire_vals, operations)
}

fn and(lhs: i32, rhs: i32) -> i32 {
    lhs.bitand(rhs)
}

fn or(lhs: i32, rhs: i32) -> i32 {
    lhs.bitor(rhs)
}

fn xor(lhs: i32, rhs: i32) -> i32 {
    lhs.bitxor(rhs)
}

fn correct_result(wire_vals: &HashMap<&str, i32>) -> isize {
    let (x, y) = wire_vals.iter().partition(|(id, _)| id.starts_with("x"));

    get_result(&x, "x") + get_result(&y, "y")
}
