pub fn parse_to_2d_char_array(s: &str) -> Vec<Vec<char>> {
    s.trim_end()
        .split("\n")
        .map(|v| v.chars().collect())
        .collect()
}
