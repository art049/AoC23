#[macro_export]
macro_rules! get_day_input {
    () => {{
        let filepath = file!();
        let day: u32 = filepath
            .strip_prefix("src/days/day")
            .and_then(|s| s.strip_suffix(".rs"))
            .and_then(|s| s.parse().ok())
            .expect("unable to parse the day");
        let path = format!("../inputs/day{:02}.txt", day);
        let data = std::fs::read_to_string(path).expect("Could not open file");
        data
    }};
}

pub use get_day_input;

#[macro_export]
macro_rules! sample_input {
    ($expression:expr) => {{
        let raw_input = indoc::indoc!($expression);
        raw_input
    }};
}

#[cfg(test)]
pub use sample_input;
