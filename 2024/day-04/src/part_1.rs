use anyhow::Result;
use itertools::Itertools;

struct Grid {
    d: Vec<u8>,
    w: i32,
    h: i32,
}

impl Grid {
    pub fn new(input: &'static str) -> Result<Grid> {
        let w = input
            .bytes()
            .position(|c| c == b'\n')
            .unwrap_or(input.len())
            .try_into()?;

        let h = (input.bytes().filter(|&c| c == b'\n').count() + 1).try_into()?;

        let d = input.bytes().filter(|&c| c != b'\n').collect();

        Ok(Grid { d, w, h })
    }

    fn check_one(&self, x: i32, y: i32, v: u8) -> bool {
        if x >= self.w || x < 0 || y >= self.h || y < 0 {
            return false;
        }
        match self.d.get((x + self.w * y) as usize) {
            Some(&d) => d == v,
            None => false,
        }
    }

    fn check_dirs(&self, x: i32, y: i32) -> u32 {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(dx, dy)| {
                self.check_one(x, y, b'X')
                    && self.check_one(x + dx, y + dy, b'M')
                    && self.check_one(x + 2 * dx, y + 2 * dy, b'A')
                    && self.check_one(x + 3 * dx, y + 3 * dy, b'S')
            })
            .count() as u32
    }

    pub fn solve(&self) -> u32 {
        (0..self.w)
            .cartesian_product(0..self.h)
            .map(|(x, y)| self.check_dirs(x, y))
            .sum()
    }
}

pub fn solve(input: &'static str) -> Result<String> {
    let grid = Grid::new(input)?;

    Ok(grid.solve().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "18");
    }
}
