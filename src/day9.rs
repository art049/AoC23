use std::iter;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let lines = input.lines().collect_vec();
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect::<Vec<_>>()
}

fn derive(input: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut iter = input.iter();
    let mut first = iter.next();
    while let (Some(a), Some(b)) = (first, iter.next()) {
        result.push(b - a);
        first = Some(b);
    }
    result
}

fn integrate(input: &[i32], c: i32) -> Vec<i32> {
    iter::once(c)
        .chain(input.iter().scan(c, |state, &x| {
            *state += x;
            Some(*state)
        }))
        .collect()
}

fn extrapolate(input: &Vec<i32>) -> i32 {
    let mut seq = input.clone();
    let mut first_stack = Vec::new();
    loop {
        let first = seq.first().unwrap();
        if seq.iter().all(|x| *x == 0) {
            break;
        }
        first_stack.push(*first);
        seq = derive(&seq);
    }
    seq.push(0);
    while let Some(c) = first_stack.pop() {
        seq = integrate(&seq, c);
    }
    *seq.last().unwrap()
}

pub fn part1(input: &str) -> i32 {
    let sequences = parse_input(input);
    sequences.iter().map(extrapolate).sum()
}

pub fn part2(input: &str) -> i32 {
    let mut sequences = parse_input(input);
    for sequence in sequences.iter_mut() {
        sequence.reverse();
    }
    sequences.iter().map(extrapolate).sum()
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
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        "};
        let expected = 114;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        "};
        let expected = 2;
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
