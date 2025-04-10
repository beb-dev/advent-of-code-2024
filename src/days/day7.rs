pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day7.txt");

    let mut part1 = 0;
    let mut part2 = 0;

    let equations: Vec<(usize, Vec<usize>)> = input
        .trim()
        .lines()
        .map(|line| {
            let (result_str, values_str) = line.split_once(':').unwrap();

            let result = result_str.parse().unwrap();
            let values = values_str
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();

            (result, values)
        })
        .collect();

    for (result, values) in equations {
        if can_be_equal_part1(result, &values, 0, 0) {
            part1 += result;
        }

        if can_be_equal_part2(result, &values, 0, 0) {
            part2 += result;
        }
    }

    (part1.to_string(), part2.to_string())
}

fn can_be_equal_part1(result: usize, values: &Vec<usize>, current: usize, depth: usize) -> bool {
    if current > result {
        return false;
    }

    if depth >= values.len() {
        return current == result;
    }

    can_be_equal_part1(result, values, current + values[depth], depth + 1)
        || can_be_equal_part1(result, values, current * values[depth], depth + 1)
}

fn can_be_equal_part2(result: usize, values: &Vec<usize>, current: usize, depth: usize) -> bool {
    if current > result {
        return false;
    }

    if depth >= values.len() {
        return current == result;
    }

    if can_be_equal_part2(result, values, current + values[depth], depth + 1)
        || can_be_equal_part2(result, values, current * values[depth], depth + 1)
    {
        return true;
    }

    if let Some(current) = concatenate(current, values[depth]) {
        return can_be_equal_part2(result, values, current, depth + 1);
    } else {
        return false;
    }
}

fn concatenate(a: usize, b: usize) -> Option<usize> {
    let mut offset = 1;

    while offset <= b {
        offset = offset.checked_mul(10)?;
    }

    Some(a.checked_mul(offset)?.checked_add(b)?)
}
