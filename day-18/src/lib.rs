use std::collections::HashSet;

use itertools::Itertools;

fn parse(s: &str) -> HashSet<[i32; 3]> {
    s.lines()
        .map(|l| {
            let v = l
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            [v[0], v[1], v[2]]
        })
        .collect()
}

pub fn solve_part1(input: &str) -> String {
    let cubes = parse(input);

    let faces_between_blocks = cubes
        .iter()
        .map(|cube| {
            (0..3_usize)
                .cartesian_product([-1, 1].into_iter())
                .filter(|(dim, dir)| {
                    let mut adjacent = *cube;
                    adjacent[*dim] += dir;
                    cubes.contains(&adjacent)
                })
                .count()
        })
        .sum::<usize>();

    (6 * cubes.len() - faces_between_blocks).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let cubes = parse(input); // original problems set of cubes

    let corner = *cubes.iter().min().unwrap(); // guarantees start faces outside
    let start = (corner, 0, -1); // (cube, dimension, direction)

    let mut stack = vec![start]; // DFS stack of faces facing outside

    let mut visited = HashSet::new(); // DFS visited

    let mut surface = 0; // outside faces, algorithm output

    while let Some(face) = stack.pop() {
        if !visited.insert(face) {
            continue;
        }

        let (cube, face_dim, face_dir) = face;

        surface += 1; // warrantied to be facing outside, see (1) (2) and (3)

        for (dim, dir) in (0..3).cartesian_product([-1, 1].into_iter()) {
            let mut diagonal = cube;
            diagonal[face_dim] += face_dir;
            diagonal[dim] += dir;

            if cubes.contains(&diagonal) {
                stack.push((diagonal, dim, -dir)); // (1) if diagonal cube in set, faces are connected
                continue;
            }

            let mut adjunct = cube;
            adjunct[dim] += dir; //

            if cubes.contains(&adjunct) {
                stack.push((adjunct, face_dim, face_dir)); // (2) if diagonal cube not in set, but adjacent is, faces are connected
                continue;
            }

            stack.push((cube, dim, dir)); // (3) no diagonal nor adjacent in set, contiguous face is connected
        }
    }

    surface.to_string()
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
