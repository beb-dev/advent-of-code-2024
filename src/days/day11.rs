use std::{collections::HashMap, mem};

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day11.txt").trim();

    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let part1 = split_stones(&stones, 25);
    let part2 = split_stones(&stones, 75);

    (part1.to_string(), part2.to_string())
}

fn split_stones(stones: &Vec<usize>, blink: usize) -> usize {
    let mut all_stones: HashMap<usize, usize> = stones.iter().map(|s| (*s, 1)).collect();
    let mut split_stones: HashMap<usize, usize> = HashMap::new();

    for _ in 0..blink {
        for (number, count) in all_stones.drain() {
            if number == 0 {
                *split_stones.entry(1).or_default() += count;
            } else if let Some((left, right)) = split_in_two(number) {
                *split_stones.entry(left).or_default() += count;
                *split_stones.entry(right).or_default() += count;
            } else {
                *split_stones.entry(number * 2024).or_default() += count;
            }
        }

        mem::swap(&mut all_stones, &mut split_stones);
    }

    all_stones.values().sum()
}

fn split_in_two(num: usize) -> Option<(usize, usize)> {
    let can_split = can_evenly_split(num);

    if can_split == None {
        return None;
    }

    let length = can_split.unwrap();
    let half_length = length >> 1;
    let base: usize = 10;

    let mut left = 0;
    let mut right = 0;
    let mut total = 0;

    for digit_index in (0..length).rev() {
        let power = base.checked_pow(digit_index as u32)?;
        total += power;

        for digit in 0..=9 {
            if total <= num {
                total += power;
            } else {
                total -= power;

                if digit_index >= half_length {
                    let pow = digit_index - half_length;
                    left += digit * base.checked_pow(pow as u32)?;
                } else {
                    right += digit * base.checked_pow(digit_index as u32)?;
                }

                break;
            }
        }
    }

    Some((left, right))
}

fn can_evenly_split(num: usize) -> Option<usize> {
    let mut power = 10;
    let mut length = 1;
    let mut is_even = false;

    while num >= power {
        length += 1;
        is_even = !is_even;

        if let Some(new_power) = power.checked_mul(10) {
            power = new_power;
        } else {
            return None;
        }
    }

    if length >= 2 && is_even {
        Some(length)
    } else {
        None
    }
}
