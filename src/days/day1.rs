use std::collections::{BinaryHeap, HashMap};

pub fn solve() -> (String, String) {
    (part1(), part2())
}

fn part1() -> String {
    let input = include_str!("../../input/day1.txt");

    // Use heaps to order numbers from largest to smallest.
    // Need two heaps to store each column separately.
    let mut left_column = BinaryHeap::new();
    let mut right_column = BinaryHeap::new();

    for line in input.lines() {
        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Line does not contain integers."))
            .collect();

        // Note: A max-heap and a min-heap will produce the same total distance.
        // The difference comes from where we start (from the smallest or largest number.)
        // This means we don't need to reverse the values we push to solve the puzzle.
        if numbers.len() == 2 {
            left_column.push(numbers[0]);
            right_column.push(numbers[1]);
        }
    }

    // Get total distance by popping each pair (getting a pair)
    let mut total_distance = 0;

    while !left_column.is_empty() {
        let a = left_column.pop().unwrap();
        let b = right_column.pop().unwrap();
        total_distance += (a - b).abs();
    }

    return total_distance.to_string();
}

// Multiply the numbers in the left column with
// the amount of times they appear in the right column.
// Then sum up all the products to get the score.
fn part2() -> String {
    let input = include_str!("../../input/day1.txt");

    let size = input.lines().count();

    // Get the numbers and the number of time they appear.
    let mut left_column = Vec::with_capacity(size);
    let mut right_column = HashMap::new();

    for line in input.lines() {
        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Line does not contain integers."))
            .collect();

        if numbers.len() == 2 {
            left_column.push(numbers[0]);

            right_column
                .entry(numbers[1])
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    // Tally up the score by multiplying numbers in the left column
    // with the number of times they appear in the right column.
    let mut score = 0;

    for value in left_column {
        if let Some(multiplier) = right_column.get(&value) {
            score += value * multiplier;
        }
    }

    return score.to_string();
}
