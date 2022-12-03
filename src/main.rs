use aoc_2022::{
    day01::{day1a, day1b},
    day02::{day2a, day2b},
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    let result = match problem {
        "day1a" => day1a(),
        "day1b" => day1b(),
        "day2a" => day2a(),
        "day2b" => day2b(),
        _ => day2b(),
        // _ => "Not yet solved".to_string(),
    };
    println!("{result}");
}
