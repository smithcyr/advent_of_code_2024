use std::str::FromStr;

pub fn part_1() -> String {
    let solutions: Vec<Equation> = include_str!("input.txt")
        .trim_end()
        .split("\n")
        .map(|s| Equation::from_str(s).unwrap())
        .filter(|e| e.has_solution(vec![Operator::Add, Operator::Multiply]))
        .collect();
    let mut result = 0;
    for solution in solutions {
        result += solution.result;
    }
    result.to_string()
}

pub fn part_2() -> String {
    let solutions: Vec<Equation> = include_str!("input.txt")
        .trim_end()
        .split("\n")
        .map(|s| Equation::from_str(s).unwrap())
        .filter(|e| e.has_solution(vec![Operator::Add, Operator::Multiply, Operator::Concat]))
        .collect();
    let mut result = 0;
    for solution in solutions {
        result += solution.result;
    }
    result.to_string()
}

struct Equation {
    result: u128,
    values: Vec<u128>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(": ").collect();
        let result = parts[0].parse::<u128>().map_err(|_| ParseEquationError)?;
        let values = parts[1]
            .split(" ")
            .map(|v| v.parse::<u128>().unwrap())
            .collect();
        Ok(Equation { result, values })
    }
}

enum Operator {
    Add,
    Multiply,
    Concat,
}

fn get_digit_at(available_operators: &Vec<Operator>, num: u128, position: usize) -> &Operator {
    let mut v = num;
    let num_operators = available_operators.len() as u128;
    for _ in 0..position {
        v /= num_operators;
    }
    &available_operators[(v % num_operators) as usize]
}

impl Equation {
    fn has_solution(&self, available_operators: Vec<Operator>) -> bool {
        let mut operators: u128 = 0;
        let num_operators = (&self).values.len() - 1;
        let mask: u128 = (available_operators.len() as u128).pow(num_operators as u32);
        'main: while operators < mask {
            let mut intermediate = (&self).values[0];
            for operator_num in 0..num_operators {
                match get_digit_at(&available_operators, operators, operator_num) {
                    Operator::Add => {
                        intermediate += (&self).values[operator_num + 1];
                    }
                    Operator::Multiply => {
                        intermediate *= (&self).values[operator_num + 1];
                    }
                    Operator::Concat => {
                        let right = (&self).values[operator_num + 1];
                        let mut mag = right;
                        while mag > 0 {
                            mag /= 10;
                            intermediate *= 10;
                            if intermediate > (&self).result {
                                operators += 1;
                                continue 'main;
                            }
                        }
                        intermediate += right;
                    }
                }
                if intermediate > (&self).result {
                    operators += 1;
                    continue 'main;
                }
            }
            if intermediate == (&self).result {
                return true;
            }
            operators += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    ";
    #[test]
    fn part_1() {
        let solutions: Vec<Equation> = TEST_STR
            .trim_end()
            .split("\n")
            .map(|s| Equation::from_str(s).unwrap())
            .filter(|e| e.has_solution(vec![Operator::Add, Operator::Multiply]))
            .collect();
        let mut result = 0;
        for solution in solutions {
            result += solution.result;
        }

        assert_eq!(result, 3749);
    }

    #[test]
    fn part_2() {
        let solutions: Vec<Equation> = TEST_STR
            .trim_end()
            .split("\n")
            .map(|s| Equation::from_str(s).unwrap())
            .filter(|e| e.has_solution(vec![Operator::Add, Operator::Multiply, Operator::Concat]))
            .collect();
        let mut result = 0;
        for solution in solutions {
            result += solution.result;
        }

        assert_eq!(result, 11387);
    }
}
