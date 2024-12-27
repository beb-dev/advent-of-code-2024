use days::{day1, day2, day3, day4};
use std::fmt::Error;

mod days;

fn main() {
    // TODO: Add a way to choose which day to run with command-line arguments.
    if let Ok((part1, part2)) = get_day(4) {
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}

fn get_day(day: u8) -> Result<(String, String), Error> {
    match day {
        1 => Ok(day1::solve()),
        2 => Ok(day2::solve()),
        3 => Ok(day3::solve()),
        4 => Ok(day4::solve()),
        _ => panic!("Invalid day. Please provide a day between 1 and 25."),
    }
}
