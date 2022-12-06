use itertools::Itertools;

pub fn day6a() -> String {
    let stream = read_data();
    const WINDOW_SIZE: usize = 4;
    if let Some((pos, _)) = stream
        .as_bytes()
        .windows(WINDOW_SIZE)
        .find_position(|w| w.iter().all_unique())
    {
        return (pos + WINDOW_SIZE).to_string();
    }
    "No marker was found".to_string()
}

pub fn day6b() -> String {
    let stream = read_data();
    const WINDOW_SIZE: usize = 14;
    if let Some((pos, _)) = stream
        .as_bytes()
        .windows(WINDOW_SIZE)
        .find_position(|w| w.iter().all_unique())
    {
        return (pos + WINDOW_SIZE).to_string();
    }
    "No marker was found".to_string()
}

fn read_data() -> String {
    std::fs::read_to_string("inputs/day06.txt").expect("could not read file")
}
