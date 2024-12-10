use std::collections::{HashMap, HashSet};

use crate::utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Trail {
    start: Point,
    position: Point,
}

pub fn part_1() -> String {
    sum_trailheads(include_str!("input.txt")).to_string()
}

pub fn part_2() -> String {
    sum_distinct_trails(include_str!("input.txt")).to_string()
}

pub fn sum_trailheads(input: &str) -> u64 {
    let map = utils::parse_to_2d_digit_array(input);
    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;
    let mut wave: HashSet<Trail> = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if *v == 0 {
                wave.insert(Trail {
                    start: Point { x, y },
                    position: Point { x, y },
                });
            }
        }
    }
    let mut completed: HashMap<Point, HashSet<Point>> = HashMap::new();
    for _ in 0..10 {
        let mut next_wave: HashSet<Trail> = HashSet::new();
        for trail in wave.iter() {
            let current = map[trail.position.y][trail.position.x];
            // up
            if trail.position.y > 0 && current + 1 == map[trail.position.y - 1][trail.position.x] {
                let next = Point {
                    x: trail.position.x,
                    y: trail.position.y - 1,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            v.insert(next);
                        }
                        None => {
                            completed.insert(trail.start, HashSet::from_iter(vec![next]));
                        }
                    }
                } else {
                    next_wave.insert(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
            // down
            if trail.position.y < max_y
                && current + 1 == map[trail.position.y + 1][trail.position.x]
            {
                let next = Point {
                    x: trail.position.x,
                    y: trail.position.y + 1,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            v.insert(next);
                        }
                        None => {
                            completed.insert(trail.start, HashSet::from_iter(vec![next]));
                        }
                    }
                } else {
                    next_wave.insert(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
            // left
            if trail.position.x > 0 && current + 1 == map[trail.position.y][trail.position.x - 1] {
                let next = Point {
                    x: trail.position.x - 1,
                    y: trail.position.y,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            v.insert(next);
                        }
                        None => {
                            completed.insert(trail.start, HashSet::from_iter(vec![next]));
                        }
                    }
                } else {
                    next_wave.insert(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
            // right
            if trail.position.x < max_x
                && current + 1 == map[trail.position.y][trail.position.x + 1]
            {
                let next = Point {
                    x: trail.position.x + 1,
                    y: trail.position.y,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            v.insert(next);
                        }
                        None => {
                            completed.insert(trail.start, HashSet::from_iter(vec![next]));
                        }
                    }
                } else {
                    next_wave.insert(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
        }
        wave = next_wave;
    }
    completed.values().fold(0, |acc, e| acc + e.len() as u64)
}

pub fn sum_distinct_trails(input: &str) -> u64 {
    let map = utils::parse_to_2d_digit_array(input);
    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;
    let mut wave: Vec<Trail> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if *v == 0 {
                wave.push(Trail {
                    start: Point { x, y },
                    position: Point { x, y },
                });
            }
        }
    }
    let mut completed: HashMap<Point, u64> = HashMap::new();
    for _ in 0..10 {
        let mut next_wave: Vec<Trail> = Vec::new();
        for trail in wave.iter() {
            let current = map[trail.position.y][trail.position.x];
            // up
            if trail.position.y > 0 && current + 1 == map[trail.position.y - 1][trail.position.x] {
                let next = Point {
                    x: trail.position.x,
                    y: trail.position.y - 1,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            *v += 1;
                        }
                        None => {
                            completed.insert(trail.start, 1);
                        }
                    }
                } else {
                    next_wave.push(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
            // down
            if trail.position.y < max_y
                && current + 1 == map[trail.position.y + 1][trail.position.x]
            {
                let next = Point {
                    x: trail.position.x,
                    y: trail.position.y + 1,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            *v += 1;
                        }
                        None => {
                            completed.insert(trail.start, 1);
                        }
                    }
                } else {
                    next_wave.push(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
            // left
            if trail.position.x > 0 && current + 1 == map[trail.position.y][trail.position.x - 1] {
                let next = Point {
                    x: trail.position.x - 1,
                    y: trail.position.y,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            *v += 1;
                        }
                        None => {
                            completed.insert(trail.start, 1);
                        }
                    }
                } else {
                    next_wave.push(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
            // right
            if trail.position.x < max_x
                && current + 1 == map[trail.position.y][trail.position.x + 1]
            {
                let next = Point {
                    x: trail.position.x + 1,
                    y: trail.position.y,
                };
                if current == 8 {
                    match completed.get_mut(&trail.start) {
                        Some(v) => {
                            *v += 1;
                        }
                        None => {
                            completed.insert(trail.start, 1);
                        }
                    }
                } else {
                    next_wave.push(Trail {
                        start: trail.start,
                        position: next,
                    });
                }
            }
        }
        wave = next_wave;
    }
    completed.values().fold(0, |acc, e| acc + *e as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part_1() {
        assert_eq!(sum_trailheads(TEST_STR), 36);
    }

    #[test]
    fn part_2() {
        assert_eq!(sum_distinct_trails(TEST_STR), 81);
    }
}
