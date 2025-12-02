ec::solution!(14);
use std::hash::{DefaultHasher, Hash, Hasher};

fn parse_input(notes: &str) -> (Vec<u64>, u8) {
    let mut out: Vec<u64> = Vec::new();
    let mut width: u8 = 0;

    for line in notes.trim().lines() {
        let mut x: u64 = 0;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                x |= 1 << j
            }
            width = width.max(j as u8);
        }
        out.push(x);
    }
    (out, width + 1)
}

fn step(grid: &[u64], width: u8) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();

    // Generate a bit mask containing as many ones as `width` on the least significant bits.
    let mask: u64 = 2_u64.pow(width as u32) - 1;

    for i in 0..grid.len() {
        let mut next = grid[i];

        // If a value is true, it will remain true only if the numver ov true values on the corners
        // is odd. If it false, it will become true if the number of true values on the corners is
        // even.
        // That means that the bit is flipped as many times as true value son the corners plus one.
        next ^= !0; // first flip.
        if i > 0 {
            next ^= grid[i - 1] >> 1; // second flip.
            next ^= grid[i - 1] << 1; // third flip.
        }
        if i < grid.len() - 1 {
            next ^= grid[i + 1] >> 1; // fourth flip.
            next ^= grid[i + 1] << 1; // fifth flip.
        }
        out.push(next & mask);
    }

    out
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    const ROUNDS: usize = 10;
    let (grid, width) = parse_input(notes);

    let mut next = grid.clone();
    let mut count: usize = 0;
    for _ in 0..ROUNDS {
        next = step(&next, width);
        count += next.iter().map(|x| x.count_ones() as usize).sum::<usize>();
    }

    Some(count.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    const ROUNDS: usize = 2025;
    let (grid, width) = parse_input(notes);

    let mut next = grid.clone();
    let mut count: usize = 0;
    for _ in 0..ROUNDS {
        next = step(&next, width);
        count += next.iter().map(|x| x.count_ones() as usize).sum::<usize>();
    }

    Some(count.to_string())
}

fn compute_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    const ROUNDS: usize = 1000000000;
    // The grid for this part starts on all 0s, and it's 34x34.
    let width: u8 = 34;
    let grid = vec![0u64; width as usize];

    // Parse input to a smaller grid
    let (mut pattern, pattern_width) = parse_input(notes);

    // We want the pattern to be centered, so it has to be shifted right 13 places (the big grid is
    // 34x34, the small grid is 8x8, so to get the data to the center, the shift is (34 - 8) / 2).
    let shift = ((width - pattern_width) / 2) as u64;
    pattern.iter_mut().for_each(|x| *x <<= shift);

    // Create a bit mask to only compare the 8 bits in the center.
    // 0b0000000000000111111110000000000000
    let mask: u64 = 0xFF << shift;

    // The goal is to detect when the center of the grid matches the input notes for this part, and
    // sum all the ones each time that happens.
    // NOTE: running all those rounds will take ages. There are cycles in the pattern, so stop when
    // a cycle is detected and multiply the answher by ROUNDS / cycle length.
    // NOTE: Cycle probably does not start on the first round, so compute batches from current
    // iteration backwards.
    let mut next_grid = grid.clone();
    let mut matches: Vec<(usize, u64)> = Vec::new();
    let mut hashes: Vec<u64> = Vec::new();
    let mut cycle_start: usize = 0;
    let mut cycle_end: usize = 0;
    hashes.push(compute_hash(&next_grid));
    for round in 0..ROUNDS {
        // To detect the cycle, compute the hash of the first grid, and then compare the hash for
        // each iteration. Stop when the hashes match.
        next_grid = step(&next_grid, width);

        let found_match = next_grid[shift as usize..(shift + pattern_width as u64) as usize]
            .iter()
            .zip(&pattern)
            .all(|(&x, &y)| (x & mask) == y);

        if found_match {
            println!("fount match at iter {round}");
            matches.push((
                round,
                next_grid
                    .iter()
                    .map(|&x| x.count_ones() as u64)
                    .sum::<u64>(),
            ));
        }

        let next_hash = compute_hash(&next_grid);
        if let Some(idx) = hashes.iter().position(|&x| x == next_hash) {
            cycle_start = idx;
            cycle_end = round;
            break;
        } else {
            hashes.push(next_hash);
        }
    }

    println!("cycle_start = {cycle_start}; cycle_end = {cycle_end}");

    // Compute checksum:
    let pre_cycle: u64 = matches
        .iter()
        .filter_map(|(i, r)| if i < &cycle_start { Some(*r) } else { None })
        .sum();

    let cycle_sum: u64 = matches
        .iter()
        .filter_map(|(i, r)| if i >= &cycle_start { Some(*r) } else { None })
        .sum();

    let num_cycles = (ROUNDS - cycle_start - 1) / cycle_end;
    println!("num_cycles = {num_cycles}");
    println!("cycle_sum = {cycle_sum}");

    let remaining: u64 = matches
        .iter()
        .filter_map(|(i, r)| {
            if *i > cycle_start && *i < (ROUNDS - cycle_start - 1) % cycle_end {
                Some(*r)
            } else {
                None
            }
        })
        .sum();

    let result = pre_cycle + cycle_sum * num_cycles as u64 + remaining;

    // NOTE: cycle starts at the second round, and the first match dows NOT happen before that, so
    // the cycle sum is the same as the sum of the whole `count` array.

    Some(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(14, 1));
        assert_eq!(result, Some(200.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(14, 3));
        assert_eq!(result, Some(278388552.to_string()));
    }
}
