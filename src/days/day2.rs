#[derive(Debug)]
enum Order {
    Ascending,
    Descending,
}

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day2.txt");

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for line in input.lines() {
        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Line does not contain integers."))
            .collect();

        // Part 1
        if _validate(&numbers) {
            part1 += 1;
        }

        // Part 2
        let mut numbers_chunk = numbers.clone();

        for (i, &num) in numbers.iter().enumerate() {
            numbers_chunk.remove(i);

            if _validate(&numbers_chunk) {
                part2 += 1;
                break;
            } else {
                numbers_chunk.insert(i, num);
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

fn _validate(report: &Vec<i32>) -> bool {
    let order = get_order(&report[0], &report[1]);

    return report.is_sorted_by(|a, b| is_sorted(a, b, &order) && compare_levels(a, b));
}

fn compare_levels(a: &i32, b: &i32) -> bool {
    let distance = (a - b).abs();
    return distance > 0 && distance <= 3;
}

fn get_order(a: &i32, b: &i32) -> Option<Order> {
    if a < b {
        Some(Order::Ascending)
    } else if a > b {
        Some(Order::Descending)
    } else {
        None
    }
}

fn is_sorted(a: &i32, b: &i32, order: &Option<Order>) -> bool {
    match order {
        Some(Order::Ascending) => a < b,
        Some(Order::Descending) => a > b,
        _ => false,
    }
}
