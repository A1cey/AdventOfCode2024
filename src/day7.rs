use std::fs;

#[derive(Debug)]
struct Equation {
    result: usize,
    nums: Vec<usize>,
}

impl Equation {
    fn new(result: usize, nums: Vec<usize>) -> Equation {
        Equation { result, nums }
    }
}

pub fn run() {
    println!("Day 7:");

    let result = fs::read_to_string("src/input/input7.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (result, equation) = line.split_once(": ").unwrap();
            let nums = equation
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Equation::new(result.parse().unwrap(), nums)
        })
        .filter(|eq| solve(&eq.nums, 1, eq.result, eq.nums[0]))
        .fold(0, |sum, eq| sum + eq.result);

    println!("Total calibration result: {}", result);
    println!()
}

fn solve(nums: &[usize], idx: usize, result: usize, curr_result: usize) -> bool {
    if idx == nums.len() {
        return curr_result == result;
    }

    let add_result = solve(nums, idx + 1, result, curr_result + nums[idx]);
    let mul_result = solve(nums, idx + 1, result, curr_result * nums[idx]);
    let concat_result = solve(
        nums,
        idx + 1,
        result,
        curr_result * 10_usize.pow(nums[idx].ilog10() + 1) + nums[idx],
    );

    add_result || mul_result || concat_result
}
