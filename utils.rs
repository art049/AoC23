#[macro_export]
macro_rules! get_day_input {
    () => {{
        use std::io::BufRead;
        let filepath = file!();
        let day: u32 = filepath
            .strip_prefix("src/bin/day")
            .and_then(|s| s.strip_suffix(".rs"))
            .and_then(|s| s.parse().ok())
            .expect("unable to parse the day");
        let path = format!("inputs/day{:02}.txt", day);
        let file = std::fs::File::open(path).expect("Could not open file");
        let lines: Vec<String> = std::io::BufReader::new(file)
            .lines()
            .map_while(|line| line.ok())
            .collect();
        lines
    }};
}

#[macro_export]
macro_rules! sample_input {
    ($expression:expr) => {{
        let raw_input = indoc::indoc!($expression);
        let lines: Vec<String> = raw_input.lines().map(|l| l.to_string()).collect();
        lines
    }};
}
