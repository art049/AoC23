#![feature(test)]

use std::f64::EPSILON;

use itertools::Itertools;

struct Race {
    duration: u64,
    record_distance: u64,
}

fn parse_input(lines: &[String]) -> Vec<Race> {
    let times = lines[0].split_whitespace().skip(1);
    let distances = lines[1].split_whitespace().skip(1);
    times
        .zip(distances)
        .map(|(t, d)| Race {
            duration: t.parse().unwrap(),
            record_distance: d.parse().unwrap(),
        })
        .collect()
}

fn get_improvement_possibilities(race: &Race) -> u64 {
    // We solve the inequation: ax^2 + bx + c > 0
    // With a = -1, b = race.duration and c = -(race.record_distance + 1)
    // We add 1 to c to make sure we beat the record between the roots
    let a = -1.0;
    let b = race.duration as f64;
    let c = -((race.record_distance + 1) as f64);
    let delta = b * b - 4.0 * a * c;
    let start_f = (-b + delta.sqrt()) / (2.0 * a);
    let end_f = (-b - delta.sqrt()) / (2.0 * a);
    let start = start_f.ceil() as u64;
    let end = end_f.floor() as u64;
    end - start + 1
}

fn part1(lines: &[String]) -> u64 {
    let races = parse_input(lines);
    races.iter().map(get_improvement_possibilities).product()
}

fn parse_input_part_2(lines: &[String]) -> Race {
    let time: u64 = lines[0]
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();
    let distance: u64 = lines[1]
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()
        .unwrap();
    Race {
        duration: time,
        record_distance: distance,
    }
}

fn part2(lines: &[String]) -> u64 {
    let race = parse_input_part_2(lines);
    get_improvement_possibilities(&race)
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
        Time:      7  15   30
        Distance:  9  40  200
        "};
        let expected = 288;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = utils::sample_input! {"
        Time:      7  15   30
        Distance:  9  40  200
        "};
        let expected = 71503;
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
