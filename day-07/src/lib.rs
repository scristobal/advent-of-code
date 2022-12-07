use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::multi::separated_list1;
use nom::{character::complete::alpha1, IResult};

#[derive(Debug)]
enum InputLine<'a> {
    ChangeDir(&'a str),
    ListDir,
    File(u32),
    Dir(&'a str),
}

fn cd(input: &str) -> IResult<&str, InputLine> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, directory_name) = alt((alpha1, tag("/"), tag("..")))(input)?;

    Ok((input, InputLine::ChangeDir(directory_name)))
}

fn ls(input: &str) -> IResult<&str, InputLine> {
    let (input, _) = tag("$ ls")(input)?;

    Ok((input, InputLine::ListDir))
}

fn file(input: &str) -> IResult<&str, InputLine> {
    let (input, size) = complete::u32(input)?;

    Ok((input, InputLine::File(size)))
}

fn directory(input: &str) -> IResult<&str, InputLine> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, InputLine::Dir(name)))
}

fn log(input: &str) -> IResult<&str, Vec<InputLine>> {
    separated_list1(newline, alt((cd, ls, file, directory)))(input)
}

fn create_fs(input: &str) -> HashMap<String, u32> {
    let logs = log(input).unwrap().1;

    let mut fs = HashMap::<String, u32>::new();

    let mut current_dir = "".to_string();

    logs.iter().for_each(|line| match *line {
        InputLine::ChangeDir(to) => {
            current_dir = match to {
                "/" => "".to_string(),
                ".." => {
                    let (res, _) = current_dir.rsplit_once("/").unwrap();

                    res.to_string()
                }
                _ => [current_dir.clone(), to.to_string()].join("/"),
            };
        }
        InputLine::File(file) => {
            fs.entry(current_dir.clone())
                .and_modify(|size| *size += file)
                .or_insert(file);

            let mut res = current_dir.rsplit_once("/");

            loop {
                let Some(up_dir) = res else{
                            break;
                        };

                fs.entry(up_dir.0.to_string())
                    .and_modify(|size| *size += file)
                    .or_insert(file);

                res = up_dir.0.rsplit_once("/");
            }
        }
        InputLine::ListDir => {}
        InputLine::Dir(_) => {}
    });

    fs
}

pub fn solve_part1(input: &str) -> String {
    let fs = create_fs(input);
    fs.iter()
        .filter_map(|(_, size)| if *size < 100000 { Some(size) } else { None })
        .sum::<u32>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let fs = create_fs(input);

    let free_space = 70_000_000 - fs.get("").unwrap();

    let required_free = 30_000_000 - free_space;

    fs.iter()
        .filter_map(|(_, size)| {
            if *size > required_free {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "24933642");
    }
}
