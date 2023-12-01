use std::fs::File;
use std::io::{BufRead, BufReader};

fn value(played: &str) -> usize {
    return match played {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => unreachable!("wrong input {}", played),
    };
}

fn match_result(played: &str) -> usize {
    return match played {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => unreachable!("wrong input {}", played),
    };
}

fn fix(n: usize) -> usize {
    if n == 0 {
        return 3;
    }
    n
}

fn me_value(elf: &str, result: &str) -> usize {
    match result {
        "Y" => value(elf),
        "X" => fix((value(elf) - 1).rem_euclid(3)),
        "Z" => fix((value(elf) + 1).rem_euclid(3)),
        _ => 0,
    }
}

fn main() {
    let filename = "input/1";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;

    let r = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split_once(" ").collect());

    /*
        for line in reader.lines() {
            let line = line.unwrap();

            let Some((elf, outcome)) = line.split_once(" ") else {
                return
            };

            dbg!(me_value(elf, outcome));

            dbg!(match_result(outcome));

            score = score + match_result(outcome) + me_value(elf, outcome);
        }
    */

    println!("{}", score)
}
