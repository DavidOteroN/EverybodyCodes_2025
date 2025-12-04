ec::solution!(16);

fn parse_input(notes: &str) -> impl Iterator<Item = u64> {
    notes.trim().split(",").map(|x| x.parse::<u64>().unwrap())
}

fn compute_blocks(spell: impl Iterator<Item = u64>, num_cols: u64) -> u64 {
    spell.fold(0, |acc, p| acc + num_cols / p)
}

fn compute_spell(columns: impl Iterator<Item = u64>) -> Vec<u64> {
    let mut columns: Vec<u64> = columns.collect();
    let mut spell: Vec<u64> = Vec::new();

    for i in 0..columns.len() {
        if columns[i] > 0 {
            spell.push(i as u64 + 1);
            for j in (i..columns.len()).step_by(i + 1) {
                columns[j] -= 1;
            }
        }
    }

    spell
}

#[allow(unused_variables, clippy::needless_range_loop)]
pub fn part_one(notes: &str) -> Option<String> {
    const NUM_COLS: u64 = 90;
    let total = compute_blocks(parse_input(notes), NUM_COLS);
    Some(total.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // Reverse computation to part_one: given the number of bricks in each column, compute the
    // numbers that generated the wall.
    let spell = compute_spell(parse_input(notes));

    Some(spell.iter().product::<u64>().to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    const N: u64 = 202520252025000;
    let spell = compute_spell(parse_input(notes));

    // Calculate an initial estimate using floating point arithmetic for the number of columns (opposite procedure to part 1).
    let mut n_cols = (N as f64 / spell.iter().fold(0f64, |acc, &p| acc + 1f64 / (p as f64))) as u64;

    // NOTE: it can be mathematically proven that the value calculated above is a LOWER BOUND for
    // the real solution (solution must be >= than n_cols).
    //
    // Iterate increasing n_cols until a solution is reached (no need for binary search).
    loop {
        if compute_blocks(spell.iter().copied(), n_cols) > N {
            // We've gone one steop too far.
            n_cols -= 1;
            break;
        } else {
            n_cols += 1;
        }
    }

    Some(n_cols.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(16, 1));
        assert_eq!(result, Some("193".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(16, 2));
        assert_eq!(result, Some("270".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(16, 3));
        assert_eq!(result, Some("94439495762954".to_string()));
    }
}
