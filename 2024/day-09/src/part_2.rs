use anyhow::Result;
use std::{collections::VecDeque, ops::Not as _};

#[derive(Debug)]
struct Block {
    size: u32,
    content: Option<u32>,
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut data = VecDeque::from(vec![]);

    let mut is_empty = false;
    let mut id = 0;

    for char in input.chars().filter_map(|c| c.to_digit(10)) {
        let block = Block {
            size: char,
            content: is_empty.not().then_some(id),
        };

        data.push_back(block);

        if !is_empty {
            id += 1;
        }

        is_empty = !is_empty;
    }

    let mut id = data[data.len() - 1].content.unwrap();

    while id > 0 {
        let id_ind = data
            .iter()
            .position(|block| block.content == Some(id))
            .unwrap();

        let id_size = data[id_ind].size;

        let mut empty_ind = None;

        for (ind, block) in data.iter().enumerate() {
            if block.content.is_none() && block.size >= id_size && ind < id_ind {
                empty_ind = Some(ind);
                break;
            }
        }

        if let Some(empty_ind) = empty_ind {
            let delta_size = data[empty_ind].size - id_size;

            data[id_ind].content = None;

            data[empty_ind].size = id_size;
            data[empty_ind].content = Some(id);

            if delta_size > 0 {
                let fill_block = Block {
                    size: delta_size,
                    content: None,
                };
                data.insert(empty_ind + 1, fill_block);
            }
        };

        id -= 1;
    }
    let mut res = 0_u64;
    let mut count = 0;

    for block in data {
        let Some(content) = block.content else {
            count += block.size as u64;
            continue;
        };

        for _ in 0..block.size {
            res += count * content as u64;
            count += 1;
        }
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "2858");
    }
}
