use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use std::collections::HashSet;

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

fn count_shortcuts(
    start: &(i32, i32),
    end: &(i32, i32),
    time: usize,
    walls: &HashSet<(i32, i32)>,
) -> usize {
    let mut reacheables_from_start = dijkstra_all(start, |coord| successors(coord, walls));
    reacheables_from_start.insert(*start, (*start, 0));

    let mut reacheables_from_end = dijkstra_all(end, |coord| successors(coord, walls));
    reacheables_from_end.insert(*end, (*end, 0));

    let (_, min_time) = reacheables_from_start.get(end).unwrap();

    let mut count = 0;

    for ((p, (_, dist_p)), (q, (_, dist_q))) in reacheables_from_start
        .iter()
        .cartesian_product(&reacheables_from_end)
    {
        let dist_mh = (p.0.abs_diff(q.0) + p.1.abs_diff(q.1)) as usize;

        if dist_mh > 20 || *min_time < (dist_p + dist_q + dist_mh) {
            continue;
        }

        if min_time - (dist_p + dist_q + dist_mh) >= time {
            count += 1;
        }
    }

    count
}

pub fn solve(input: &'static str) -> Result<String> {
    let (walls, start, end) = parse(input);
    Ok(count_shortcuts(&start, &end, 100, &walls).to_string())
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
    fn solve_sample2() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 50, &walls), 285);
    }

    #[test]
    fn solve_sampler3() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 52, &walls), 253);
    }

    #[test]
    fn solve_sample4() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 54, &walls), 222);
    }

    #[test]
    fn solve_sample5() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 56, &walls), 193);
    }

    #[test]
    fn solve_sample6() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 58, &walls), 154);
    }

    #[test]
    fn solve_sample7() {
        let (walls, start, end) = parse(SAMPLE);

        assert_eq!(count_shortcuts(&start, &end, 60, &walls), 129);
    }

    #[test]
    fn solve_sample8() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 62, &walls), 106);
    }

    #[test]
    fn solve_sample9() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 64, &walls), 86);
    }

    #[test]
    fn solve_sample10() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 66, &walls), 67);
    }

    #[test]
    fn solve_sample11() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 68, &walls), 55);
    }

    #[test]
    fn solve_sample12() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 70, &walls), 41);
    }

    #[test]
    fn solve_sample13() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 72, &walls), 29);
    }

    #[test]
    fn solve_sample14() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 74, &walls), 7);
    }

    #[test]
    fn solve_sample15() {
        let (walls, start, end) = parse(SAMPLE);
        assert_eq!(count_shortcuts(&start, &end, 76, &walls), 3);
    }
}
