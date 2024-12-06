use regex::Regex;

pub fn solve() -> (String, String) {
    let input = include_str!("../../input/day3.txt");

    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    let mut is_on = true;

    // Parse input with regex to get commands.
    let regex = Regex::new(
        r#"(?x) 
        (do)\(\) |
        (don\'t)\(\) |
        (mul)\(([0-9]+),([0-9]+)\)
        "#,
    )
    .unwrap();

    let captures = regex.captures_iter(input).map(|captures| {
        captures
            .iter() // All the captured groups
            .skip(1) // Skipping the complete match
            .flat_map(|c| c) // Ignoring all empty optional matches
            .map(|c| c.as_str()) // Grab the original strings
            .collect::<Vec<_>>() // Create a vector
    });

    // Execute all commands found.
    for capture in captures {
        match Some(capture).as_ref().map(|c| c.as_slice()) {
            Some(["do"]) => is_on = true,
            Some(["don't"]) => is_on = false,
            Some(["mul", x, y]) => {
                let x: i64 = x.parse().unwrap();
                let y: i64 = y.parse().unwrap();
                let product = x * y;

                part1 += product;

                if is_on {
                    part2 += product;
                }
            }
            _ => (),
        }
    }

    (part1.to_string(), part2.to_string())
}
