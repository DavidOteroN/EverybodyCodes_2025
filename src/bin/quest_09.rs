// https://everybody.codes/event/2025/quests/9

use std::collections::HashSet;

ec::solution!(9);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    // The DNA is formed by a sequence of 'A', 'T', 'C', and 'G'. The child's DNA comes from either
    // one of the parents, which means that, it the two parents have the same letter at a given
    // possition, then the child must have the same letter at that position.
    //
    // First, the child must be identified.
    let samples = parse_input(notes);
    let (a, b, c) = (samples[0], samples[1], samples[2]);
    let mut can_be_child = (true, true, true);
    for (x, (y, z)) in a.1.bytes().zip(b.1.bytes().zip(c.1.bytes())) {
        // Test each possible combination of parents and child.
        can_be_child.0 = can_be_child.0 && if y == z { x == y } else { true };
        can_be_child.1 = can_be_child.1 && if x == z { x == y } else { true };
        can_be_child.2 = can_be_child.2 && if x == y { y == z } else { true };
    }
    let child_id: usize = match can_be_child {
        (true, false, false) => 0,
        (false, true, false) => 1,
        (false, false, true) => 2,
        _ => panic!("Genetic dissaster!"),
    };

    // Then, compute the similarity of the child to the parents and multiply their scores.
    let mut score = 1u32;
    for dna in &samples {
        if dna.0 != samples[child_id].0 {
            score *= compute_similarity(samples[child_id].1, dna.1);
        }
    }
    Some(score.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // There are multiple sets of parents and multiple childre. Each children can only be traced to
    // a set of parents.
    let samples = parse_input(notes);
    let mut score = 0usize;
    // NOTE: O(n3) implementation. This could take a while.
    'child: for i in 0..samples.len() {
        for j in 0..samples.len() {
            for k in j + 1..samples.len() {
                if j == i || k == i {
                    continue;
                }
                let c = samples[i].1; // test child
                let p1 = samples[j].1; // test parent 1
                let p2 = samples[k].1; // test parent 2
                if is_child(c, p1, p2) {
                    score +=
                        compute_similarity(c, p1) as usize * compute_similarity(c, p2) as usize;
                    // Child found. Continue to next child test.
                    continue 'child;
                }
            }
        }
    }
    Some(score.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // Find the largest family and output the sum of their IDs.
    let samples = parse_input(notes);

    // Build a sort of tree by storing two hash maps: parents and children. Use IDs as keys, and
    // store the IDs of corresponding children or parents as values.
    let mut families: Vec<HashSet<u32>> = Vec::with_capacity(samples.len());
    'child: for i in 0..samples.len() {
        'parent_1: for j in 0..samples.len() {
            if j == i {
                continue 'parent_1;
            }
            'parent_2: for k in j + 1..samples.len() {
                if k == i {
                    continue 'parent_2;
                }
                let c = samples[i].1; // test child
                let p1 = samples[j].1; // test parent 1
                let p2 = samples[k].1; // test parent 2
                if is_child(c, p1, p2) {
                    let id_c = samples[i].0;
                    let id_p1 = samples[j].0;
                    let id_p2 = samples[k].0;
                    families.push(HashSet::from([id_c, id_p1, id_p2]));
                    // Child found. Continue to next child test.
                    continue 'child;
                }
            }
        }
    }

    // Iterate through the families and join if they have members in common.
    for i in (0..families.len()).rev() {
        for j in 0..i {
            if !families[i]
                .intersection(&families[j])
                .collect::<HashSet<&u32>>()
                .is_empty()
            {
                let other = families[i].clone();
                families[j].extend(other.iter());
                families.swap_remove(i);
                break;
            }
        }
    }

    // Get the size of the biggest family (sum of their IDs):
    let _len = families.iter().map(|x| x.len()).max().unwrap();
    let _idx = families.iter().position(|x| x.len() == _len).unwrap();
    let biggest: u32 = families[_idx].iter().sum();
    Some(biggest.to_string())
}

fn parse_input(notes: &str) -> Vec<(u32, &str)> {
    let mut out: Vec<(u32, &str)> = Vec::new();
    for line in notes.trim().lines() {
        out.push(parse_line(line).unwrap());
    }
    out
}

fn parse_line(line: &str) -> Option<(u32, &str)> {
    match line.trim().split(":").collect::<Vec<&str>>()[..] {
        [first, last] => Some((first.parse::<u32>().unwrap(), last)),
        _ => None,
    }
}

fn compute_similarity(dna1: &str, dna2: &str) -> u32 {
    dna1.bytes()
        .zip(dna2.bytes())
        .filter(|x| x.0 == x.1)
        .count() as u32
}

fn is_child(c: &str, p1: &str, p2: &str) -> bool {
    for (x, (y, z)) in c.bytes().zip(p1.bytes().zip(p2.bytes())) {
        // Naive approach: each character in the DNA string must come from either one of the
        // parents.
        if x != y && x != z {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(9, 1));
        assert_eq!(result, Some(414.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(9, 2));
        assert_eq!(result, Some(1245.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(9, 3));
        assert_eq!(result, Some(36.to_string()));
    }
}
