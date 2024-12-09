use core::str;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    point: Point,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Maze {
    wall_x: HashMap<usize, Vec<usize>>,
    wall_y: HashMap<usize, Vec<usize>>,
    maze: Vec<Vec<char>>,
    start: Position,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMazeError;

impl str::FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maze: Vec<Vec<char>> = s
            .trim_end()
            .split("\n")
            .map(|v| v.chars().collect())
            .collect();
        let mut wall_x: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut wall_y: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut start = Position {
            point: Point { x: 0, y: 0 },
            direction: Direction::Up,
        };
        for (y, row) in maze.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    if let Some(arr) = wall_y.get_mut(&x) {
                        arr.push(y);
                    }
                    if let Some(arr) = wall_x.get_mut(&y) {
                        arr.push(x);
                    }
                } else if *c == '^' {
                    start.point.x = x;
                    start.point.y = y;
                }
            }
        }
        Ok(Maze {
            wall_x,
            wall_y,
            start,
            maze,
        })
    }
}

impl Maze {
    fn check_loop(mut maze: Maze, block: Position) -> bool {
        // set possible block
        match block.direction {
            Direction::Up => {
                maze.maze[block.point.y - 1][block.point.x] = '#';
            }
            Direction::Down => {
                maze.maze[block.point.y + 1][block.point.x] = '#';
            }
            Direction::Left => {
                maze.maze[block.point.y][block.point.x - 1] = '#';
            }
            Direction::Right => {
                maze.maze[block.point.y][block.point.x + 1] = '#';
            }
        }
        let mut visited: HashSet<Position> = HashSet::new();
        let mut position = maze.start.to_owned();
        visited.insert(position);
        let max_y = maze.maze.len() - 1;
        let max_x = maze.maze[0].len() - 1;
        loop {
            match position.direction {
                Direction::Up => {
                    if position.point.y == 0 {
                        break;
                    }
                    if maze.maze[position.point.y - 1][position.point.x] == '#' {
                        position.direction = Direction::Right;
                    } else {
                        position.point.y -= 1;
                        if visited.contains(&position) {
                            return true;
                        }
                        visited.insert(position.to_owned());
                    }
                }
                Direction::Down => {
                    if position.point.y == max_y {
                        break;
                    }
                    if maze.maze[position.point.y + 1][position.point.x] == '#' {
                        position.direction = Direction::Left;
                    } else {
                        position.point.y += 1;
                        if visited.contains(&position) {
                            return true;
                        }
                        visited.insert(position.to_owned());
                    }
                }
                Direction::Left => {
                    if position.point.x == 0 {
                        break;
                    }
                    if maze.maze[position.point.y][position.point.x - 1] == '#' {
                        position.direction = Direction::Up;
                    } else {
                        position.point.x -= 1;
                        if visited.contains(&position) {
                            return true;
                        }
                        visited.insert(position.to_owned());
                    }
                }
                Direction::Right => {
                    if position.point.x == max_x {
                        break;
                    }
                    if maze.maze[position.point.y][position.point.x + 1] == '#' {
                        position.direction = Direction::Down;
                    } else {
                        position.point.x += 1;
                        if visited.contains(&position) {
                            return true;
                        }
                        visited.insert(position.to_owned());
                    }
                }
            }
        }
        false
    }
}

pub fn part_1() -> String {
    let maze = Maze::from_str(include_str!("input.txt").trim_end()).unwrap();
    let mut position = maze.start.to_owned();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(position.point.to_owned());
    let max_y = maze.maze.len() - 1;
    let max_x = maze.maze[0].len() - 1;
    loop {
        match position.direction {
            Direction::Up => {
                if position.point.y == 0 {
                    break;
                }
                if maze.maze[position.point.y - 1][position.point.x] == '#' {
                    position.direction = Direction::Right;
                } else {
                    position.point.y -= 1;
                    visited.insert(position.point.to_owned());
                }
            }
            Direction::Down => {
                if position.point.y == max_y {
                    break;
                }
                if maze.maze[position.point.y + 1][position.point.x] == '#' {
                    position.direction = Direction::Left;
                } else {
                    position.point.y += 1;
                    visited.insert(position.point.to_owned());
                }
            }
            Direction::Left => {
                if position.point.x == 0 {
                    break;
                }
                if maze.maze[position.point.y][position.point.x - 1] == '#' {
                    position.direction = Direction::Up;
                } else {
                    position.point.x -= 1;
                    visited.insert(position.point.to_owned());
                }
            }
            Direction::Right => {
                if position.point.x == max_x {
                    break;
                }
                if maze.maze[position.point.y][position.point.x + 1] == '#' {
                    position.direction = Direction::Down;
                } else {
                    position.point.x += 1;
                    visited.insert(position.point.to_owned());
                }
            }
        }
    }
    visited.len().to_string()
}

pub fn part_2() -> String {
    let maze = Maze::from_str(include_str!("input.txt").trim_end()).unwrap();
    let mut position = maze.start.to_owned();
    let mut block_positions: HashSet<Point> = HashSet::new();
    let max_y = maze.maze.len() - 1;
    let max_x = maze.maze[0].len() - 1;
    loop {
        match position.direction {
            Direction::Up => {
                if position.point.y == 0 {
                    break;
                }
                if maze.maze[position.point.y - 1][position.point.x] == '#' {
                    position.direction = Direction::Right;
                } else {
                    if Maze::check_loop(maze.to_owned(), position.to_owned()) {
                        let mut block = position.point.to_owned();
                        block.y -= 1;
                        block_positions.insert(block);
                    }
                    position.point.y -= 1;
                }
            }
            Direction::Down => {
                if position.point.y == max_y {
                    break;
                }
                if maze.maze[position.point.y + 1][position.point.x] == '#' {
                    position.direction = Direction::Left;
                } else {
                    if Maze::check_loop(maze.to_owned(), position.to_owned()) {
                        let mut block = position.point.to_owned();
                        block.y += 1;
                        block_positions.insert(block);
                    }
                    position.point.y += 1;
                }
            }
            Direction::Left => {
                if position.point.x == 0 {
                    break;
                }
                if maze.maze[position.point.y][position.point.x - 1] == '#' {
                    position.direction = Direction::Up;
                } else {
                    if Maze::check_loop(maze.to_owned(), position.to_owned()) {
                        let mut block = position.point.to_owned();
                        block.x -= 1;
                        block_positions.insert(block);
                    }
                    position.point.x -= 1;
                }
            }
            Direction::Right => {
                if position.point.x == max_x {
                    break;
                }
                if maze.maze[position.point.y][position.point.x + 1] == '#' {
                    position.direction = Direction::Down;
                } else {
                    if Maze::check_loop(maze.to_owned(), position.to_owned()) {
                        let mut block = position.point.to_owned();
                        block.x += 1;
                        block_positions.insert(block);
                    }
                    position.point.x += 1;
                }
            }
        }
    }
    block_positions.len().to_string()
}
