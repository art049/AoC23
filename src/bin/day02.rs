#![feature(test)]

use itertools::{max, Itertools};
use parse_display::{Display, FromStr};

struct Game {
    id: u64,
    sets: Vec<GameCubeSet>,
}

#[derive(Debug)]
struct GameCubeSet {
    blue: u64,
    red: u64,
    green: u64,
}

impl FromIterator<GameCubes> for GameCubeSet {
    fn from_iter<T: IntoIterator<Item = GameCubes>>(iter: T) -> Self {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for cube in iter {
            match cube {
                GameCubes::Blue(n) => blue += n,
                GameCubes::Red(n) => red += n,
                GameCubes::Green(n) => green += n,
            }
        }
        Self { blue, red, green }
    }
}

#[derive(Display, FromStr, Debug)]
enum GameCubes {
    #[display("{0} blue")]
    Blue(u64),
    #[display("{0} red")]
    Red(u64),
    #[display("{0} green")]
    Green(u64),
}

fn parse_games(lines: &[String]) -> impl Iterator<Item = Game> + '_ {
    lines.iter().map(|line| {
        let (game_str, sets_str) = line.split_once(": ").unwrap();
        let id: u64 = game_str.strip_prefix("Game ").unwrap().parse().unwrap();
        let sets = sets_str
            .split("; ")
            .map(|set| {
                let cubes: Vec<GameCubes> =
                    set.split(", ").map(|cube| cube.parse().unwrap()).collect();
                cubes.into_iter().collect::<GameCubeSet>()
            })
            .collect_vec();
        Game { id, sets }
    })
}

fn part1(lines: &[String]) -> u64 {
    let games = parse_games(lines);
    games
        .filter(|game| {
            game.sets.iter().all(|set| {
                const MAX_RED: u64 = 12;
                const MAX_GREEN: u64 = 13;
                const MAX_BLUE: u64 = 14;
                set.blue <= MAX_BLUE && set.red <= MAX_RED && set.green <= MAX_GREEN
            })
        })
        .fold(0, |acc, game| acc + game.id)
}

fn part2(lines: &[String]) -> u64 {
    let games = parse_games(lines);
    let larger_cubesets = games.map(|game| GameCubeSet {
        blue: max(game.sets.iter().map(|set| set.blue)).unwrap_or(0),
        red: max(game.sets.iter().map(|set| set.red)).unwrap_or(0),
        green: max(game.sets.iter().map(|set| set.green)).unwrap_or(0),
    });
    let powers = larger_cubesets.map(|set| set.blue * set.red * set.green);
    powers.sum()
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
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let expected = 8;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = utils::sample_input! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let expected = 2286;
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
