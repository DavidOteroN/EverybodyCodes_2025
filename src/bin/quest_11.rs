ec::solution!(11);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let mut cols: Vec<u32> = parse_input(notes);
    // Phase 1: Iterate through the columns comparing each element to the next. If the first
    // element has more ducks than the second, decrement the first by one and increment the second
    // by one.
    const MAX_ITER: usize = 10;
    let mut iter = 0usize;
    loop {
        iter += 1;
        for i in 0..cols.len() - 1 {
            if cols[i] > cols[i + 1] {
                cols[i] -= 1;
                cols[i + 1] += 1;
            }
        }
        if cols.is_sorted() || iter >= MAX_ITER {
            break;
        }
    }

    // Phase 2: Iterate like phase one, but if the first element has less ducks than the second,
    // then decrement the second and increment the first.
    let avg: u32 = cols.iter().sum::<u32>() / cols.len() as u32;
    loop {
        iter += 1;
        for i in 0..cols.len() - 1 {
            if cols[i] < cols[i + 1] {
                cols[i] += 1;
                cols[i + 1] -= 1;
            }
        }
        if cols.iter().all(|&x| x == avg) || iter >= MAX_ITER {
            break;
        }
    }

    // Compute checksum:
    let checksum: u32 = cols
        .iter()
        .enumerate()
        .fold(0u32, |acc, x| acc + (x.0 as u32 + 1) * x.1);
    Some(checksum.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let mut cols: Vec<u32> = parse_input(notes);
    // Same as part one, but without capping the number of iterations. Run thhe algorythm to
    // completion and output the number of iterations it takes to complete.
    let mut iter = 0usize;
    loop {
        iter += 1;
        for i in 0..cols.len() - 1 {
            if cols[i] > cols[i + 1] {
                cols[i] -= 1;
                cols[i + 1] += 1;
            }
        }
        if cols.is_sorted() {
            break;
        }
    }

    let avg: u32 = cols.iter().sum::<u32>() / cols.len() as u32;
    loop {
        iter += 1;
        for i in 0..cols.len() - 1 {
            if cols[i] < cols[i + 1] {
                cols[i] += 1;
                cols[i + 1] -= 1;
            }
        }
        if cols.iter().all(|&x| x == avg) {
            break;
        }
    }

    Some(iter.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // NOTE: The input is already monotonically increassing, so we can skip straight to phase tow.
    // NOTE: The input in this part has large numbers, so they need to be parsed as u64.
    // NOTE: The net result of each round of phase two is to move one single duck from one column
    // that has more ducks than the average to one with less, so the number of iterations will be
    // the sum of the differences of the columns with less than average values to the average.
    // (Sadly I did not come up with this).
    let cols: Vec<usize> = parse_input(notes);
    let avg: usize = cols.iter().sum::<usize>() / cols.len() as usize;
    let iters: usize = cols
        .iter()
        .filter_map(|&x| if x < avg { Some(avg - x) } else { None })
        .sum::<usize>();
    Some(iters.to_string())
}

fn parse_input<T>(notes: &str) -> Vec<T>
where
    T: std::str::FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    notes
        .trim()
        .lines()
        .map(|x| {
            x.parse::<T>().unwrap_or_else(|_| {
                panic!("Cannot parse \"{x}\" to {}.", std::any::type_name::<T>())
            })
        })
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(11, 1));
        assert_eq!(result, Some(109.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(11, 2));
        assert_eq!(result, Some(1579.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(11, 3));
        assert_eq!(result, None);
    }
}
