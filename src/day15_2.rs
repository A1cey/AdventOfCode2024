use std::{cmp::Ordering, fmt::Display, fs};

type Map = Vec<Vec<Tile>>;
type WideMap = Vec<Vec<WideTile>>;

enum Tile {
    Empty,
    Robot,
    Box,
    Wall,
}

#[derive(Copy, Clone)]
enum WideTile {
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            other => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::Box,
            '.' => Tile::Empty,
            '@' => Tile::Robot,
            '#' => Tile::Wall,
            other => unreachable!("{other}"),
        }
    }
}

impl From<char> for WideTile {
    fn from(value: char) -> Self {
        match value {
            '[' => WideTile::BoxLeft,
            ']' => WideTile::BoxRight,
            '.' => WideTile::Empty,
            '@' => WideTile::Robot,
            '#' => WideTile::Wall,
            other => unreachable!("{other}"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Robot => write!(f, "@"),
            Tile::Box => write!(f, "O"),
            Tile::Wall => write!(f, "#"),
        }
    }
}

impl std::fmt::Display for WideTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WideTile::Empty => write!(f, "."),
            WideTile::Robot => write!(f, "@"),
            WideTile::BoxLeft => write!(f, "["),
            WideTile::BoxRight => write!(f, "]"),
            WideTile::Wall => write!(f, "#"),
        }
    }
}

fn print_grid<T: Display>(m: &[Vec<T>]) {
    for r in m {
        for c in r {
            print!("{c}");
        }
        println!();
    }
}

fn get_cell<T>(xy: (usize, usize), m: &[Vec<T>]) -> &T {
    &m[xy.1][xy.0]
}

fn char_to_wide_tiles(value: char) -> [WideTile; 2] {
    match value {
        'O' => [WideTile::BoxLeft, WideTile::BoxRight],
        '.' => [WideTile::Empty, WideTile::Empty],
        '@' => [WideTile::Robot, WideTile::Empty],
        '#' => [WideTile::Wall, WideTile::Wall],
        other => unreachable!("{other}"),
    }
}

fn parse_map(s: &str) -> Map {
    s.lines()
        .map(|line| line.chars().map(Into::into).collect())
        .collect()
}

fn parse_wide_map(s: &str) -> WideMap {
    s.lines()
        .map(|line| line.chars().flat_map(char_to_wide_tiles).collect())
        .collect()
}

/// (map, moves)
fn parse_input(s: &str) -> (Map, Vec<Direction>) {
    let (map, moves) = s.split_once("\r\n\r\n").unwrap();
    let map = parse_map(map);

    let moves = moves
        .chars()
        .filter(|c| ['<', '>', '^', 'v'].contains(c))
        .map(Into::into)
        .collect();
    (map, moves)
}

/// (map, moves)
fn parse_input_wide(s: &str) -> (WideMap, Vec<Direction>) {
    let (map, moves) = s.split_once("\r\n\r\n").unwrap();
    let map = parse_wide_map(map);
    let moves = moves
        .chars()
        .filter(|c| ['<', '>', '^', 'v'].contains(c))
        .map(Into::into)
        .collect();
    (map, moves)
}

/// (x, y)
fn robot_coord(m: &Map) -> (usize, usize) {
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, t) in r.iter().enumerate() {
            if matches!(t, Tile::Robot) {
                return (c_idx, r_idx);
            }
        }
    }
    unreachable!();
}

/// (x, y)
fn robot_coord_wide(m: &WideMap) -> (usize, usize) {
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, t) in r.iter().enumerate() {
            if matches!(t, WideTile::Robot) {
                return (c_idx, r_idx);
            }
        }
    }
    unreachable!();
}

fn next_coord(xy: (usize, usize), mve: &Direction) -> (usize, usize) {
    match mve {
        Direction::Up => (xy.0, xy.1 - 1),
        Direction::Down => (xy.0, xy.1 + 1),
        Direction::Left => (xy.0 - 1, xy.1),
        Direction::Right => (xy.0 + 1, xy.1),
    }
}

fn next_coords<const N: usize>(
    mut coords: [(usize, usize); N],
    mve: &Direction,
    map: &WideMap,
) -> [(usize, usize); N] {
    debug_assert!(matches!(mve, Direction::Up | Direction::Down));
    coords.map(|xy| next_coord(xy, mve))
}

