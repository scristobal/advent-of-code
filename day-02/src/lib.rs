pub mod part1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn value(played: &str) -> usize {
        match played {
            "A" | "X" => 1,
            "B" | "Y" => 2,
            "C" | "Z" => 3,
            _ => 0,
        }
    }

    fn win(elf: &str, me: &str) -> usize {
        let difference = value(elf) as i32 - value(me) as i32;

        match (difference).rem_euclid(3) {
            0 => 3,
            1 => 0,
            2 => 6,
            _ => 0,
        }
    }

    pub fn main() {
        let filename = "input/1";

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut score = 0;

        for line in reader.lines() {
            let line = line.unwrap();

            let Some((elf, me)) = line.split_once(' ') else {
            return
        };

            dbg!(value(me));
            dbg!(win(elf, me));

            score = score + win(elf, me) + value(me);

            dbg!(score);
        }

        println!("{}", score)
    }
}
