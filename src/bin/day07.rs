#![feature(test)]

use int_enum::IntEnum;
use itertools::Itertools;
use strum::EnumString;

#[derive(Debug, Copy, Clone, EnumString, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    #[strum(serialize = "9")]
    _9 = 9,
    #[strum(serialize = "8")]
    _8 = 8,
    #[strum(serialize = "7")]
    _7 = 7,
    #[strum(serialize = "6")]
    _6 = 6,
    #[strum(serialize = "5")]
    _5 = 5,
    #[strum(serialize = "4")]
    _4 = 4,
    #[strum(serialize = "3")]
    _3 = 3,
    #[strum(serialize = "2")]
    _2 = 2,
    WeakJ = 1,
}

type Hand = [Card; 5];

#[derive(Debug, Copy, Clone, IntEnum, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Combination {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct HandWithCombination {
    hand: Hand,
    combination: Combination,
}

impl From<Hand> for HandWithCombination {
    fn from(hand: Hand) -> Self {
        let mut counts = [0; 15];
        for card in hand.iter() {
            counts[*card as usize] += 1;
        }
        let mut counts = counts.iter().enumerate().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(a.1));
        let combination = match counts[0].1 {
            5 => Combination::FiveOfAKind,
            4 => Combination::FourOfAKind,
            3 => {
                if counts[1].1 == &2 {
                    Combination::FullHouse
                } else {
                    Combination::ThreeOfAKind
                }
            }
            2 => {
                if counts[1].1 == &2 {
                    Combination::TwoPairs
                } else {
                    Combination::OnePair
                }
            }
            _ => Combination::HighCard,
        };
        Self { hand, combination }
    }
}

impl PartialOrd for HandWithCombination {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithCombination {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.combination.cmp(&other.combination) {
            std::cmp::Ordering::Equal => self.hand.cmp(&other.hand),
            other => other,
        }
    }
}

fn parse_input(lines: &[String]) -> Vec<(Hand, u64)> {
    lines
        .iter()
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(' ').unwrap();
            let mut hand = [Card::A; 5];
            for (i, card) in hand_str.char_indices() {
                hand[i] = card.to_string().parse().unwrap();
            }
            (hand, bid_str.parse().unwrap())
        })
        .collect()
}

fn part1(lines: &[String]) -> u64 {
    let mut hands_with_bid = parse_input(lines)
        .into_iter()
        .map(|(hand, bid)| (HandWithCombination::from(hand), bid))
        .collect_vec();
    hands_with_bid.sort_by_key(|(hand, _)| *hand);
    hands_with_bid
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid)
        .sum()
}

impl HandWithCombination {
    fn from_hand_with_jokers(hand: Hand) -> Self {
        let mut counts = [0; 15];
        let mut jokers = 0;
        for card in hand.iter() {
            if *card == Card::WeakJ {
                jokers += 1;
            } else {
                counts[*card as usize] += 1;
            }
        }
        let mut counts = counts.iter().enumerate().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(a.1));
        let combination = match counts[0].1 + jokers {
            5 => Combination::FiveOfAKind,
            4 => Combination::FourOfAKind,
            3 => {
                if counts[1].1 == &2 {
                    Combination::FullHouse
                } else {
                    Combination::ThreeOfAKind
                }
            }
            2 => {
                if counts[1].1 == &2 {
                    Combination::TwoPairs
                } else {
                    Combination::OnePair
                }
            }
            _ => Combination::HighCard,
        };
        Self { hand, combination }
    }
}

fn part2(lines: &[String]) -> u64 {
    let mut hands_with_bid = parse_input(lines)
        .into_iter()
        .map(|(hand, bid)| {
            (
                HandWithCombination::from_hand_with_jokers(hand.map(|c| {
                    if c == Card::J {
                        Card::WeakJ
                    } else {
                        c
                    }
                })),
                bid,
            )
        })
        .collect_vec();
    hands_with_bid.sort_by_key(|(hand, _)| *hand);
    hands_with_bid
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid)
        .sum()
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
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        "};
        let expected = 6440;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = utils::sample_input! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        "};
        let expected = 5905;
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
