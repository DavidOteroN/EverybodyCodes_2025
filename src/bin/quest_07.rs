ec::solution!(7);

use std::collections::{HashMap, HashSet};

// Type alias to make code less verbose:
type RuleSet = HashMap<char, Vec<char>>;

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (names, rules) = parse_input(notes).unwrap();
    for name in names {
        if check_rule(name, &rules) {
            return Some(name.to_string());
        }
    }
    None
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let (names, rules) = parse_input(notes).unwrap();
    let mut total = 0usize;
    for (idx, name) in names.iter().enumerate() {
        if check_rule(name, &rules) {
            total += idx + 1; // NOTE: +1 because positions are one-based.
        }
    }
    Some(total.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // In this part, we have to expand a set of given name prefixes to generate all possible names
    // from 7 to 11 letters (inclusive). The output is the number of unique names.
    // NOTE: Some prefixes from the input may not be compatible with the given rules!!
    let (names, rules) = parse_input(notes).unwrap();
    let mut name_set = HashSet::new();
    for name in names.iter() {
        if check_rule(name, &rules) {
            expand_name(name, &rules, &mut name_set);
        }
    }
    Some(name_set.len().to_string())
}

fn parse_input(notes: &str) -> Option<(Vec<&str>, RuleSet)> {
    let mut rules = RuleSet::new();

    match notes.trim().split("\n\n").collect::<Vec<&str>>()[..] {
        [x, y] => {
            let names: Vec<&str> = x.split(",").collect();
            for line in y.lines() {
                let (left, right) = process_rule(line).unwrap();
                rules
                    .entry(left)
                    .and_modify(|e| {
                        for x in &right {
                            e.push(*x);
                        }
                    })
                    .or_insert(right);
            }
            Some((names, rules))
        }
        _ => None,
    }
}

fn process_rule(line: &str) -> Option<(char, Vec<char>)> {
    match line.split(" > ").collect::<Vec<&str>>()[..] {
        [left, right] => {
            let left: char = left.as_bytes()[0] as char;
            let right: Vec<char> = right.chars().filter(|c| *c != ',').collect();
            Some((left, right))
        }
        _ => None,
    }
}

fn check_rule(name: &str, rules: &RuleSet) -> bool {
    for idx in 0..name.len() - 1 {
        let curr = name.chars().nth(idx).unwrap();
        let next = name.chars().nth(idx + 1).unwrap();
        let Some(rule) = rules.get(&curr) else {
            continue;
        };
        if !rule.contains(&next) {
            return false;
        }
    }
    true
}

fn expand_name<'a>(name: &'a str, rules: &'a RuleSet, name_set: &'a mut HashSet<String>) {
    // Recursive function to expand names based on rules.
    const MAX_LEN: usize = 11;
    const MIN_LEN: usize = 7;

    if name.len() >= MAX_LEN {
        return;
    }

    let Some(next_letter) = rules.get(&name.chars().last().unwrap()) else {
        return;
    };

    for c in next_letter {
        let mut new_name = name.chars().collect::<Vec<char>>();
        new_name.push(*c);
        let new_name: String = new_name.iter().collect::<String>();
        if new_name.len() >= MIN_LEN {
            // Only insert the new name if it is longer than MIN_LEN. Else, only expand.
            name_set.insert(new_name.to_string());
        }
        expand_name(&new_name[..], rules, name_set);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(7, 1));
        assert_eq!(result, Some("Oroneth".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(7, 2));
        assert_eq!(result, Some(23.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(7, 3));
        assert_eq!(result, Some(1154.to_string()));
    }
}
