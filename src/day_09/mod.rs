use std::fmt::{self, Display};

#[derive(Clone)]
struct Block {
    id: usize,
    size: u32,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.size)
    }
}

pub fn part_1() -> String {
    part_1_impl(include_str!("input.txt")).to_string()
}

fn part_1_impl(input: &str) -> u128 {
    let mut blocks: Vec<Block> = Vec::new();
    for (index, c) in input
        .trim()
        .chars()
        .map(|v| v.to_digit(10).unwrap())
        .enumerate()
    {
        blocks.push(Block {
            id: index / 2,
            size: c,
        })
    }
    let mut blocks_rev = blocks.to_owned();
    let mut last_file_iter = blocks_rev
        .iter_mut()
        .enumerate()
        .filter(|(s, _)| *s % 2 == 0)
        .rev();
    let mut last_file = last_file_iter.next().unwrap();
    let mut result: Vec<Block> = Vec::new();
    for (index, block) in blocks.iter_mut().enumerate() {
        if index > last_file.0 {
            break;
        }
        if index == last_file.0 {
            result.push(last_file.1.to_owned());
            break;
        }
        if index % 2 == 0 {
            result.push(block.to_owned());
            continue;
        } else {
            // this fills in small files into the large free space
            while block.size >= last_file.1.size {
                result.push(last_file.1.to_owned());
                block.size -= last_file.1.size;
                last_file = last_file_iter.next().unwrap();
            }
            // this fills in parts of a large file into a smaller free space
            if block.size != 0 {
                result.push(Block {
                    id: last_file.1.id,
                    size: block.size,
                });
                last_file.1.size -= block.size;
            }
        }
    }
    let mut checksum: u128 = 0;
    let mut index = 0;
    for block in result.iter() {
        for _ in 0..block.size {
            checksum += (index * block.id) as u128;
            index += 1;
        }
    }

    checksum
}

pub fn part_2() -> String {
    part_2_impl(include_str!("input.txt")).to_string()
}

#[derive(Clone)]
struct Block2 {
    id: u128,
    file: u32,
    space: u32,
}

fn part_2_impl(input: &str) -> u128 {
    let mut blocks: Vec<Block2> = Vec::new();
    let mut chars = input.trim().chars().map(|v| v.to_digit(10).unwrap());
    let mut id = 0;
    loop {
        let file = match chars.next() {
            Some(v) => v,
            None => break,
        };

        let space = match chars.next() {
            Some(v) => v,
            None => 0,
        };
        blocks.push(Block2 { id, file, space });
        id += 1;
    }
    let mut last = blocks.len() - 1;
    while last > 0 {
        if blocks[last].file > 0 {
            let mut index: usize = 0;
            loop {
                if index == last {
                    break;
                }
                if blocks[index].space >= blocks[last].file {
                    let new_block = Block2 {
                        id: blocks[last].id,
                        file: blocks[last].file,
                        space: blocks[index].space - blocks[last].file,
                    };
                    blocks[last].space += blocks[last].file;
                    blocks[last].file = 0;
                    blocks[index].space = 0;
                    let _: Vec<_> = blocks
                        .splice(index + 1..index + 1, vec![new_block])
                        .collect();
                    last += 1;
                    break;
                }
                index += 1;
            }
        }
        last -= 1;
    }

    let mut checksum: u128 = 0;
    let mut index: u128 = 0;
    for block in blocks.iter() {
        for _ in 0..block.file {
            checksum += index * block.id;
            index += 1;
        }
        index += block.space as u128;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "2333133121414131402";

    #[test]
    fn part_1() {
        assert_eq!(part_1_impl(TEST_STR), 1928)
    }

    #[test]
    fn part_2() {
        assert_eq!(part_2_impl(TEST_STR), 2858)
    }
}
