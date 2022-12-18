use std::collections::HashSet;

fn parse(s: &str) -> HashSet<(i32, i32, i32)> {
    s.lines()
        .map(|l| {
            let v = l
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        })
        .collect()
}

pub fn solve_part1(input: &str) -> String {
    let cubes = parse(input);

    let mut surface = 0;

    for cube in &cubes {
        surface += 6;

        if cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            surface -= 1;
        }
        if cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            surface -= 1;
        }
        if cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            surface -= 1;
        }
        if cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            surface -= 1;
        }
        if cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            surface -= 1;
        }
        if cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            surface -= 1;
        }
    }

    surface.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let cubes = parse(input);

    let mut surface = 0;

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "64");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "58");
    }
}
