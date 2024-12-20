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

    let mut inc_ind = 1;
    assert!(data[inc_ind].content.is_none());

    let mut dec_ind = data.len() - 1;
    assert!(data[dec_ind].content.is_some());

    while inc_ind < dec_ind {
        let mut low_block = data.remove(inc_ind).unwrap();
        dec_ind -= 1;

        let mut high_block = data.remove(dec_ind).unwrap();

        if high_block.size >= low_block.size {
            low_block.content = high_block.content;

            if high_block.size > low_block.size {
                high_block.size -= low_block.size;
                data.insert(dec_ind, high_block);
            } else {
                dec_ind -= 2;
            }

            data.insert(inc_ind, low_block);
            dec_ind += 1;
            inc_ind += 2;

            continue;
        }

        // empty_block.size < last_used_block.size

        dec_ind -= 2;

        low_block.size -= high_block.size;

        data.insert(inc_ind, high_block);
        inc_ind += 1;
        dec_ind += 1;

        data.insert(inc_ind, low_block);
        dec_ind += 1;
    }

    let mut res = 0_u64;
    let mut count = 0;

    for block in data {
        let Some(content) = block.content else {
            break;
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
        assert_eq!(result, "1928");
    }
}
