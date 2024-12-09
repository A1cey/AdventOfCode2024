use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
struct File {
    val: isize,
    len: usize,
}

impl File {
    const fn new(val: isize, len: usize) -> File {
        File { val, len }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty(usize),
    File(File),
}

pub fn run() {
    println!("Day 9:");

    let input = fs::read_to_string("src/input/input9.txt")
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    println!(
        "The first checksum is: {}",
        calculate_first_checksum(&input)
    );
    println!(
        "The second checksum is: {}",
        calculate_second_checksum(&input)
    );
    println!();
}

fn calculate_first_checksum(input: &[u32]) -> isize {
    let mut is_id = true;
    let mut idx = 0;
    let mut compressed_block = input.iter().fold(Vec::new(), |mut acc, &num| {
        if is_id {
            (0..num).for_each(|_| acc.push(Some(idx)));
            idx += 1;
        } else {
            (0..num).for_each(|_| acc.push(None));
        }

        is_id = !is_id;
        acc
    });

    for idx in 0..compressed_block.len() {
        if idx < compressed_block.len() && compressed_block[idx].is_none() {
            while compressed_block.last().unwrap().is_none() {
                compressed_block.pop();
            }
            compressed_block.swap_remove(idx);
        }
    }

    compressed_block
        .iter()
        .enumerate()
        .map(|(idx, &id)| id.unwrap() * (idx as isize))
        .sum()
}

fn calculate_second_checksum(input: &[u32]) -> isize {
    let mut is_id = true;
    let mut idx = 0;

    let mut compressed_block = input.iter().fold(Vec::new(), |mut acc, &num| {
        if is_id {
            acc.push(Block::File(File::new(idx as isize, num as usize)));
            idx += 1;
        } else {
            acc.push(Block::Empty(num as usize));
        }

        is_id = !is_id;
        acc
    });

    let mut idx = 1;

    while idx <= compressed_block.len() {
        let block_idx = compressed_block.len() - idx;

        if let Block::File(file) = compressed_block[block_idx] {
            if let Some(empty_idx) = compressed_block[..block_idx]
                .iter()
                .position(|block| matches!(block, Block::Empty(len) if *len >= file.len))
            {
                compressed_block.insert(empty_idx, Block::File(file));
                if let Block::Empty(len) = compressed_block[empty_idx + 1] {
                    compressed_block[empty_idx + 1] = Block::Empty(len.saturating_sub(file.len));
                }
                compressed_block.remove(block_idx + 1);
                compressed_block.insert(block_idx + 1, Block::Empty(file.len));
            }
        };

        idx += 1;
    }

    compressed_block
        .iter()
        .flat_map(|block| match block {
            Block::File(file) => vec![Some(file.val); file.len],
            Block::Empty(len) => vec![None; *len],
        })
        .enumerate()
        .map(|(idx, block)| block.map(|num| num * (idx as isize)).unwrap_or(0))
        .sum()
}
