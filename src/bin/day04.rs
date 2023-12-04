#![feature(test)]

use std::collections::HashSet;

use itertools::Itertools;

struct Card {
    id: u64,
    winning_numbers: HashSet<u64>,
    player_numbers: HashSet<u64>,
}

impl From<String> for Card {
    fn from(s: String) -> Self {
        let mut parts = s.split(": ");
        let id = parts
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let mut numbers = parts.next().unwrap().split(" | ");
        let winning_numbers = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let player_numbers = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        Self {
            id,
            winning_numbers,
            player_numbers,
        }
    }
}

fn part1(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|l| Card::from(l.to_string()))
        .map(|c| {
            let matches = c.player_numbers.intersection(&c.winning_numbers).count();
            if matches == 0 {
                return 0;
            }
            2u64.pow((matches - 1) as u32)
        })
        .sum()
}

fn part2(lines: &[String]) -> u64 {
    let cards = lines
        .iter()
        .map(|l| Card::from(l.to_string()))
        .collect_vec();
    let mut card_count = vec![1u64; cards.len()];
    for (i, c) in cards.iter().enumerate() {
        let matches = c.player_numbers.intersection(&c.winning_numbers).count();
        for j in 0..matches {
            card_count[i + j + 1] += card_count[i];
        }
    }
    card_count.iter().sum()
}

fn main() {
    let lines = utils::get_day_input!();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = utils::sample_input! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let expected = 13;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = utils::sample_input! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let expected = 30;
        assert_eq!(part2(&input), expected);
    }

    extern crate test;
    use test::test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines = utils::get_day_input!();
        b.iter(|| part1(&lines));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines = utils::get_day_input!();
        b.iter(|| part2(&lines));
    }
}
