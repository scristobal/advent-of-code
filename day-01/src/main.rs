use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input/1";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut elves_calories = BinaryHeap::new();

    let mut current_elf_calories_accumulator = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            elves_calories.push(current_elf_calories_accumulator);
            current_elf_calories_accumulator = 0;
            continue;
        }

        current_elf_calories_accumulator += line.parse::<i32>().unwrap()
    }

    let max_calories = elves_calories.pop().unwrap();

    println!("max calories carried by a single elf are {}", max_calories);

    let top_3_calories =
        elves_calories.pop().unwrap() + elves_calories.pop().unwrap() + max_calories;

    println!(
        "total calories carried by top 3 elves is {}",
        top_3_calories
    )
}