fn next_empty_tile_coord(
    mut xy: (usize, usize),
    mve: &Direction,
    map: &Map,
) -> Option<(usize, usize)> {
    loop {
        xy = next_coord(xy, mve);
        match get_cell(xy, map) {
            Tile::Box => continue,
            Tile::Wall => return None,
            Tile::Empty => return Some(xy),
            Tile::Robot => unreachable!(),
        }
    }
}

fn next_empty_tile_coord_wide_horiz(
    mut xy: (usize, usize),
    mve: &Direction,
    map: &WideMap,
) -> Option<(usize, usize)> {
    debug_assert!(matches!(mve, Direction::Left | Direction::Right));
    loop {
        xy = next_coord(xy, mve);
        match get_cell(xy, map) {
            WideTile::BoxLeft => continue,
            WideTile::BoxRight => continue,
            WideTile::Wall => return None,
            WideTile::Empty => return Some(xy),
            WideTile::Robot => unreachable!(),
        }
    }
}

fn check_moves_vert_wide<const N: usize>(
    mut coords: [(usize, usize); N],
    mve: &Direction,
    map: &WideMap,
    mut target_moves: Vec<(usize, usize)>,
) -> (bool, Vec<(usize, usize)>) {
    debug_assert!(matches!(mve, Direction::Up | Direction::Down));
    println!("checking moves at coords {:?}", coords);
    let next_coords = next_coords(coords, mve, map);
    let mut next_can_move = vec![];
    for xy in next_coords {
        let tile = get_cell(xy, map);
        match tile {
            WideTile::Empty => {
                next_can_move.push(true);
                target_moves.push(xy);
            }
            WideTile::BoxLeft => {
                let box_coords = [xy, (xy.0 + 1, xy.1)];
                target_moves.push(xy);
                let mut a;
                (a, target_moves) = check_moves_vert_wide(box_coords, mve, map, target_moves);
                next_can_move.push(a);
            }
            WideTile::BoxRight => {
                let box_coords = [xy, (xy.0 - 1, xy.1)];
                target_moves.push(xy);
                let mut a;
                (a, target_moves) = check_moves_vert_wide(box_coords, mve, map, target_moves);
                next_can_move.push(a);
            }
            WideTile::Wall => return (false, target_moves),
            WideTile::Robot => unreachable!(),
        }
    }
    println!("Check moves outcome: {:?}", target_moves);
    (next_can_move.iter().all(|b| *b), target_moves)
}

fn apply_move(mut map: Map, mve: Direction) -> Map {
    let robot_pos = robot_coord(&map);
    let maybe_next_pos = next_coord(robot_pos, &mve);
    let next_tile = get_cell(maybe_next_pos, &map);
    match next_tile {
        Tile::Empty => {
            map[maybe_next_pos.1][maybe_next_pos.0] = Tile::Robot;
            map[robot_pos.1][robot_pos.0] = Tile::Empty;
            map
        }
        Tile::Box => {
            let Some(next_empty) = next_empty_tile_coord(maybe_next_pos, &mve, &map) else {
                return map;
            };
            map[maybe_next_pos.1][maybe_next_pos.0] = Tile::Robot;
            map[robot_pos.1][robot_pos.0] = Tile::Empty;
            map[next_empty.1][next_empty.0] = Tile::Box;
            map
        }
        Tile::Wall => map,
        Tile::Robot => unreachable!(),
    }
}

fn shift_boxes_horiz(map: &mut [Vec<WideTile>], start: (usize, usize), end: (usize, usize)) {
    println!("Shuffling! start {:?}, end {:?}", start, end);
    let delta = -(end.0 as isize - start.0 as isize).signum();
    let mut i = end.0 as isize;
    loop {
        if i as usize == start.0 {
            break;
        }
        println!("i: {i}");
        map[start.1][i as usize] = map[start.1][(i + delta) as usize];
        map[start.1][(i + delta) as usize] = WideTile::Empty;
        i += delta;
    }
}

