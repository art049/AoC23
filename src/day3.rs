use std::{collections::HashSet, iter::once};

use itertools::Itertools;

#[derive(Debug)]
struct Symbol {
    value: char,
    line: usize,
    col: usize,
}

#[derive(Debug)]
struct Number {
    value: u64,
    line: usize,
    col_start: usize,
    col_end: usize,
}

fn parse_input(input: &str) -> (Vec<Symbol>, Vec<Number>) {
    let lines = input.lines().collect_vec();
    // Ordered by (line, col)
    let mut numbers = vec![];
    let mut symbols = vec![];
    lines.iter().enumerate().for_each(|(i, l)| {
        let mut chars = l.chars().enumerate().peekable();
        while let Some((col, c)) = chars.next() {
            if c.is_ascii_digit() {
                let col_start = col;
                let other_digits = chars
                    .peeking_take_while(|(_, c)| c.is_ascii_digit())
                    .map(|(_, c)| c);
                let digits = once(c).chain(other_digits).collect::<String>();
                let value = digits.parse::<u64>().unwrap();
                let col_end = col_start + digits.len() - 1;
                numbers.push(Number {
                    value,
                    line: i,
                    col_start,
                    col_end,
                });
            } else if c != '.' {
                symbols.push(Symbol {
                    value: c,
                    line: i,
                    col,
                });
            }
        }
    });
    (symbols, numbers)
}

fn is_symbol_close_to_number(symbol: &Symbol, number: &Number) -> bool {
    let are_lines_close = (symbol.line as i64 - number.line as i64).abs() <= 1;
    let col_dist = if symbol.col < number.col_start {
        number.col_start - symbol.col
    } else if symbol.col > number.col_end {
        symbol.col - number.col_end
    } else {
        0
    };
    are_lines_close && col_dist <= 1
}

// Complexity: O(n^2)
pub fn part1(input: &str) -> u64 {
    let (symbols, numbers) = parse_input(input);
    let mut part_number_idxs = HashSet::new();
    for symbol in symbols.iter() {
        let new_parts_idx = numbers
            .iter()
            .enumerate()
            .filter(|(_, n)| is_symbol_close_to_number(symbol, n))
            .map(|(i, _)| i);
        part_number_idxs.extend(new_parts_idx);
    }
    part_number_idxs.iter().map(|i| numbers[*i].value).sum()
}

// Complexity: O(n^2)
pub fn part2(input: &str) -> u64 {
    let (symbols, numbers) = parse_input(input);
    let gears = symbols.iter().filter(|s| s.value == '*');
    let mut gear_ratio_sum = 0;
    for gear in gears {
        let numbers_close_to_symbol = numbers
            .iter()
            .enumerate()
            .filter(|(_, n)| is_symbol_close_to_number(gear, n))
            .map(|(_, n)| n.value)
            .collect_vec();
        if numbers_close_to_symbol.len() == 2 {
            gear_ratio_sum += numbers_close_to_symbol[0] * numbers_close_to_symbol[1];
        }
    }
    gear_ratio_sum
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
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "};
        let expected = 4361;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "};
        let expected = 467835;
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
