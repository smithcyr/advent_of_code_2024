pub fn part_1() -> String {
    let input = parse_input();
    let len_y = input.len();
    let len_x = input[0].len();
    let mut result: i32 = 0;
    for y in 0..len_y {
        for x in 0..len_x {
            let left: bool = x > 2;
            let right = x < len_x - 3;
            let top = y > 2;
            let bottom = y < len_y - 3;
            if left {
                result += is_xmas(
                    input[y][x],
                    input[y][x - 1],
                    input[y][x - 2],
                    input[y][x - 3],
                )
            }
            if top && left {
                result += is_xmas(
                    input[y][x],
                    input[y - 1][x - 1],
                    input[y - 2][x - 2],
                    input[y - 3][x - 3],
                )
            }
            if top {
                result += is_xmas(
                    input[y][x],
                    input[y - 1][x],
                    input[y - 2][x],
                    input[y - 3][x],
                )
            }
            if top && right {
                result += is_xmas(
                    input[y][x],
                    input[y - 1][x + 1],
                    input[y - 2][x + 2],
                    input[y - 3][x + 3],
                )
            }
            if right {
                result += is_xmas(
                    input[y][x],
                    input[y][x + 1],
                    input[y][x + 2],
                    input[y][x + 3],
                )
            }
            if bottom && right {
                result += is_xmas(
                    input[y][x],
                    input[y + 1][x + 1],
                    input[y + 2][x + 2],
                    input[y + 3][x + 3],
                )
            }
            if bottom {
                result += is_xmas(
                    input[y][x],
                    input[y + 1][x],
                    input[y + 2][x],
                    input[y + 3][x],
                )
            }
            if bottom && left {
                result += is_xmas(
                    input[y][x],
                    input[y + 1][x - 1],
                    input[y + 2][x - 2],
                    input[y + 3][x - 3],
                )
            }
        }
    }
    result.to_string()
}

pub fn part_2() -> String {
    let input = parse_input();
    let len_y = input.len();
    let len_x = input[0].len();
    let mut result: i32 = 0;
    for y in 0..len_y {
        for x in 0..len_x {
            let left: bool = x > 0;
            let right = x < len_x - 1;
            let top = y > 0;
            let bottom = y < len_y - 1;

            if top && left && bottom && right {
                result += is_x_mas(
                    input[y - 1][x - 1],
                    input[y - 1][x + 1],
                    input[y][x],
                    input[y + 1][x - 1],
                    input[y + 1][x + 1],
                )
            }
        }
    }
    result.to_string()
}

fn is_xmas(a: char, b: char, c: char, d: char) -> i32 {
    if a == 'X' && b == 'M' && c == 'A' && d == 'S' {
        1
    } else {
        0
    }
}

fn is_x_mas(a: char, c: char, e: char, g: char, i: char) -> i32 {
    if (a == 'M' && e == 'A' && i == 'S' || a == 'S' && e == 'A' && i == 'M')
        && (c == 'M' && e == 'A' && g == 'S' || c == 'S' && e == 'A' && g == 'M')
    {
        1
    } else {
        0
    }
}

fn parse_input() -> Vec<Vec<char>> {
    include_str!("input.txt")
        .trim_end()
        .split("\n")
        .map(|v| v.chars().collect())
        .collect()
}
