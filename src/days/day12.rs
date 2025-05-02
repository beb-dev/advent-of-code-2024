use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Garden {
    plots: HashSet<Plot>,
    fences: HashSet<Fence>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Plot {
    row: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Fence {
    row: usize,
    column: usize,
    direction: Direction,
}

impl Garden {
    pub fn new() -> Self {
        Self {
            plots: HashSet::new(),
            fences: HashSet::new(),
        }
    }

    pub fn area(&self) -> usize {
        self.plots.len()
    }

    pub fn perimeter(&self) -> usize {
        self.fences.len()
    }

    pub fn sides(&self) -> usize {
        let mut sides = 0;
        let mut fences = self.fences.clone();

        while fences.len() > 0 {
            let fence_origin = fences.iter().next().unwrap().clone();
            let mut fence = fence_origin;

            // Clear fence in one direction
            while fences.remove(&fence) {
                match fence.direction {
                    Direction::Up | Direction::Down => fence.column = fence.column.wrapping_sub(1),
                    Direction::Left | Direction::Right => fence.row = fence.row.wrapping_sub(1),
                }
            }

            // Clear fence in other direction
            fence = fence_origin;
            fences.insert(fence_origin);

            while fences.remove(&fence) {
                match fence.direction {
                    Direction::Up | Direction::Down => fence.column += 1,
                    Direction::Left | Direction::Right => fence.row += 1,
                }
            }

            sides += 1;
        }

        sides
    }
}

impl Plot {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

impl Fence {
    pub fn new(row: usize, column: usize, direction: Direction) -> Self {
        Self {
            row,
            column,
            direction,
        }
    }
}

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day12.txt").trim();

    let mut part1 = 0;
    let mut part2 = 0;
    let gardens = get_gardens(&input);

    for garden in gardens {
        part1 += garden.area() * garden.perimeter();
        part2 += garden.area() * garden.sides();
    }

    (part1.to_string(), part2.to_string())
}

fn get_gardens(input: &str) -> Vec<Garden> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut gardens: Vec<Garden> = Vec::new();
    let mut plots_visited: HashSet<Plot> = HashSet::new();
    let mut plots_to_visite: VecDeque<Plot> = VecDeque::new();

    for row in 0..height {
        for column in 0..width {
            let plot = Plot { row, column };

            if plots_visited.contains(&plot) {
                continue;
            }

            // Create new garden.
            let mut garden = Garden::new();
            let region = grid[row][column];

            // Find all plots and fences for this garden with a breadth first search.
            plots_to_visite.push_back(plot);

            while let Some(plot) = plots_to_visite.pop_front() {
                if plots_visited.contains(&plot) {
                    continue;
                }

                garden.plots.insert(plot);
                plots_visited.insert(plot);

                // Up
                if plot.row > 0 && grid[plot.row - 1][plot.column] == region {
                    let plot_up = Plot::new(plot.row - 1, plot.column);
                    plots_to_visite.push_back(plot_up);
                } else {
                    let fence = Fence::new(plot.row, plot.column, Direction::Up);
                    garden.fences.insert(fence);
                }

                // Down
                if plot.row + 1 < height && grid[plot.row + 1][plot.column] == region {
                    let plot_down = Plot::new(plot.row + 1, plot.column);
                    plots_to_visite.push_back(plot_down);
                } else {
                    let fence = Fence::new(plot.row, plot.column, Direction::Down);
                    garden.fences.insert(fence);
                }

                // Left
                if plot.column > 0 && grid[plot.row][plot.column - 1] == region {
                    let plot_left = Plot::new(plot.row, plot.column - 1);
                    plots_to_visite.push_back(plot_left);
                } else {
                    let fence = Fence::new(plot.row, plot.column, Direction::Left);
                    garden.fences.insert(fence);
                }

                // Right
                if plot.column + 1 < width && grid[plot.row][plot.column + 1] == region {
                    let plot_right = Plot::new(plot.row, plot.column + 1);
                    plots_to_visite.push_back(plot_right);
                } else {
                    let fence = Fence::new(plot.row, plot.column, Direction::Right);
                    garden.fences.insert(fence);
                }
            }

            gardens.push(garden);
        }
    }

    gardens
}
