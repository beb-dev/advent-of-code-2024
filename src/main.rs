use days::day1;
use std::fmt::Error;

mod days;

fn main() {
    // TODO: Add a way to choose which day to run with command-line arguments.
    if let Ok((part1, part2)) = get_day(1) {
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}

fn get_day(day: u8) -> Result<(String, String), Error> {
    match day {
        1 => Ok(day1::solve()),
        _ => panic!("Invalid day. Please provide a day between 1 and 25."),
    }
}
