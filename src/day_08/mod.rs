use core::str;
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    str::FromStr,
};

use crate::utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Antennas {
    map: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Point>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseAntennasError;

impl str::FromStr for Antennas {
    type Err = ParseAntennasError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = utils::parse_to_2d_char_array(s);
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        for (y, row) in map.iter().enumerate() {
            for (x, f) in row.iter().enumerate() {
                if *f != '.' {
                    let point = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    match antennas.get_mut(f) {
                        Some(l) => {
                            l.push(point);
                        }
                        None => {
                            antennas.insert(*f, vec![point]);
                        }
                    }
                }
            }
        }
        Ok(Antennas { antennas, map })
    }
}

pub fn part_1() -> String {
    let map = Antennas::from_str(include_str!("input.txt")).unwrap();
    count_overlaps(&map).to_string()
}

fn count_overlaps(map: &Antennas) -> usize {
    let max_x = map.map.len() as i32;
    let max_y = map.map[0].len() as i32;
    let mut overlaps: HashSet<Point> = HashSet::new();
    for (_, points) in &map.antennas {
        for i in 0..(points.len() - 1) {
            let a = points[i];
            for b in points[i + 1..].iter() {
                let ab = Point {
                    x: a.x + a.x - b.x,
                    y: a.y + a.y - b.y,
                };
                if ab.x >= 0 && ab.x < max_x && ab.y >= 0 && ab.y < max_y {
                    overlaps.insert(ab);
                }
                let ba = Point {
                    x: b.x + b.x - a.x,
                    y: b.y + b.y - a.y,
                };
                if ba.x >= 0 && ba.x < max_x && ba.y >= 0 && ba.y < max_y {
                    overlaps.insert(ba);
                }
            }
        }
    }
    overlaps
        .iter()
        .filter(|p| p.x >= 0 && p.x < max_x && p.y >= 0 && p.y < max_y)
        .count()
}

fn count_t_overlaps(map: &Antennas) -> usize {
    let max_x = map.map.len() as i32;
    let max_y = map.map[0].len() as i32;
    let mut overlaps: HashSet<Point> = HashSet::new();
    for (_, points) in &map.antennas {
        for i in 0..(points.len() - 1) {
            let a = points[i];
            for b in points[i + 1..].iter() {
                overlaps.insert(a);
                overlaps.insert(*b);
                for harmonic in 1..50 {
                    let ab = Point {
                        x: a.x + harmonic * (a.x - b.x),
                        y: a.y + harmonic * (a.y - b.y),
                    };
                    if ab.x >= 0 && ab.x < max_x && ab.y >= 0 && ab.y < max_y {
                        overlaps.insert(ab);
                    }
                    let ba = Point {
                        x: b.x + harmonic * (b.x - a.x),
                        y: b.y + harmonic * (b.y - a.y),
                    };
                    if ba.x >= 0 && ba.x < max_x && ba.y >= 0 && ba.y < max_y {
                        overlaps.insert(ba);
                    }
                }
            }
        }
    }
    overlaps.len()
}

pub fn part_2() -> String {
    let map = Antennas::from_str(include_str!("input.txt")).unwrap();
    count_t_overlaps(&map).to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn part_1() {
        let a = Antennas::from_str(TEST_STR).unwrap();
        assert_eq!(count_overlaps(&a), 14)
    }

    #[test]
    fn part_2() {
        let a = Antennas::from_str(TEST_STR).unwrap();
        assert_eq!(count_t_overlaps(&a), 34)
    }
}