fn shift_boxes_vert(
    map: &mut [Vec<WideTile>],
    mve: &Direction,
    mut target_moves: Vec<(usize, usize)>,
) {
    println!("Shuffling moves - before dedup {:?}", target_moves);
    target_moves.sort_by(|(x1, y1), (x2, y2)| {
        let order = if matches!(mve, Direction::Down) {
            y1.cmp(y2)
        } else {
            y2.cmp(y1)
        };
        if matches!(order, Ordering::Equal) {
            return x2.cmp(x1);
        }
        order
    });
    target_moves.dedup();
    println!("Shuffling moves - after dedup {:?}", target_moves);
    let rev_delta = match mve {
        Direction::Up => 1,
        Direction::Down => -1,
        Direction::Left | Direction::Right => unreachable!(),
    };
    for mve in target_moves.iter().rev() {
        println!("mve: {:?}", mve);
        map[mve.1][mve.0] = map[mve.1.checked_add_signed(rev_delta).unwrap()][mve.0];
        map[mve.1.checked_add_signed(rev_delta).unwrap()][mve.0] = WideTile::Empty
    }
}

fn apply_move_wide(mut map: Vec<Vec<WideTile>>, mve: Direction) -> Vec<Vec<WideTile>> {
    let robot_pos = robot_coord_wide(&map);
    let maybe_next_pos = next_coord(robot_pos, &mve);
    let next_tile = get_cell(maybe_next_pos, &map);
    match next_tile {
        WideTile::Empty => {
            map[maybe_next_pos.1][maybe_next_pos.0] = WideTile::Robot;
            map[robot_pos.1][robot_pos.0] = WideTile::Empty;
            map
        }
        WideTile::BoxLeft | WideTile::BoxRight
            if matches!(mve, Direction::Left | Direction::Right) =>
        {
            let Some(next_empty) = next_empty_tile_coord_wide_horiz(maybe_next_pos, &mve, &map)
            else {
                return map;
            };
            shift_boxes_horiz(&mut map, robot_pos, next_empty);
            map
        }
        WideTile::BoxLeft => {
            let box_coords = [maybe_next_pos, (maybe_next_pos.0 + 1, maybe_next_pos.1)];
            let (a, moves) = check_moves_vert_wide(box_coords, &mve, &map, vec![]);
            if a {
                shift_boxes_vert(&mut map, &mve, moves);
                map[maybe_next_pos.1][maybe_next_pos.0] = WideTile::Robot;
                map[robot_pos.1][robot_pos.0] = WideTile::Empty;
            }
            map
        }
        WideTile::BoxRight => {
            let box_coords = [maybe_next_pos, (maybe_next_pos.0 - 1, maybe_next_pos.1)];
            let (a, moves) = check_moves_vert_wide(box_coords, &mve, &map, vec![]);
            if a {
                shift_boxes_vert(&mut map, &mve, moves);
                map[maybe_next_pos.1][maybe_next_pos.0] = WideTile::Robot;
                map[robot_pos.1][robot_pos.0] = WideTile::Empty;
            }
            map
        }
        WideTile::Wall => map,
        WideTile::Robot => unreachable!(),
    }
}

fn total_gps_coords(m: &Map) -> usize {
    let mut total = 0;
    let width = m[0].len();
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if matches!(c, Tile::Box) {
                total += 100 * r_idx + c_idx;
            }
        }
    }
    total
}

fn total_gps_coords_wide(m: &WideMap) -> usize {
    let mut total = 0;
    let width = m[0].len();
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if matches!(c, WideTile::BoxLeft) {
                total += 100 * r_idx + c_idx;
            }
        }
    }
    total
}

fn solve_part_1(s: &str) -> usize {
    let (mut map, moves) = parse_input(s);
    for mve in moves {
        map = apply_move(map, mve);
    }
    total_gps_coords(&map)
}

fn solve_part_2(s: &str) -> usize {
    let (mut map, moves) = parse_input_wide(s);
    for mve in moves {
        println!("Next move is {:?}", mve);
        map = apply_move_wide(map, mve);
    }
    total_gps_coords_wide(&map)
}

pub fn run() {
    let input = fs::read_to_string("src/input/input15.txt").unwrap();

    println!("Total GPS coords: {}", solve_part_1(&input));
    println!("Total GPS coords: {}", solve_part_2(&input));
}
