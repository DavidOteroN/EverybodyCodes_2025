ec::solution!(5);
use ec::utils::Fishbone;
use regex::Regex;

fn parse_input(notes: &str) -> (u32, Vec<u32>) {
    let re = Regex::new(r"(?<id>\d+):(?<nums>(?:\d+,?)+)").unwrap();
    let Some(caps) = re.captures(notes) else {
        panic!("No matches.")
    };
    (
        caps["id"].parse::<u32>().unwrap(),
        caps["nums"]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect(),
    )
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let (_, nums) = parse_input(notes);
    let f = Fishbone::build_fishbone(nums);
    Some(f.get_quality().to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut min = u64::MAX;
    let mut max = 0u64;

    for line in notes.lines() {
        let (_, nums) = parse_input(line);
        let q = Fishbone::build_fishbone(nums)
            .get_quality()
            .parse::<u64>()
            .unwrap();
        if q > max {
            max = q;
        }
        if q < min {
            min = q;
        }
    }
    Some(format!("{}", max - min))
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // Build Fishbones for all swords, keeping the IDs.
    // Order by quality, from most to least.
    // If two swords have the same quality, compare the nodes.
    // If after comparing the nodes, the swords are still the same, then order by ID.
    // NOTE: All these rules have been implemented for the type Fishbone in src/utils/fishbone.rs,
    // except for the ID ordering.
    let mut swords: Vec<(u32, Fishbone)> = Vec::new();
    for line in notes.lines() {
        let (id, nums) = parse_input(line);
        swords.push((id, Fishbone::build_fishbone(nums)));
    }
    swords.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.partial_cmp(&b.1).unwrap()
        } else {
            a.0.cmp(&b.0)
        }
    });

    // Compute checksum
    let s = swords
        .iter()
        .rev()
        .enumerate()
        .fold(0usize, |acc, e| acc + (e.0 + 1usize) * e.1.0 as usize);
    Some(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(5, 1));
        assert_eq!(result, Some("581078".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(5, 2));
        assert_eq!(result, Some("77053".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(5, 3));
        assert_eq!(result, Some("260".to_string()));
    }
}
