pub fn part_1() -> String {
    let rows = parse_input();
    let mut result: i32 = 0;
    for row in rows {
        if valid_row(&row) {
            result += 1;
        }
    }
    result.to_string()
}

pub fn part_2() -> String {
    let rows = parse_input();
    let mut result: i32 = 0;
    for row in rows {
        if valid_row(&row) {
            result += 1;
        } else {
            let row_len = row.len();
            for index in 0..row_len {
                let omitted_row = [&row[0..index], &row[index + 1..]].concat();
                if valid_row(&omitted_row) {
                    result += 1;
                    break;
                }
            }
        }
    }
    result.to_string()
}

fn valid_row(row: &Vec<i32>) -> bool {
    let row_len = row.len();
    if row_len == 1 {
        return true;
    } else {
        let mut direction: i8 = 0;
        for index in 1..row_len {
            let diff = row[index] - row[index - 1];
            if !valid_diff(&diff, &direction) {
                return false;
            }
            if direction == 0 {
                direction = if diff > 0 { 1 } else { -1 }
            }
        }
    }
    return true;
}

fn valid_diff(diff: &i32, direction: &i8) -> bool {
    return *diff != 0
        && diff.abs() <= 3
        && (*direction == 0 || *direction == -1 && *diff < 0 || *direction == 1 && *diff > 0);
}

fn parse_input() -> Vec<Vec<i32>> {
    include_str!("input.txt")
        .trim_end()
        .split("\n")
        .map(|s| s.split(" ").map(|v| v.parse().unwrap()).collect())
        .collect()
}
