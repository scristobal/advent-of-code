fn main() {
    let s = include_str!("../../input/2025/day12.txt");

    println!("part 1: {}", solve_p1(s));
    // println!("part 2: {}", solve_p2(s));
}

type Shape = [[bool; 3]; 3];

fn solve_p1(s: &str) -> usize {
    let mut solvable = 0;

    let mut blocks = s.split("\n\n");

    let mut shapes: Box<[Shape]> = Box::new([[[false; 3]; 3]; 6]);
    let mut areas: Box<[u8]> = Box::new([0; 6]);

    for i in 0..=5 {
        let shape = blocks.next().unwrap().lines().skip(1);
        for (y, line) in shape.enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    shapes[i][y][x] = true;
                    areas[i] += 1;
                }
            }
        }
    }

    let mut grids: Vec<(u8, u8, Box<[u8]>)> = vec![];

    for grid in blocks.next().unwrap().lines() {
        let mut tokens = grid.split_whitespace();

        let (width, height) = tokens
            .next()
            .unwrap()
            .trim_matches(|c| c == ':')
            .split_once('x')
            .unwrap();

        let width = width.parse::<u8>().unwrap();
        let height = height.parse::<u8>().unwrap();

        let grid_size = width as usize * height as usize;

        let shapes_count = tokens.map(|s| s.parse::<u8>().unwrap()).collect::<Box<_>>();
        let shapes_total = shapes_count.iter().map(|n| *n as usize).sum::<usize>();

        if grid_size >= shapes_total * 9 {
            solvable += 1;
        }

        let shapes_area = shapes_count
            .iter()
            .map(|&i| areas[i as usize] as usize)
            .sum::<usize>();

        if grid_size > shapes_area {
            grids.push((width, height, shapes_count));
        }
    }

    if grids.is_empty() {
        return solvable;
    }

    // for each grid in grids, DFS with state current grid occupation and next state using masks:
    //      4 (rotations) x 2 (flip) x width-3 (x-translations) x height-3 (y-translations) x 5 (shapes)
    todo!()
}

fn solve_p2(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn sample_p1() {
        assert_eq!(solve_p1(SAMPLE), 2)
    }

    #[test]
    #[ignore]
    fn sample_p2() {
        assert_eq!(solve_p2(SAMPLE), 0)
    }
}
