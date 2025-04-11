use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
struct Grid {
    width: i64,
    height: i64,
    antennas: HashMap<char, Vec<Point>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Grid {
    fn new(input: &str) -> Self {
        let width: i64 = input.lines().next().unwrap().len().try_into().unwrap();
        let height: i64 = input.lines().count().try_into().unwrap();

        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

        for (column, line) in input.lines().enumerate() {
            for (row, c) in line.chars().enumerate() {
                // Skip empty positions & test characters
                if c == '.' || c == '#' {
                    continue;
                }

                let x = row.try_into().unwrap();
                let y = column.try_into().unwrap();
                let point = Point::new(x, y);

                antennas.entry(c).or_default().push(point);
            }
        }

        Self {
            width,
            height,
            antennas,
        }
    }

    fn check_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day8.txt").trim();

    let grid = Grid::new(&input);
    let part1 = part1(&grid);
    let part2 = part2(&grid);

    (part1, part2)
}

fn part1(grid: &Grid) -> String {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for positions in grid.antennas.values() {
        let length = positions.len();

        for i in 0..length {
            let pos1 = positions[i];

            for j in i + 1..length {
                let pos2 = positions[j];
                let distance = pos2 - pos1;

                let antinode_one = pos1 - distance;
                let antinode_two = pos2 + distance;

                if grid.check_bounds(antinode_one) {
                    antinodes.insert(antinode_one);
                }

                if grid.check_bounds(antinode_two) {
                    antinodes.insert(antinode_two);
                }
            }
        }
    }

    antinodes.len().to_string()
}

fn part2(grid: &Grid) -> String {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for positions in grid.antennas.values() {
        let length = positions.len();

        for i in 0..length {
            for j in i + 1..length {
                let mut pos1 = positions[i];
                let mut pos2 = positions[j];
                let distance = pos2 - pos1;

                while grid.check_bounds(pos1) {
                    antinodes.insert(pos1);
                    pos1 = pos1 - distance;
                }

                while grid.check_bounds(pos2) {
                    antinodes.insert(pos2);
                    pos2 = pos2 + distance;
                }
            }
        }
    }

    antinodes.len().to_string()
}
