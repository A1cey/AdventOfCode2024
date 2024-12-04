use std::fs;

pub fn run() {
    println!("Day 4:");

    let mut sum = 0;

    let input = fs::read_to_string("src/input4.txt").unwrap();
    let line_len = input.find("\n").unwrap() - 1;

    let mut transposed_lines: Vec<Vec<char>> = vec![];
    let mut diagonal_lines: Vec<Vec<char>> = vec![];
    let mut diagonal_lines_rev: Vec<Vec<char>> = vec![];

    input.lines().enumerate().for_each(|(line_idx, line)| {
        sum += read_xmas_from_line(line);
        sum += read_xmas_from_line(line.chars().rev().collect::<String>().as_str());

        line.chars().enumerate().for_each(|(idx, c)| {
            match transposed_lines.get(idx) {
                Some(_) => transposed_lines[idx].push(c),
                None => transposed_lines.push(vec![c]),
            };

            let i = if usize::checked_sub(idx, line_idx).is_none() {
                line_len + line_idx - idx - 1
            } else {
                idx - line_idx
            };

            match diagonal_lines.get(i) {
                Some(_) => diagonal_lines[i].push(c),
                None => diagonal_lines.push(vec![c]),
            }
        });

        line.chars().rev().enumerate().for_each(|(idx, c)| {
            let i = if usize::checked_sub(idx, line_idx).is_none() {
                line_len + line_idx - idx - 1
            } else {
                idx - line_idx
            };

            match diagonal_lines_rev.get(i) {
                Some(_) => diagonal_lines_rev[i].push(c),
                None => diagonal_lines_rev.push(vec![c]),
            }
        });
    });

    sum += read_xmas_from_vec(&transposed_lines);
    sum += read_xmas_from_vec(&diagonal_lines);
    sum += read_xmas_from_vec(&diagonal_lines_rev);

    println!("{} 'XMAS' gefunden.", sum);
    println!();
}

fn read_xmas_from_vec(v: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    v.iter().for_each(|line_vec| {
        sum += read_xmas_from_line(line_vec.iter().collect::<String>().as_str());
        sum += read_xmas_from_line(line_vec.iter().rev().collect::<String>().as_str())
    });

    sum
}

fn read_xmas_from_line(line: &str) -> i32 {
    let word_len = "XMAS".len();

    if line.len() < word_len {
        return 0;
    }
    let mut sum = 0;
    let mut idx = 0;

    while idx <= line.len() - word_len {
        if line[idx..].starts_with("XMAS") {
            sum += 1;
            idx += word_len;
        } else {
            idx += 1;
        }
    }

    sum
}
