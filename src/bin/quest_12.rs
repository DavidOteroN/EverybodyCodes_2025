ec::solution!(12);

use std::collections::{HashMap, HashSet};

fn parse_input(notes: &str) -> HashMap<(usize, usize), u8> {
    let mut barrels = HashMap::new();

    // Represent the grid as a HashMap containing the position (x, y) as keys and the number as
    // values.
    notes.trim().lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            barrels.insert((i, j), c.to_digit(10).unwrap() as u8);
        })
    });

    barrels
}

fn burn_barrels(
    barrels: &HashMap<(usize, usize), u8>,
    burnt: &HashSet<(usize, usize)>,
    init: Vec<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    // Create a processing queue. This will be last come, first served.
    let mut queue = init.clone();
    let mut ignited = burnt.clone();

    // Process entries in the queue until it's empty.
    while let Some(k) = queue.pop() {
        if ignited.contains(&k) {
            continue;
        }
        ignited.insert(k);
        // Add the four adjacent elements to the processing queue.
        for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = k.0.wrapping_add_signed(i);
            let new_y = k.1.wrapping_add_signed(j);

            if let Some(val) = barrels.get(&(new_x, new_y))
                && val <= barrels.get(&k).unwrap()
            {
                queue.push((new_x, new_y));
            }
        }
    }

    ignited
}

fn greedy_search(
    barrels: &HashMap<(usize, usize), u8>,
    init: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    // TODO: Add optimizations like skipping low values or starting on local maximums.
    let mut best = init.clone();
    for k in barrels.keys() {
        if init.contains(k) {
            continue;
        }
        let start = vec![*k];
        let _tmp = burn_barrels(barrels, init, start);
        if _tmp.len() > best.len() {
            best = _tmp;
        }
    }

    best
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let barrels = parse_input(notes);
    // The starting state is:
    // - Barrel at (0, 0) burning,
    // - All the rest intact.
    let init: HashSet<(usize, usize)> = HashSet::new();
    let start = vec![(0usize, 0usize)];
    let ignited = burn_barrels(&barrels, &init, start);
    Some(ignited.len().to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // Exactly like part one, but this time, on top of igniting the first barrel, the one on the
    // opposite corner is also ignited.
    let barrels = parse_input(notes);

    // Due to the parser implementation, there's some code needed to get the grid size.
    let (rows, cols) = barrels.keys().max_by_key(|(kx, ky)| kx + ky).unwrap();

    // The starting state is:
    // - Barrel at (0, 0) burning,
    // - Barrel at (M, N) burning (where M is the number of rows and M is the number of columns).
    // - All the rest intact.
    let init: HashSet<(usize, usize)> = HashSet::new();
    let start = vec![(0usize, 0usize), (*rows, *cols)];
    let ignited = burn_barrels(&barrels, &init, start);
    Some(ignited.len().to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // Find three fireballs shuch that:
    // - The first one destroys as many barrels as possible.
    // - The second one destroys as many barrenls as possible after the first one.
    // - The third one destroys as many barrenls as possible after the second one.
    let barrels = parse_input(notes);

    // One option would be to iterate on a starting position and calculate the number of barrels
    // that would be ignited (brute-force), but there should be something better...
    let mut init: HashSet<(usize, usize)> = HashSet::new();
    for _ in 0..3 {
        init = greedy_search(&barrels, &init);
    }

    Some(init.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(12, 1));
        assert_eq!(result, Some(16.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(12, 2));
        assert_eq!(result, Some(58.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(12, 3));
        assert_eq!(result, Some(136.to_string()));
    }
}
