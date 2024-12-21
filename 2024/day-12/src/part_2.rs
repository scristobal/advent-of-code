use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

struct Zone {
    area: usize,
    corners: usize, // num_corners = num_sides
}

impl Zone {
    fn price(&self) -> usize {
        self.area * self.corners
    }
}

struct State {
    coords: (i32, i32),
    zone_id: usize,
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut map = HashMap::<(i32, i32), char>::with_capacity(140 * 140);

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), char);
        }
    }

    let mut zone_id_count = 0;
    let mut zones = HashMap::<usize, Zone>::new();
    let mut visited = HashSet::<(i32, i32)>::new();

    let mut queue: VecDeque<State> = VecDeque::from(vec![State {
        coords: (0, 0),
        zone_id: zone_id_count,
    }]);

    while let Some(state) = queue.pop_back() {
        if visited.contains(&state.coords) {
            continue;
        }

        let Some(char) = map.get(&state.coords) else {
            continue;
        };

        let mut check_next = |next_coord: (i32, i32)| match map.get(&next_coord) {
            Some(next_char) if next_char == char => {
                queue.push_back(State {
                    coords: next_coord,
                    zone_id: state.zone_id,
                });
            }
            Some(next_char) if next_char != char => {
                zone_id_count += 1;
                queue.push_front(State {
                    coords: next_coord,
                    zone_id: zone_id_count,
                });
            }
            _ => {}
        };

        check_next((state.coords.0 + 1, state.coords.1));
        check_next((state.coords.0, state.coords.1 + 1));
        check_next((state.coords.0 - 1, state.coords.1));
        check_next((state.coords.0, state.coords.1 - 1));

        let right = map.get(&(state.coords.0 + 1, state.coords.1));
        let left = map.get(&(state.coords.0 - 1, state.coords.1));
        let top = map.get(&(state.coords.0, state.coords.1 - 1));
        let bottom = map.get(&(state.coords.0, state.coords.1 + 1));

        let top_left = map.get(&(state.coords.0 - 1, state.coords.1 - 1));
        let top_right = map.get(&(state.coords.0 + 1, state.coords.1 - 1));
        let bottom_left = map.get(&(state.coords.0 - 1, state.coords.1 + 1));
        let bottom_right = map.get(&(state.coords.0 + 1, state.coords.1 + 1));

        let mut num_corners = 0;
        if right != Some(char) && top != Some(char) {
            num_corners += 1;
        }

        if right == Some(char) && top == Some(char) && top_right != Some(char) {
            num_corners += 1;
        };

        if right != Some(char) && bottom != Some(char) {
            num_corners += 1;
        }

        if right == Some(char) && bottom == Some(char) && bottom_right != Some(char) {
            num_corners += 1;
        };

        if left != Some(char) && bottom != Some(char) {
            num_corners += 1;
        }

        if left == Some(char) && bottom == Some(char) && bottom_left != Some(char) {
            num_corners += 1;
        };

        if left != Some(char) && top != Some(char) {
            num_corners += 1;
        }

        if left == Some(char) && top == Some(char) && top_left != Some(char) {
            num_corners += 1;
        };

        let zone = zones.entry(state.zone_id).or_insert(Zone {
            area: 0,
            corners: 0,
        });

        zone.corners += num_corners;
        zone.area += 1;

        visited.insert(state.coords);
    }

    Ok(zones.values().map(|z| z.price()).sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample1() {
        let result = solve(
            "AAAA
BBCD
BBCC
EEEC
",
        )
        .unwrap();
        assert_eq!(result, "80");
    }

    #[test]
    fn solve_sample2() {
        let result = solve(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        )
        .unwrap();
        assert_eq!(result, "436");
    }

    #[test]
    fn solve_sample3() {
        let result = solve(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        )
        .unwrap();
        assert_eq!(result, "236");
    }

    #[test]
    fn solve_sample4() {
        let result = solve(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        )
        .unwrap();
        assert_eq!(result, "368");
    }

    #[test]
    fn solve_sample5() {
        let result = solve(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        )
        .unwrap();
        assert_eq!(result, "1206");
    }
}
