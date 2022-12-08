use aoc_2022::{
    day01::{day1a, day1b},
    day02::{day2a, day2b},
    day03::{day3a, day3b},
    day04::{day4a, day4b},
    day05::{day5a, day5b},
    day06::{day6a, day6b},
    day07::{day7a, day7b},
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
        "day5a" => day5a(),
        "day5b" => day5b(),
        "day6a" => day6a(),
        "day6b" => day6b(),
        "day7a" => day7a(),
        "day7b" => day7b(),
        _ => day7b(),
        // _ => "Not yet solved".to_string(),
    };
    println!("{result}");
}
