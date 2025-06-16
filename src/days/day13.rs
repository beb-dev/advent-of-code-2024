#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ClawMachine {
    a: Point,
    b: Point,
    prize: Point,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl ClawMachine {
    fn new(a: Point, b: Point, prize: Point) -> Self {
        Self { a, b, prize }
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day13.txt");
    let machines = parse_machines(&input);

    let mut part1 = 0;
    let mut part2 = 0;

    for machine in machines {
        if let Some(tokens) = get_lowest_cost_win(&machine, 0) {
            part1 += tokens;
        }

        if let Some(tokens) = get_lowest_cost_win(&machine, 10000000000000) {
            part2 += tokens;
        }
    }

    (part1.to_string(), part2.to_string())
}

fn get_lowest_cost_win(machine: &ClawMachine, offset: i64) -> Option<i64> {
    let a = machine.a;
    let b = machine.b;
    let p = Point::new(machine.prize.x + offset, machine.prize.y + offset);

    let det = (a.x * b.y) - (b.x * a.y);

    if det == 0 {
        return None;
    }

    let a_press = ((p.x * b.y) - (p.y * b.x)) / det;
    let b_press = ((p.y * a.x) - (p.x * a.y)) / det;

    let px_target = (a_press * a.x) + (b_press * b.x);
    let py_target = (a_press * a.y) + (b_press * b.y);

    if p.x == px_target && p.y == py_target {
        Some((a_press * 3) + b_press)
    } else {
        None
    }
}

fn parse_machines(input: &str) -> Vec<ClawMachine> {
    input
        .trim()
        .split("\r\n\r\n")
        .map(|s| {
            let mut lines = s.lines();

            let a_input = lines.next().unwrap();
            let b_input = lines.next().unwrap();
            let p_input = lines.next().unwrap();

            // Button A
            let a_split = a_input.find(',').unwrap();
            let ax: i64 = a_input[12..a_split].parse().unwrap();
            let ay: i64 = a_input[a_split + 3..].parse().unwrap();
            let a = Point::new(ax, ay);

            // Button B
            let b_split = a_input.find(',').unwrap();
            let bx: i64 = b_input[12..b_split].parse().unwrap();
            let by: i64 = b_input[b_split + 3..].parse().unwrap();
            let b = Point::new(bx, by);

            // Prize
            let p_split = p_input.find(',').unwrap();
            let px: i64 = p_input[9..p_split].parse().unwrap();
            let py: i64 = p_input[p_split + 4..].parse().unwrap();
            let p = Point::new(px, py);

            ClawMachine::new(a, b, p)
        })
        .collect()
}
