use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn part_1() -> String {
    let config = parse_input();
    let mut result: u32 = 0;
    for row in &config.values {
        if config.validate(&row) {
            let middle = u32::from(row[row.len() / 2]);
            result += middle;
        }
    }
    result.to_string()
}

pub fn part_2() -> String {
    let config = parse_input();
    let mut result: u32 = 0;
    for row in &config.values {
        if !config.validate(&row) {
            let mut sorted_row = row.to_owned();
            // This sort works because of an implicit assumption that every number has a corresponding rule
            config.sort(&mut sorted_row);
            let middle = u32::from(sorted_row[sorted_row.len() / 2]);
            result += middle;
        }
    }
    result.to_string()
}

struct Input {
    rules: HashMap<u8, Vec<u8>>,
    values: Vec<Vec<u8>>,
}

impl Input {
    fn sort(&self, values: &mut Vec<u8>) {
        let mut sorting_rule: HashMap<u8, HashSet<u8>> = HashMap::new();
        for (key, rule) in (&self).rules.iter() {
            sorting_rule.insert(*key, HashSet::from_iter(rule.clone()));
        }
        values.sort_by(|a, b| {
            if let Some(rule) = sorting_rule.get(a) {
                if rule.contains(b) {
                    return cmp::Ordering::Less;
                }
            }
            if let Some(rule) = sorting_rule.get(b) {
                if rule.contains(a) {
                    return cmp::Ordering::Greater;
                }
            }
            cmp::Ordering::Equal
        });
    }

    fn filtered_rules(&self, values: &Vec<u8>) -> HashMap<u8, Vec<u8>> {
        let all_values: HashSet<u8> = HashSet::from_iter(values.clone());
        let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
        for (key, rule) in (&self).rules.iter() {
            let filtered_rule: Vec<u8> = rule
                .into_iter()
                .filter(|v| all_values.contains(v))
                .map(|v| *v)
                .collect::<Vec<u8>>();
            if filtered_rule.len() > 0 {
                rules.insert(*key, filtered_rule);
            }
        }
        rules
    }

    fn validate(&self, values: &Vec<u8>) -> bool {
        let rules = &self.filtered_rules(values);
        let mut seen: HashSet<u8> = HashSet::new();
        for v in values {
            if let Some(rule) = rules.get(v) {
                for required in rule {
                    if let None = seen.get(required) {
                        return false;
                    }
                }
            }
            seen.insert(*v);
        }
        true
    }
}

fn parse_input() -> Input {
    let config: Vec<String> = include_str!("input.txt")
        .trim_end()
        .split("\n\n")
        .map(|v| v.to_string())
        .collect();
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    for pair in config[0]
        .split("\n")
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
    {
        let parsed_pair: Vec<u8> = pair.split("|").map(|v| v.parse::<u8>().unwrap()).collect();
        match rules.get_mut(&parsed_pair[1]) {
            Some(v) => {
                v.push(parsed_pair[0]);
            }
            None => {
                rules.insert(parsed_pair[1], vec![parsed_pair[0]]);
            }
        }
    }
    let mut values: Vec<Vec<u8>> = Vec::new();
    for row in config[1]
        .split("\n")
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
    {
        values.push(row.split(",").map(|v| v.parse::<u8>().unwrap()).collect())
    }

    Input { rules, values }
}
