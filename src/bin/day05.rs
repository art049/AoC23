#![feature(test)]

use itertools::Itertools;

#[derive(Debug)]
struct Map {
    sources: Vec<u64>,
    destinations: Vec<u64>,
    lengths: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    /// Returns the last index of the range (inclusive)
    fn last(&self) -> u64 {
        self.start + self.length
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        let start = self.start.max(other.start);
        let end = self.last().min(other.last());
        if start > end {
            return None;
        }
        Some(Range {
            start,
            length: end - start,
        })
    }
}

impl Map {
    fn get_destination(&self, input: u64) -> u64 {
        for i in 0..self.sources.len() {
            let source = self.sources[i];
            let length = self.lengths[i];
            if input >= source && input < source + length as u64 {
                return self.destinations[i] + input - source;
            }
        }
        input
    }

    fn get_destination_ranges(&self, range: &Range) -> Vec<Range> {
        let mut input_ranges = Vec::new();
        let mut ranges = Vec::new();

        // Compute possible intersections
        for i in 0..self.sources.len() {
            if let Some(intersection) = range.intersection(&Range {
                start: self.sources[i],
                length: self.lengths[i] as u64,
            }) {
                let length = intersection.last() - intersection.start;
                ranges.push(Range {
                    start: self.destinations[i] + intersection.start - self.sources[i],
                    length,
                });
                input_ranges.push(intersection);
            }
        }

        // If there are no intersections, return the unchanged range
        if input_ranges.is_empty() {
            return vec![range.clone()];
        }

        input_ranges.sort_by_key(|r| r.start);

        // Add the ranges before the first intersection
        if input_ranges[0].start > range.start {
            ranges.insert(
                0,
                Range {
                    start: range.start,
                    length: input_ranges[0].start - range.start,
                },
            );
        }
        // Fill the gaps between intersections
        for i in 0..input_ranges.len() - 1 {
            if input_ranges[i].last() < input_ranges[i + 1].start {
                ranges.push(Range {
                    start: input_ranges[i].last(),
                    length: input_ranges[i + 1].start - input_ranges[i].last(),
                });
            }
        }
        // Add the ranges after the last intersection
        if input_ranges[input_ranges.len() - 1].last() < range.last() {
            ranges.push(Range {
                start: input_ranges[input_ranges.len() - 1].last(),
                length: range.last() - input_ranges[input_ranges.len() - 1].last(),
            });
        }
        ranges
    }
}

fn parse_input(lines: &[String]) -> (Vec<u64>, Vec<Map>) {
    let mut lines_iter = lines.iter().peekable();
    let seeds = lines_iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut maps = Vec::new();
    loop {
        lines_iter.next();
        let map_lines = lines_iter
            .take_while_ref(|l| !l.is_empty())
            .skip(1)
            .collect_vec();
        if map_lines.is_empty() {
            break;
        }
        let mut sources = Vec::new();
        let mut destinations = Vec::new();
        let mut lengths = Vec::new();
        for line in map_lines {
            let (destination, source, length) = line.split_whitespace().collect_tuple().unwrap();
            sources.push(source.parse::<u64>().unwrap());
            destinations.push(destination.parse::<u64>().unwrap());
            lengths.push(length.parse::<usize>().unwrap());
        }
        maps.push(Map {
            sources,
            destinations,
            lengths,
        });
    }
    (seeds, maps)
}

fn part1(lines: &[String]) -> u64 {
    let (seeds, maps) = parse_input(lines);
    let locations = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.get_destination(acc)));
    locations.min().unwrap()
}

fn part2(lines: &[String]) -> u64 {
    let (seeds, maps) = parse_input(lines);
    let mut ranges = seeds
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            length: chunk[1],
        })
        .collect_vec();

    for map in maps.iter() {
        let mut new_ranges = Vec::new();
        for range in ranges.iter() {
            new_ranges.extend(map.get_destination_ranges(range));
        }
        ranges = new_ranges;
    }
    ranges.iter().map(|r| r.start).min().unwrap()
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
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        "};
        let expected = 35;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = utils::sample_input! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        "};
        let expected = 46;
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
