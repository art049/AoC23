fn part1(lines: &Vec<String>) -> u64 {
    0
}

fn part2(lines: &Vec<String>) -> u64 {
    0
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

        "};
        let expected = 0;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = utils::sample_input! {"

        "};
        let expected = 0;
        assert_eq!(part2(&input), expected);
    }
}
