#![feature(test)]
use int_enum::IntEnum;
use itertools::Itertools;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

fn part1(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|l| {
            let digits_chars = l.chars().filter(|c| c.is_ascii_digit());
            let first = digits_chars.clone().next().unwrap();
            let last = digits_chars.last().unwrap();
            let number: String = [first, last].iter().collect();
            number.parse::<u32>().unwrap()
        })
        .sum()
}

#[derive(Debug, EnumString, EnumIter, IntEnum, Display, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
#[repr(u32)]
enum Digit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

fn part2(lines: &[String]) -> u32 {
    let lines = lines
        .iter()
        .map(|line| {
            let mut line = line.clone();
            let mut first_str_digit = (line.len(), None);
            let mut last_str_digit = (0, None);
            for digit in Digit::iter() {
                if let Some(pos) = line.find(digit.to_string().as_str()) {
                    if pos < first_str_digit.0 {
                        first_str_digit = (pos, Some(digit))
                    }
                }
                if let Some(pos) = line.rfind(digit.to_string().as_str()) {
                    if pos > last_str_digit.0 {
                        last_str_digit = (pos, Some(digit))
                    }
                }
            }
            if let (pos, Some(digit)) = first_str_digit {
                line.insert_str(pos, digit.int_value().to_string().as_str());
            }
            if let (pos, Some(digit)) = last_str_digit {
                line.insert_str(pos + 1, digit.int_value().to_string().as_str());
            }
            line
        })
        .collect_vec();
    part1(&lines)
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
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        let expected = 142;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = utils::sample_input! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        let expected = 281;
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
