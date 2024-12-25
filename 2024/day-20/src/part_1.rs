use anyhow::Result;
use pathfinding::prelude::dijkstra_all;
use std::collections::{HashMap, HashSet};

fn parse(input: &'static str) -> (HashSet<(i32, i32)>, (i32, i32), (i32, i32)) {
    let mut walls = HashSet::new();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let coords = (x as i32, y as i32);
            match char {
                '#' => {
                    walls.insert(coords);
                }
                'S' => start = coords,
                'E' => end = coords,
                '.' => continue,
                _ => unreachable!(),
            }
        }
    }

    (walls, start, end)
}

fn dirs(coord: &(i32, i32)) -> [(i32, i32); 4] {
    [
        (coord.0, coord.1 - 1),
        (coord.0, coord.1 + 1),
        (coord.0 + 1, coord.1),
        (coord.0 - 1, coord.1),
    ]
}

fn successors(&coord: &(i32, i32), walls: &HashSet<(i32, i32)>) -> Vec<((i32, i32), usize)> {
    dirs(&coord)
        .into_iter()
        .filter_map(|coord: (i32, i32)| (!walls.contains(&coord)).then_some((coord, 1)))
        .collect()
}

fn shortcuts(
    start: &(i32, i32),
    end: &(i32, i32),
    walls: &HashSet<(i32, i32)>,
) -> HashMap<usize, Vec<(i32, i32)>> {
    let mut reacheables_from_start = dijkstra_all(start, |coord| successors(coord, walls));
    reacheables_from_start.insert(*start, (*start, 0));

    let mut reacheables_from_end = dijkstra_all(end, |coord| successors(coord, walls));
    reacheables_from_end.insert(*end, (*end, 0));

    let (_, min_time) = reacheables_from_start.get(end).unwrap();

    let mut shortcuts = HashMap::with_capacity(walls.len());

    for coord in walls {
        let dist_to_start = dirs(coord)
            .iter()
            .filter_map(|dir| reacheables_from_start.get(dir).map(|(_, time)| time))
            .min();

        let dist_to_end = dirs(coord)
            .iter()
            .filter_map(|dir| reacheables_from_end.get(dir).map(|(_, time)| time))
            .min();

        let total_time = dist_to_start.and_then(|&dist_to_start| {
            dist_to_end.map(|&dist_to_end| dist_to_end + dist_to_start + 2)
        });

        if let Some(total_time) = total_time
            && *min_time > total_time
        {
            shortcuts
                .entry(min_time - total_time)
                .and_modify(|e: &mut Vec<(i32, i32)>| e.push(*coord))
                .or_insert(vec![*coord]);
        }
    }

    shortcuts
}
pub fn solve(input: &'static str) -> Result<String> {
    let (walls, start, end) = parse(input);

    let shortcuts = shortcuts(&start, &end, &walls);

    let res: usize = shortcuts
        .iter()
        .filter(|(&saved_time, _)| saved_time >= 100)
        .map(|(_, v)| v.len())
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn solve_sample1() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&2).unwrap_or(&vec![]).len();

        assert_eq!(res, 14);
    }

    #[test]
    fn solve_sample2() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&4).unwrap_or(&vec![]).len();

        assert_eq!(res, 14);
    }

    #[test]
    fn solve_sampler3() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&6).unwrap_or(&vec![]).len();

        assert_eq!(res, 2);
    }

    #[test]
    fn solve_sample4() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&8).unwrap_or(&vec![]).len();

        assert_eq!(res, 4);
    }

    #[test]
    fn solve_sample5() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&10).unwrap_or(&vec![]).len();

        assert_eq!(res, 2);
    }

    #[test]
    fn solve_sample6() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&12).unwrap_or(&vec![]).len();

        assert_eq!(res, 3);
    }

    #[test]
    fn solve_sample7() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&20).unwrap_or(&vec![]).len();

        assert_eq!(res, 1);
    }

    #[test]
    fn solve_sample8() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&36).unwrap_or(&vec![]).len();

        assert_eq!(res, 1);
    }

    #[test]
    fn solve_sample9() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&38).unwrap_or(&vec![]).len();

        assert_eq!(res, 1);
    }

    #[test]
    fn solve_sample10() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&40).unwrap_or(&vec![]).len();

        assert_eq!(res, 1);
    }

    #[test]
    fn solve_sample11() {
        let (walls, start, end) = parse(SAMPLE);

        let shortcuts = shortcuts(&start, &end, &walls);
        let res = shortcuts.get(&64).unwrap_or(&vec![]).len();

        assert_eq!(res, 1);
    }
}
