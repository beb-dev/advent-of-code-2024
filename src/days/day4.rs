pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day4.txt");

    let grid = build_grid(&input);
    let part1 = part1(&grid);
    let part2 = part2();

    (part1, part2)
}

fn part1(grid: &Vec<Vec<char>>) -> String {
    let mut count = 0;
    let xmas = ['X', 'M', 'A', 'S'];
    let dirs = get_directions();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == xmas[0] {
                for d in dirs.iter() {
                    let (mut x2, mut y2) = (x as i32, y as i32);

                    for n in 1..xmas.len() {
                        (x2, y2) = (x2 + d.0, y2 + d.1);

                        if x2 < 0 || y2 < 0 || x2 >= grid[x].len() as i32 || y2 >= grid.len() as i32
                        {
                            break;
                        }

                        if grid[y2 as usize][x2 as usize] == xmas[n] {
                            if n + 1 == xmas.len() {
                                count += 1;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    count.to_string()
}

fn part2() -> String {
    let input = include_str!("../../input/day4.txt");

    let row_count = input.lines().count();
    let row_length = input.lines().next().unwrap().len();

    let chars: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();

    // Search for "MAS" shapes using a 1D array.
    // The middle point has to be 'A', so we can skip the edges.
    let mut count = 0;

    for y in 1..row_count - 1 {
        for x in 1..row_length - 1 {
            let i = (y * row_length) + x;

            if chars[i] == 'A' {
                // Get diagonals
                let top_left = chars[((y - 1) * row_length) + (x - 1)];
                let top_right = chars[((y - 1) * row_length) + (x + 1)];
                let bottom_left = chars[((y + 1) * row_length) + (x - 1)];
                let bottom_right = chars[((y + 1) * row_length) + (x + 1)];

                // Check if both diagonals can spell "MAS"
                let pair1 = (top_left, bottom_right);
                let pair2 = (top_right, bottom_left);

                match (pair1, pair2) {
                    (('M', 'S'), ('M', 'S')) => count += 1,
                    (('M', 'S'), ('S', 'M')) => count += 1,
                    (('S', 'M'), ('M', 'S')) => count += 1,
                    (('S', 'M'), ('S', 'M')) => count += 1,
                    _ => (),
                }
            }
        }
    }

    count.to_string()
}

fn get_directions() -> Vec<(i32, i32)> {
    // All cardinal & intercardinal directions
    vec![
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
}

fn build_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());

        for character in line.chars() {
            grid[i].push(character);
        }
    }

    return grid;
}
