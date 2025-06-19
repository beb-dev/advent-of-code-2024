#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector2D {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Robot {
    pos: Vector2D,
    vel: Vector2D,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    width: i64,
    height: i64,
    robots: Vec<Robot>,
}

impl Vector2D {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Robot {
    fn new(pos: Vector2D, vel: Vector2D) -> Self {
        Self { pos, vel }
    }
}

impl Grid {
    fn new(width: i64, height: i64, robots: Vec<Robot>) -> Self {
        Self {
            width,
            height,
            robots,
        }
    }

    fn simulate(&mut self, seconds: usize) {
        for _ in 0..seconds {
            for robot in self.robots.iter_mut() {
                robot.pos.x = (robot.pos.x + robot.vel.x).rem_euclid(self.width);
                robot.pos.y = (robot.pos.y + robot.vel.y).rem_euclid(self.height);
            }
        }
    }

    fn get_safety_factor(&self) -> usize {
        let mut quadrant1 = 0;
        let mut quadrant2 = 0;
        let mut quadrant3 = 0;
        let mut quadrant4 = 0;

        let mid_x = self.width / 2;
        let mid_y = self.height / 2;

        for robot in self.robots.iter() {
            if robot.pos.x < mid_x && robot.pos.y < mid_y {
                quadrant1 += 1;
            }

            if robot.pos.x < mid_x && robot.pos.y > mid_y {
                quadrant2 += 1;
            }

            if robot.pos.x > mid_x && robot.pos.y < mid_y {
                quadrant3 += 1;
            }

            if robot.pos.x > mid_x && robot.pos.y > mid_y {
                quadrant4 += 1;
            }
        }

        quadrant1 * quadrant2 * quadrant3 * quadrant4
    }

    // Find the tree by looking for the lowest safety factor in the simulation.
    // This does work only because our input puts the tree in one quadrant.
    // If it was in the middle somewhat, this would not work.
    fn find_tree(&mut self) -> usize {
        let mut second = 0;
        let mut lowest_safety_factor = self.get_safety_factor();

        for i in 1..=6243 {
            self.simulate(1);
            let factor = self.get_safety_factor();

            if lowest_safety_factor > factor {
                lowest_safety_factor = factor;
                second = i;
            }
        }

        second
    }
}

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day14.txt");

    let mut grid_part1 = Grid::new(101, 103, get_robots(&input));
    let mut grid_part2 = Grid::new(101, 103, get_robots(&input));

    grid_part1.simulate(100);

    let part1 = grid_part1.get_safety_factor();
    let part2 = grid_part2.find_tree();

    (part1.to_string(), part2.to_string())
}

fn get_robots(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    let lines = input.trim().lines();

    for line in lines {
        let (pos, vel) = line.split_once(' ').unwrap();
        let (pos_x, pos_y) = pos.strip_prefix("p=").unwrap().split_once(",").unwrap();
        let (vel_x, vel_y) = vel.strip_prefix("v=").unwrap().split_once(",").unwrap();

        let position = Vector2D::new(pos_x.parse().unwrap(), pos_y.parse().unwrap());
        let velocity = Vector2D::new(vel_x.parse().unwrap(), vel_y.parse().unwrap());
        let robot = Robot::new(position, velocity);

        robots.push(robot);
    }

    robots
}
