use anyhow::Result;

#[derive(Default, Debug)]
struct Position {
    coords: (i32, i32),
    prove: (i32, i32),
}

#[derive(Debug)]
struct World {
    width: usize,
    height: usize,
    walls: Vec<Position>,
    rocks: Vec<Position>,
    robot: Position,
}
impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x as i32, y as i32);
                let ch = if self.walls.iter().any(|w| w.coords == pos) {
                    '#'
                } else if self.robot.coords == pos {
                    '@'
                } else if self.rocks.iter().any(|r| r.coords == pos) {
                    'O'
                } else {
                    '.'
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl World {
    fn update(&mut self, direction: &(i32, i32)) {
        self.push(direction);

        match self.is_legal() {
            true => self.apply(),
            false => self.discard(),
        }
    }

    fn push(&mut self, direction: &(i32, i32)) {
        self.robot.prove = (
            self.robot.coords.0 + direction.0,
            self.robot.coords.1 + direction.1,
        );

        let mut next_position = self.robot.prove;

        while let Some(rock) = self
            .rocks
            .iter_mut()
            .find(|rock| rock.coords == next_position)
        {
            rock.prove = (rock.coords.0 + direction.0, rock.coords.1 + direction.1);
            next_position = rock.prove
        }
    }

    fn is_legal(&self) -> bool {
        !self.walls.iter().any(|wall| {
            wall.coords == self.robot.prove
                || self.rocks.iter().any(|rock| wall.coords == rock.prove)
        })
    }

    fn discard(&mut self) {
        self.robot.prove = self.robot.coords;
        self.rocks
            .iter_mut()
            .for_each(|rock| rock.prove = rock.coords);
    }

    fn apply(&mut self) {
        self.robot.coords = self.robot.prove;
        self.rocks
            .iter_mut()
            .for_each(|rock| rock.coords = rock.prove);
    }
}

pub fn solve(input: &'static str) -> Result<String> {
    let width = input.lines().next().unwrap().chars().count();
    let height = input
        .lines()
        .filter(|line| line.chars().count() == width)
        .count();

    let mut input = input.split("\n\n");

    let mut world = World {
        width,
        height,
        walls: Vec::new(),
        rocks: Vec::new(),
        robot: Position::default(),
    };

    for (y, line) in input.next().unwrap().lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let position = Position {
                coords: (x as i32, y as i32),
                prove: (x as i32, y as i32),
            };

            match char {
                '#' => world.walls.push(position),
                'O' => world.rocks.push(position),
                '@' => world.robot = position,
                '.' => continue,
                _ => unreachable!(),
            }
        }
    }

    let mut movements: Vec<(i32, i32)> = input
        .next()
        .unwrap()
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => unreachable!(),
        })
        .rev()
        .collect();

    // println!("Initial state:\n{world}");

    while let Some(direction) = movements.pop() {
        world.update(&direction);
        // let r#move = match direction {
        //     (-1, 0) => "<",
        //     (1, 0) => ">",
        //     (0, -1) => "^",
        //     (0, 1) => "v",
        //     _ => unreachable!(),
        // };
        // println!("Move {move}:\n{world}");
    }

    let res = world
        .rocks
        .iter()
        .map(|rock| rock.coords.0 + 100 * rock.coords.1)
        .sum::<i32>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample_smaller() {
        #[rustfmt::skip]
        let result = solve(
"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
",
        )
        .unwrap();
        assert_eq!(result, "2028");
    }

    #[test]
    fn solve_sample_larger() {
        #[rustfmt::skip]
        let result = solve(
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
",
        )
        .unwrap();
        assert_eq!(result, "10092");
    }
}
