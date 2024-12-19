use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn run() {
    println!("Day 19:");

    let input = fs::read_to_string("src/input/input19.txt").unwrap();

    let (patterns, designs) = parse_input(&input);

    let possible_designs = find_possible_designs(&patterns, &designs);    

    println!("There are {} possible designs.",possible_designs.iter().filter(|&&num| num != 0).count());
    println!("There are {} different ways to make the designs.", possible_designs.iter().sum::<usize>());
    println!();
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns_str, designs_str) = input.split_once("\r\n\r\n").unwrap();

    let patterns = patterns_str.split(", ").collect();
    let designs = designs_str.lines().collect();

    (patterns, designs)
}

fn find_possible_designs(patterns: &[&str], designs: &[&str]) -> Vec<usize> {
    // println!("patterns: {:?}", patterns);
  designs
        .iter()
        .map(|design| test_patterns_dfs(design, patterns, &mut HashMap::<&str, usize>::new())).collect()
}

fn test_patterns(design: &str, patterns: &[&str]) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(design);

    while let Some(design_part) = queue.pop_front() {
        // println!("queue: {:?}",queue);
        // println!("Current design: {design_part}");
        if design_part.is_empty() {
            return true;
        }

        patterns.iter().for_each(|pattern| {
            if let Some(rest_design_part) = design_part.strip_prefix(pattern) {
                // println!("found matching pattern: {pattern}, design:  {rest_design_part}");
                queue.push_back(rest_design_part);
            }
        });
    }

    false
}

fn test_patterns_dfs<'a>(
    design: &'a str,
    patterns: &[&str],
    found_designs: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&num) = found_designs.get(design) {
        return num;
    }

    let mut result = 0;

    for pattern in patterns.iter() {
        if let Some(res) = design.strip_prefix(pattern).map(|rest|test_patterns_dfs(rest, patterns, found_designs)) {
            // println!("found matching pattern: {pattern}, design:  {rest_design_part}");
            result += res;
        }
    }

    found_designs.insert(design, result);
    result
}

