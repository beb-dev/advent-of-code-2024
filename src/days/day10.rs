use std::collections::HashSet;

struct Trail {
    score: usize,
    rating: usize,
    ends: HashSet<(i64, i64)>,
}

impl Trail {
    fn new() -> Self {
        Self {
            score: 0,
            rating: 0,
            ends: HashSet::new(),
        }
    }
}

struct Grid {
    width: i64,
    height: i64,
    values: Vec<i64>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len().try_into().unwrap();
        let height = input.lines().count().try_into().unwrap();
        let tiles = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        Self {
            width,
            height,
            values: tiles,
        }
    }

    pub fn get_value(&self, x: i64, y: i64) -> Option<i64> {
        if self.check_bounds(x, y) {
            let index = (self.width * y) + x;
            return Some(self.values[index as usize]);
        } else {
            return None;
        }
    }

    pub fn get_position(&self, index: i64) -> Option<(i64, i64)> {
        if index >= 0 && index < self.width * self.height {
            let x = index % self.width;
            let y = index / self.width;
            return Some((x, y));
        } else {
            return None;
        }
    }

    fn check_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }
}

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day10.txt").trim();

    let grid = Grid::new(input);

    let mut part1 = 0;
    let mut part2 = 0;

    for (index, value) in grid.values.iter().enumerate() {
        if *value == 0 {
            if let Some((x, y)) = grid.get_position(index.try_into().unwrap()) {
                let mut trail = Trail::new();
                walk_trail(&mut trail, &grid, x, y, 0);
                part1 += trail.score;
                part2 += trail.rating;
            }
        }
    }

    (part1.to_string(), part2.to_string())
}

fn walk_trail(trail: &mut Trail, grid: &Grid, x: i64, y: i64, depth: i64) {
    if grid.check_bounds(x, y) == false {
        return;
    }

    let value = grid.get_value(x, y).unwrap();

    if value == depth {
        if value == 9 {
            // Found end of trail
            trail.ends.insert((x, y));
            trail.score = trail.ends.len();
            trail.rating += 1;
        } else {
            // Explore trail in all 4 directions
            walk_trail(trail, grid, x + 1, y, depth + 1);
            walk_trail(trail, grid, x - 1, y, depth + 1);
            walk_trail(trail, grid, x, y + 1, depth + 1);
            walk_trail(trail, grid, x, y - 1, depth + 1);
        }
    }
}
