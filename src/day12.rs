use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Tile {
    tile_type: char,
    row: usize,
    col: usize,
}

impl Tile {
    const fn new(tile_type: char, row: usize, col: usize) -> Tile {
        Tile {
            tile_type,
            row,
            col,
        }
    }
}

pub fn run() {
    println!("Day 12:");

    let mut field = fs::read_to_string("src/input/input12.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, name)| Tile::new(name, row, col))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut regions = vec![];
    let mut visited = HashSet::new();

    for i in 0..field.len() {
        for j in 0..field[0].len() {
            let curr_tile = field[i][j];

            if !visited.contains(&curr_tile) {
                let (fences, region) = get_regions(&field, curr_tile, &mut visited);
                regions.push((fences, region));
            }
        }
    }


    // regions.iter().for_each(|region| {
    //     println!(
    //         "Tiles: {}; Fences: {}",
    //         region.1.len(),
    //         region.0 //count_corners(&region.1, field.len(), field[0].len())
    //     );
    //     // region
    //     //     .1
    //     //     .iter()
    //     //     .for_each(|field| println!("{}: {}|{}", field.tile_type, field.row, field.col));
    //     println!("----------------------");
    // });


    let price = regions
        .iter()
        .map(|region| region.0 * region.1.len() as i32)
        .sum::<i32>();

    println!("Total price: {price}");
    println!();
}

fn get_regions(field: &[Vec<Tile>], start: Tile, visited: &mut HashSet<Tile>) -> (i32, Vec<Tile>) {
    let mut region = Vec::new();
    let mut queue = VecDeque::new();
    let mut fences = 0;

    queue.push_back(start);
    visited.insert(start);

    while let Some(tile) = queue.pop_front() {
        region.push(tile);

        let neighbors = [
            (tile.row > 0, (tile.row.saturating_sub(1), tile.col)),
            (tile.row < field.len() - 1, (tile.row + 1, tile.col)),
            (tile.col > 0, (tile.row, tile.col.saturating_sub(1))),
            (tile.col < field[0].len() - 1, (tile.row, tile.col + 1)),
        ];

        for (is_valid, (new_row, new_col)) in neighbors {
            if !is_valid {
                fences += 1;
                continue;
            }

            let neighbor = field[new_row][new_col];

            if neighbor.tile_type != tile.tile_type {
                fences += 1;
                continue;
            }

            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    (fences, region)
}

fn count_corners(region_tiles: &[Tile], max_row: usize, max_col: usize) -> usize {
    region_tiles
        .iter()
        .filter(|tile| {
            let up = tile.row.checked_sub(1).map(|row| (row, tile.col));
            let down = if tile.row + 1 <= max_row {
                Some((tile.row + 1, tile.col))
            } else {
                None
            };
            let left = tile.col.checked_sub(1).map(|col| (tile.row, col));
            let right = if tile.col + 1 <= max_col {
                Some((tile.row, tile.col + 1))
            } else {
                None
            };

            let has_up = up.map_or(false, |(row, col)| {
                region_tiles.contains(&Tile::new(tile.tile_type, row, col))
            });
            let has_down = down.map_or(false, |(row, col)| {
                region_tiles.contains(&Tile::new(tile.tile_type, row, col))
            });
            let has_left = left.map_or(false, |(row, col)| {
                region_tiles.contains(&Tile::new(tile.tile_type, row, col))
            });
            let has_right = right.map_or(false, |(row, col)| {
                region_tiles.contains(&Tile::new(tile.tile_type, row, col))
            });

            (!has_up && !has_left)
                || (!has_up && !has_right)
                || (!has_down && !has_left)
                || (!has_down && !has_right)
        })
        .count()
}
