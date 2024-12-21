use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

struct Zone {
    area: usize,
    perimeter: usize,
}

impl Zone {
    fn price(&self) -> usize {
        self.area * self.perimeter
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

        let zone = zones.entry(state.zone_id).or_insert(Zone {
            area: 0,
            perimeter: 0,
        });

        zone.area += 1;

        let mut check_next = |next_coord: (i32, i32)| match map.get(&next_coord) {
            Some(next_char) if next_char == char => {
                queue.push_back(State {
                    coords: next_coord,
                    zone_id: state.zone_id,
                });
            }
            Some(next_char) if next_char != char => {
                zone.perimeter += 1;

                zone_id_count += 1;
                queue.push_front(State {
                    coords: next_coord,
                    zone_id: zone_id_count,
                });
            }
            None => {
                zone.perimeter += 1;
            }
            _ => unreachable!(),
        };

        check_next((state.coords.0 + 1, state.coords.1));
        check_next((state.coords.0, state.coords.1 + 1));
        check_next((state.coords.0 - 1, state.coords.1));
        check_next((state.coords.0, state.coords.1 - 1));

        visited.insert(state.coords);
    }

    Ok(zones.values().map(|z| z.price()).sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "1930");
    }
}
