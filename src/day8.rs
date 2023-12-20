use std::collections::HashMap;

use itertools::Itertools;
use num::integer;
use parse_display::FromStr;

type NodeId = [char; 3];

struct Node {
    id: NodeId,
    children: (NodeId, NodeId),
}

#[derive(Debug, FromStr)]
enum Direction {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<NodeId, Node>) {
    let mut lines = input.lines();
    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    lines.next();
    let nodes = lines
        .map(|line| {
            let mut chars = line.chars();
            let id = [
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ];
            let mut chars = chars.skip_while(|c| *c != '(');
            chars.next();
            let l_child = [
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ];
            let mut chars = chars.skip_while(|c| *c != ' ');
            chars.next();
            let r_child = [
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ];
            (
                id,
                Node {
                    id,
                    children: (l_child, r_child),
                },
            )
        })
        .collect::<HashMap<NodeId, Node>>();

    (directions, nodes)
}

pub fn part1(input: &str) -> u64 {
    let (directions, nodes) = parse_input(input);
    let mut current = ['A', 'A', 'A'];
    const END_NODE_ID: NodeId = ['Z', 'Z', 'Z'];
    for (i, direction) in directions.iter().cycle().enumerate() {
        let node = &nodes[&current];
        match direction {
            Direction::Left => current = node.children.0,
            Direction::Right => current = node.children.1,
        }
        if current == END_NODE_ID {
            return i as u64 + 1;
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> u64 {
    let (directions, nodes) = parse_input(input);
    let mut current_nodes = nodes.values().filter(|n| n.id[2] == 'A').collect_vec();
    let dist_to_z = current_nodes
        .iter()
        .map(|n| {
            let mut current = *n;
            for (i, direction) in directions.iter().cycle().enumerate() {
                match direction {
                    Direction::Left => current = &nodes[&current.children.0],
                    Direction::Right => current = &nodes[&current.children.1],
                }
                if current.id[2] == 'Z' {
                    return i as u64 + 1;
                }
            }
            unreachable!()
        })
        .collect_vec();
    let lcm = dist_to_z.iter().fold(1, |acc, x| integer::lcm(acc, *x));
    lcm
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
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        "};
        let expected = 6;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        "};
        let expected = 6;
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
