use std::fs;

const NUM_CHARS: usize = 16;

// generated using old solution
const LUT: [[usize; 6]; NUM_CHARS * NUM_CHARS] = [
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [19, 50, 33, 0, 0, 0],
    [18, 35, 51, 49, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [21, 83, 51, 49, 0, 0],
    [19, 53, 81, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [21, 85, 83, 51, 49, 0],
    [19, 53, 85, 81, 0, 0],
    [21, 85, 81, 0, 0, 0],
    [21, 85, 85, 83, 51, 49],
    [19, 53, 85, 85, 81, 0],
    [21, 85, 85, 81, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [21, 84, 65, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 68, 69, 81, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 69, 81, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [19, 53, 81, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 35, 49, 0, 0, 0],
    [18, 36, 65, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [21, 83, 49, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [21, 84, 65, 0, 0, 0],
    [21, 85, 83, 49, 0, 0],
    [21, 85, 81, 0, 0, 0],
    [21, 85, 84, 65, 0, 0],
    [21, 85, 85, 83, 49, 0],
    [21, 85, 85, 81, 0, 0],
    [21, 85, 85, 84, 65, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 68, 66, 33, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 66, 33, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [20, 68, 65, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [21, 84, 65, 0, 0, 0],
    [21, 84, 68, 65, 0, 0],
    [21, 85, 81, 0, 0, 0],
    [21, 85, 84, 65, 0, 0],
    [21, 85, 84, 68, 65, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 36, 65, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [19, 53, 81, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [21, 84, 65, 0, 0, 0],
    [19, 53, 85, 81, 0, 0],
    [21, 85, 81, 0, 0, 0],
    [21, 85, 84, 65, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [19, 50, 33, 0, 0, 0],
    [19, 51, 49, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [19, 51, 53, 81, 0, 0],
    [19, 53, 81, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [19, 51, 53, 85, 81, 0],
    [19, 53, 85, 81, 0, 0],
    [21, 85, 81, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 68, 66, 34, 33, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 66, 34, 33, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [18, 36, 65, 0, 0, 0],
    [18, 36, 68, 65, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [20, 68, 65, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [21, 84, 65, 0, 0, 0],
    [21, 84, 68, 65, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 34, 36, 65, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 34, 33, 0, 0, 0],
    [19, 50, 33, 0, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [18, 36, 65, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [19, 53, 81, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [21, 84, 65, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 34, 33, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [19, 50, 34, 33, 0, 0],
    [19, 51, 50, 33, 0, 0],
    [19, 50, 33, 0, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [19, 51, 49, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [19, 51, 53, 81, 0, 0],
    [19, 53, 81, 0, 0, 0],
    [21, 81, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 68, 66, 34, 34, 33],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [20, 66, 34, 34, 33, 0],
    [18, 34, 33, 0, 0, 0],
    [18, 34, 36, 65, 0, 0],
    [18, 34, 36, 68, 65, 0],
    [18, 33, 0, 0, 0, 0],
    [18, 36, 65, 0, 0, 0],
    [18, 36, 68, 65, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [20, 68, 65, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 34, 34, 36, 65, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 34, 34, 33, 0, 0],
    [19, 50, 34, 33, 0, 0],
    [18, 34, 33, 0, 0, 0],
    [18, 34, 36, 65, 0, 0],
    [19, 50, 33, 0, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [18, 36, 65, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
    [20, 65, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [18, 34, 34, 33, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [19, 50, 34, 34, 33, 0],
    [19, 51, 50, 34, 33, 0],
    [19, 50, 34, 33, 0, 0],
    [18, 34, 33, 0, 0, 0],
    [19, 51, 50, 33, 0, 0],
    [19, 50, 33, 0, 0, 0],
    [18, 33, 0, 0, 0, 0],
    [19, 51, 49, 0, 0, 0],
    [19, 49, 0, 0, 0, 0],
    [17, 0, 0, 0, 0, 0],
];

fn encode(c: char) -> usize {
    match c {
        'A' => 1,
        'v' => 2,
        '<' => 3,
        '>' => 4,
        '^' => 5,
        '0' => 6,
        '1' => 7,
        '2' => 8,
        '3' => 9,
        '4' => 10,
        '5' => 11,
        '6' => 12,
        '7' => 13,
        '8' => 14,
        '9' => 15,
        _ => unreachable!(),
    }
}

fn solve_keycode(counts: &Vec<i128>, depth: i32) -> i128 {
    let mut counts = counts.clone();
    for _ in 0..depth {
        let mut new_counts = vec![0; NUM_CHARS * NUM_CHARS];
        for i in 1..NUM_CHARS * NUM_CHARS {
            for j in 0..6 {
                new_counts[LUT[i][j]] += counts[i];
            }
        }
        counts = new_counts;
    }
    counts[1..].iter().sum()
}

// count transitions of the sequence
// each sequence (a count of transitions leads to new transitions via a LUT)
//  029A = (0,2):1,(2,9):1,(9,A):1   and leads to e.g. (<,A):1,(A,^):1,(^,A):2,....
// since there is exactly one optimal solution for each transition (_independent of depth_)
//  all transitions can be precalculated
//  TODO: why is there such a unique solution (this was exhaustively checked using the old solution)
 fn solve(data: &str) -> (String, String) {
    let mut counts = vec![0; NUM_CHARS * NUM_CHARS];
    for inp in data.lines() {
        let mut inp = inp.trim();
        let numval = inp[..inp.len() - 1].parse::<i128>().unwrap();

        let mut old_code = encode('A');
        for c in inp.chars() {
            let curr_code = encode(c);
            counts[old_code * NUM_CHARS + curr_code] += numval;
            old_code = curr_code;
        }
    }

    let p1 = solve_keycode(&counts, 3);
    let p2 = solve_keycode(&counts, 26);
    (p1.to_string(), p2.to_string())
}


pub fn run() {
    let (part1, part2) = solve(&fs::read_to_string("src/input/input21.txt").unwrap());
    println!("part 1: {}, part 2: {}", part1,part2);
    println!()
}
