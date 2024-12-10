pub fn parse_to_2d_char_array(s: &str) -> Vec<Vec<char>> {
    s.trim_end()
        .split("\n")
        .map(|v| v.chars().collect())
        .collect()
}

pub fn parse_to_2d_digit_array(s: &str) -> Vec<Vec<u32>> {
    s.trim_end()
        .split("\n")
        .map(|v| v.chars().map(|v| v.to_digit(10).unwrap()).collect())
        .collect()
}
