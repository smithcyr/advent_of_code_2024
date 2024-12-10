use core::str;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Mul {
    pub left: i64,
    pub right: i64,
}

impl Mul {
    pub fn value(&self) -> i64 {
        &self.left * &self.right
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMulError;

impl str::FromStr for Mul {
    type Err = ParseMulError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix("mul(")
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParseMulError)?;

        let x_fromstr = x.parse::<i64>().map_err(|_| ParseMulError)?;
        let y_fromstr = y.parse::<i64>().map_err(|_| ParseMulError)?;
        Ok(Mul {
            left: x_fromstr,
            right: y_fromstr,
        })
    }
}

pub fn part_1() -> String {
    parse_program(&parse_input().as_str()).to_string()
}

pub fn part_2() -> String {
    let mut result: i64 = 0;
    for part in parse_input().split("do()") {
        result += parse_program(part.split("don't()").next().unwrap())
    }

    result.to_string()
}

fn parse_program(input: &str) -> i64 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut result: i64 = 0;
    for part in re.captures_iter(input) {
        let mut_str = part.get(0).unwrap().as_str();
        result += Mul::from_str(mut_str).unwrap().value();
    }
    result
}

fn parse_input() -> String {
    include_str!("input.txt").trim_end().to_string()
}
