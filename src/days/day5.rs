use std::cmp::Ordering;
use std::collections::HashSet;

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day5.txt");

    // Parse input
    let (orders_input, updates_input) = input.trim().split_once("\r\n\r\n").unwrap();

    let orders: HashSet<(u32, u32)> = orders_input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect();

    let updates: Vec<Vec<u32>> = updates_input
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    // Check if numbers are ordered in each update.
    // Get the sums of the middle point of each update that is ordered for part 1.
    // Fix updates with bad orders, then do the sums of the middle point for fixed orders for part 2.
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for mut update in updates {
        if update.is_sorted_by(|a, b| compare(a, b, &orders) != Ordering::Greater) {
            part1 += update[update.len() / 2];
        } else {
            update.sort_by(|a, b| compare(a, b, &orders));
            part2 += update[update.len() / 2];
        }
    }

    (part1.to_string(), part2.to_string())
}

fn compare(a: &u32, b: &u32, orders: &HashSet<(u32, u32)>) -> Ordering {
    if orders.contains(&(*a, *b)) {
        Ordering::Less
    } else if orders.contains(&(*b, *a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
