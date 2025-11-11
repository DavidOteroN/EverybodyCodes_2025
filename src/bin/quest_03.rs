ec::solution!(3);
use std::collections::{HashMap, HashSet};

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let nums = parse_notes(notes);
    let nums_set: HashSet<u32> = nums.into_iter().collect();
    let set_size = nums_set.iter().sum::<u32>();
    Some(format!("{}", set_size))
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let nums = parse_notes(notes);

    // This time, we have to select the 20 smallest numbers in the set.
    let nums_set: HashSet<u32> = nums.into_iter().collect();
    let mut hash_vec: Vec<u32> = nums_set.into_iter().collect();
    hash_vec.sort();
    Some(format!("{}", hash_vec[0..20].iter().sum::<u32>()))
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // For part 3, the duplicate entries must be included in new sets, such that all elements are
    // in sets. The output will be the number of sets that will be required.
    // Naive approach: Iteratively create sets as neede.
    // Better approach: There will be as many sets as the maximum ammount of repetitions of a
    // single element. Create a hash map instead, counting the occurances of each element.
    let nums = parse_notes(notes);
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for n in nums {
        match counter.get(&n) {
            Some(count) => counter.insert(n, count + 1),
            None => counter.insert(n, 1),
        };
    }
    let m = counter
        .iter()
        .fold(0u32, |acc, e| if *e.1 > acc { *e.1 } else { acc });
    Some(format!("{}", m))
}

fn parse_notes(notes: &str) -> Vec<u32> {
    notes
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(3, 1));
        assert_eq!(result, Some("29".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(3, 2));
        assert_eq!(result, Some("781".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_two(&read_example_file(3, 3));
        assert_eq!(result, Some("3".to_string()));
    }
}
