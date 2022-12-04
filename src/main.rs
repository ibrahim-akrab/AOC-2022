use aoc_2022::{
    day01::{day1a, day1b},
    day02::{day2a, day2b},
    day03::{day3a, day3b},
    day04::{day4a, day4b},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    let result = match problem {
        "day1a" => day1a(),
        "day1b" => day1b(),
        "day2a" => day2a(),
        "day2b" => day2b(),
        "day3a" => day3a(),
        "day3b" => day3b(),
        "day4a" => day4a(),
        "day4b" => day4b(),
        _ => day4b(),
        // _ => "Not yet solved".to_string(),
    };
    println!("{result}");
}
