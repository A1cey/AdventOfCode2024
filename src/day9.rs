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
        .map(|num| num.unwrap())
        .enumerate()
        .fold(0, |acc, (idx, num)| acc + num * (idx as isize))
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
            if let Some(empty_idx) =
                compressed_block[..block_idx]
                    .iter()
                    .position(|block| match block {
                        Block::Empty(len) => *len >= file.len,
                        Block::File(_) => false,
                    })
            {
                compressed_block.insert(empty_idx, Block::File(file));
                // remove this ? should always be Block::Empty
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
        .fold(Vec::new(), |mut acc, block| {
            match block {
                Block::File(file) => (0..file.len).for_each(|_| acc.push(Some(file.val))),
                Block::Empty(len) => (0..*len).for_each(|_| acc.push(None)),
            }
            acc
        })
        .iter()
        .enumerate()
        .fold(0, |mut acc, (idx, block)| {
            if let Some(num) = block {
                acc += num * (idx as isize);
            }
            acc
        })
}
