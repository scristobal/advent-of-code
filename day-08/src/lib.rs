use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Tree {
    index: usize,
    size: u32,
}

pub fn solve_part1(input: &str) -> String {
    let field = input
        .replace("\n", "")
        .chars()
        .enumerate()
        .map(|(i, c)| Tree {
            index: i,
            size: c.to_digit(10).unwrap(),
        })
        .collect::<Vec<Tree>>();

    fn row(m: &Vec<Tree>, ind: usize) -> Vec<Tree> {
        let size = (m.len() as f32).sqrt() as usize;
        m.iter()
            .copied()
            .skip(size * ind)
            .take(size)
            .collect::<Vec<_>>()
    }

    fn col(m: &Vec<Tree>, ind: usize) -> Vec<Tree> {
        let size = (m.len() as f32).sqrt() as usize;
        m.iter()
            .copied()
            .skip(ind)
            .step_by(size)
            .take(size)
            .collect::<Vec<_>>()
    }

    fn visible(l: &Vec<Tree>) -> Vec<Tree> {
        l.iter()
            .scan(0, |max, &tree| {
                if tree.size + 1 > *max {
                    *max = tree.size + 1;
                    Some((tree, true))
                } else {
                    Some((tree, false))
                }
            })
            .filter_map(|(tree, visible)| if visible { Some(tree) } else { None })
            .collect()
    }

    fn visible_from_north(m: &Vec<Tree>) -> HashSet<Tree> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let column = col(&m, col_id);

                visible(&column)
            })
            .collect()
    }

    fn visible_from_south(m: &Vec<Tree>) -> HashSet<Tree> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let mut column = col(&m, col_id);

                column.reverse();

                visible(&column)
            })
            .collect()
    }

    fn visible_from_east(m: &Vec<Tree>) -> HashSet<Tree> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let row = row(&m, col_id);

                visible(&row)
            })
            .collect()
    }

    fn visible_from_west(m: &Vec<Tree>) -> HashSet<Tree> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let mut row = row(&m, col_id);

                row.reverse();

                visible(&row)
            })
            .collect()
    }

    let n = visible_from_north(&field);
    let s = visible_from_south(&field);
    let e = visible_from_east(&field);
    let w = visible_from_west(&field);

    let h = n.union(&s).copied().collect::<HashSet<_>>();
    let v = e.union(&w).copied().collect::<HashSet<_>>();

    let t = h.union(&v).copied().collect::<HashSet<_>>();

    t.len().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let field = input
        .replace("\n", "")
        .chars()
        .enumerate()
        .map(|(i, c)| Tree {
            index: i,
            size: c.to_digit(10).unwrap(),
        })
        .collect::<Vec<Tree>>();

    fn row(m: &Vec<Tree>, ind: usize) -> Vec<Tree> {
        let size = (m.len() as f32).sqrt() as usize;
        m.iter()
            .copied()
            .skip(size * ind)
            .take(size)
            .collect::<Vec<_>>()
    }

    //dbg!(row(&field, 2));

    fn col(m: &Vec<Tree>, ind: usize) -> Vec<Tree> {
        let size = (m.len() as f32).sqrt() as usize;
        m.iter()
            .copied()
            .skip(ind)
            .step_by(size)
            .take(size)
            .collect::<Vec<_>>()
    }

    fn scenic_score(l: &Vec<Tree>) -> HashMap<Tree, usize> {
        let rem = l.clone();

        l.iter()
            .scan(rem, |rem, tree| {
                let size = tree.size;
                rem.drain(0..1);

                let mut num_visible_trees = rem
                    .iter()
                    .clone()
                    .take_while(|&tree| tree.size < size)
                    .count();

                if num_visible_trees != rem.len() {
                    num_visible_trees += 1;
                }

                Some((*tree, num_visible_trees))
            })
            .collect()
    }

    fn scenic_score_from_north(m: &Vec<Tree>) -> HashMap<Tree, usize> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let column = col(&m, col_id);

                scenic_score(&column)
            })
            .collect()
    }

    fn scenic_score_from_south(m: &Vec<Tree>) -> HashMap<Tree, usize> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let mut column = col(&m, col_id);

                column.reverse();

                scenic_score(&column)
            })
            .collect()
    }

    fn scenic_score_from_east(m: &Vec<Tree>) -> HashMap<Tree, usize> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let row = row(&m, col_id);

                scenic_score(&row)
            })
            .collect()
    }

    fn scenic_score_from_west(m: &Vec<Tree>) -> HashMap<Tree, usize> {
        let size = (m.len() as f32).sqrt() as usize;

        (0..size)
            .flat_map(|col_id| {
                let mut row = row(&m, col_id);

                row.reverse();

                scenic_score(&row)
            })
            .collect()
    }

    let n = scenic_score_from_north(&field);
    let s = scenic_score_from_south(&field);
    let e = scenic_score_from_east(&field);
    let w = scenic_score_from_west(&field);

    field
        .iter()
        .map(|tree| {
            n.get(tree).unwrap()
                * s.get(tree).unwrap()
                * w.get(tree).unwrap()
                * e.get(tree).unwrap()
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "8");
    }
}
