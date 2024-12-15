use std::{collections::HashSet, fs};

type Grid = Vec<Vec<char>>;

struct Puzzle {
    grid: Grid,
    directions: Vec<(isize, isize)>,
    corners: Vec<((isize, isize), (isize, isize), (isize, isize))>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

        let directions = Vec::from([(0, 1), (1, 0), (-1, 0), (0, -1)]);
        let corners = Vec::from([
            ((0, 1), (1, 0), (1, 1)),
            ((1, 0), (0, -1), (1, -1)),
            ((0, -1), (-1, 0), (-1, -1)),
            ((-1, 0), (0, 1), (-1, 1)),
        ]);

        Self {
            grid,
            directions,
            corners,
        }
    }
}

impl Puzzle {
    fn part_1(&self) -> isize {
        let mut total = 0;
        for blob in self.find_blobs() {
            let mut fences = 0;
            for (x, y) in &blob {
                fences += self.find_neighbors(*x, *y);
            }
            total += fences * blob.len() as isize;
            // println!("{} {}", fences, blob.len());
        }
        total
    }

    fn part_2(&self) -> isize {
        let mut total = 0;
        for blob in self.find_blobs() {
            let mut fences = 0;
            for (x, y) in &blob {
                fences += self.find_corners(*x, *y);
            }
            total += fences * blob.len() as isize;
            // println!("{} {}", fences, blob.len());
        }
        total
    }

    fn find_blobs(&self) -> Vec<Vec<(isize, isize)>> {
        let mut found_pos = HashSet::new();
        let mut blobs = Vec::new();
        for (x, y) in self.positions() {
            if found_pos.contains(&(x, y)) {
                continue;
            }
            let mut new_blob = Vec::new();
            found_pos.insert((x, y));
            new_blob.push((x, y));
            let mut investigate = Vec::new();
            for (dx, dy) in &self.directions {
                if self.char_at(x + dx, y + dy) == self.char_at(x, y) {
                    investigate.push((x + dx, y + dy));
                }
            }
            while investigate.len() > 0 {
                let (ix, iy) = investigate.pop().unwrap();
                if found_pos.contains(&(ix, iy)) {
                    continue;
                }
                found_pos.insert((ix, iy));
                new_blob.push((ix, iy));
                for (dx, dy) in &self.directions {
                    if self.char_at(ix + dx, iy + dy) == self.char_at(ix, iy) {
                        investigate.push((ix + dx, iy + dy));
                    }
                }
            }
            blobs.push(new_blob);
        }
        blobs
    }

    fn char_at(&self, x: isize, y: isize) -> Option<char> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn find_neighbors(&self, x: isize, y: isize) -> isize {
        let mut neighbor_count = 4;
        for (dx, dy) in &self.directions {
            if self.char_at(x + dx, y + dy) == self.char_at(x, y) {
                neighbor_count -= 1;
            }
        }
        neighbor_count
    }

    fn find_corners(&self, x: isize, y: isize) -> isize {
        let mut corner_count = 0;
        for ((dx0, dy0), (dx1, dy1), (dx2, dy2)) in &self.corners {
            if self.char_at(x + dx0, y + dy0) != self.char_at(x, y)
                && self.char_at(x + dx1, y + dy1) != self.char_at(x, y)
            {
                corner_count += 1;
            }

            if self.char_at(x + dx0, y + dy0) == self.char_at(x, y)
                && self.char_at(x + dx1, y + dy1) == self.char_at(x, y)
                && self.char_at(x + dx2, y + dy2) != self.char_at(x, y)
            {
                corner_count += 1;
            }
        }
        corner_count
    }

    fn positions(&self) -> impl Iterator<Item = (isize, isize)> {
        let x_size = self.grid.len();
        let y_size = self.grid[0].len();
        (0..x_size).flat_map(move |x| (0..y_size).map(move |y| (x as isize, y as isize)))
    }
}

pub fn run() {
    println!("Solution part 1: {}", Puzzle::from("src/input/input12.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("src/input/input12.txt").part_2());
}