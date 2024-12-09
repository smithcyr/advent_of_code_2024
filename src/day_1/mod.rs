use std::{collections::HashMap, iter::zip};

pub fn part_1() -> String {
    let rows = parse_input();
    let mut left: Vec<i32> = vec![0; rows.len()];
    let mut right: Vec<i32> = vec![0; rows.len()];

    for (i, row) in rows.iter().enumerate() {
        left[i] = row[0];
        right[i] = row[1];
    }

    left.sort();
    right.sort();
    let mut result: i32 = 0;
    for pair in zip(left, right) {
        result += (pair.1 - pair.0).abs();
    }
    result.to_string()
}

pub fn part_2() -> String {
    let rows = parse_input();
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    let mut left: Vec<i32> = vec![0; rows.len()];
    let mut right: Vec<i32> = vec![0; rows.len()];

    for (i, row) in rows.iter().enumerate() {
        left[i] = row[0];
        match right_counts.get_mut(&row[1]) {
            Some(v) => {
                *v += 1;
            }
            None => {
                right_counts.insert(row[1], 1);
            }
        }
        right[i] = row[1];
    }

    let mut result: i32 = 0;
    for v in left {
        if let Some(count) = right_counts.get(&v) {
            result += count * v;
        }
    }
    result.to_string()
}

fn parse_input() -> Vec<Vec<i32>> {
    include_str!("input.txt")
        .trim_end()
        .split("\n")
        .map(|s| s.split("   ").map(|v| v.parse().unwrap()).collect())
        .collect()
}
