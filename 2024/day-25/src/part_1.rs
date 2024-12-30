use std::collections::HashMap;

use anyhow::Result;

pub fn solve(input: &'static str) -> Result<String> {
    let blocks = input.split("\n\n");

    let num_pins = 5;
    let ping_height = 7;

    let mut keys = vec![];
    let mut locks = vec![];

    for block in blocks {
        let mut grid = HashMap::new();

        for (y, line) in block.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid.insert((x, y), char);
            }
        }

        let is_key = block.lines().next().unwrap().chars().all(|c| c == '.');

        if is_key {
            keys.push(grid);
        } else {
            locks.push(grid);
        }
    }

    let mut count = 0;
    for key in &keys {
        'locks: for lock in &locks {
            for x in 0..num_pins {
                for y in 0..ping_height {
                    if matches!(lock.get(&(x, y)), Some(&'#'))
                        && matches!(key.get(&(x, y)), Some(&'#'))
                    {
                        continue 'locks;
                    }
                }
            }
            count += 1;
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let result = solve(
"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
").unwrap();

        assert_eq!(result, "3");
    }
}
