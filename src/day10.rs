use std::collections::HashSet;

use itertools::iproduct;
use parse_display::FromStr;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, FromStr, PartialEq, Eq)]
enum Tile {
    #[display("S")]
    Start,
    #[display(".")]
    Ground,
    #[display("|")]
    Vertical,
    #[display("-")]
    Horizontal,
    #[display("L")]
    NorthEast,
    #[display("J")]
    NorthWest,
    #[display("7")]
    SouthWest,
    #[display("F")]
    SouthEast,
}

impl Tile {
    fn next_direction(&self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (Self::Vertical, Direction::North) => Some(Direction::North),
            (Self::Vertical, Direction::South) => Some(Direction::South),
            (Self::Horizontal, Direction::East) => Some(Direction::East),
            (Self::Horizontal, Direction::West) => Some(Direction::West),

            (Self::NorthEast, Direction::South) => Some(Direction::East),
            (Self::NorthEast, Direction::West) => Some(Direction::North),

            (Self::NorthWest, Direction::South) => Some(Direction::West),
            (Self::NorthWest, Direction::East) => Some(Direction::North),

            (Self::SouthWest, Direction::North) => Some(Direction::West),
            (Self::SouthWest, Direction::East) => Some(Direction::South),

            (Self::SouthEast, Direction::North) => Some(Direction::East),
            (Self::SouthEast, Direction::West) => Some(Direction::South),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_next_pos_from_direction(
    pos: (usize, usize),
    direction: Direction,
    grid_size: (usize, usize),
) -> Option<(usize, usize)> {
    match direction {
        Direction::North => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        Direction::East => {
            if pos.1 == grid_size.1 - 1 {
                None
            } else {
                Some((pos.0, pos.1 + 1))
            }
        }
        Direction::South => {
            if pos.0 == grid_size.0 - 1 {
                None
            } else {
                Some((pos.0 + 1, pos.1))
            }
        }
        Direction::West => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
    }
}

fn get_loop_path(
    map: &[Vec<Tile>],
    start: (usize, usize),
    grid_size: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut dir = Direction::iter()
        .find(|dir| {
            let next_pos = get_next_pos_from_direction(start, *dir, grid_size);
            if let Some(pos) = next_pos {
                map[pos.0][pos.1].next_direction(*dir).is_some()
            } else {
                false
            }
        })
        .unwrap();
    let mut pos = start;
    let mut path = Vec::new();
    loop {
        path.push(pos);
        let next_pos = get_next_pos_from_direction(pos, dir, grid_size).unwrap();
        let next_dir = map[next_pos.0][next_pos.1].next_direction(dir).unwrap();
        pos = next_pos;
        dir = next_dir;
        if pos == start {
            break;
        }
    }
    path
}

pub fn part1(input: &str) -> u64 {
    let map = parse_input(input);
    let grid_size = (map.len(), map[0].len());
    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, tile)| **tile == Tile::Start)
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    let loop_path = get_loop_path(&map, start, grid_size);
    let loop_size = loop_path.len() as u64;
    loop_size.div_ceil(2)
}

pub fn part2(input: &str) -> u64 {
    let map = parse_input(input);
    let grid_size = (map.len(), map[0].len());
    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, tile)| **tile == Tile::Start)
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    let loop_path = get_loop_path(&map, start, grid_size);
    let loop_group: HashSet<(usize, usize)> = HashSet::from_iter(loop_path.into_iter());
    let mut is_capturing = false;
    let mut is_first_line_item_capturing = is_capturing;
    let mut captured = HashSet::new();
    for (i, j) in iproduct!((0..grid_size.0), (0..grid_size.1)) {
        if i == 0 {
            is_capturing = is_first_line_item_capturing;
        }
        if loop_group.contains(&(i, j)) {
            is_capturing = !is_capturing;
            if i == 0 {
                is_first_line_item_capturing = is_capturing;
            }
            continue;
        }
        if map[i][j] == Tile::Ground && is_capturing {
            captured.insert((i, j));
        }
    }
    captured.len() as u64
}

fn main() {
    let lines = crate::utils::get_day_input!();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = crate::utils::sample_input! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
        "};
        let expected = 4;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part1_with_bounds() {
        let input = crate::utils::sample_input! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
        "};
        let expected = 8;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        "};
        let expected = 4;
        assert_eq!(part2(&input), expected);
    }

    extern crate test;
    use test::test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines = crate::utils::get_day_input!();
        b.iter(|| part1(&lines));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines = crate::utils::get_day_input!();
        b.iter(|| part2(&lines));
    }
}
