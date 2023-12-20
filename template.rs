pub fn part1(input: &str) -> u64 {
    0
}

pub fn part2(input: &str) -> u64 {
    0
}

fn main() {
    let input = crate::utils::get_day_input!();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = crate::utils::sample_input! {"

        "};
        let expected = 0;
        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = crate::utils::sample_input! {"

        "};
        let expected = 0;
        assert_eq!(part2(&input), expected);
    }

    extern crate test;
    use test::test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = crate::utils::get_day_input!();
        b.iter(|| part1(&input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = crate::utils::get_day_input!();
        b.iter(|| part2(&input));
    }
}
