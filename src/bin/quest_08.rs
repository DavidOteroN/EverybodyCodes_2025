ec::solution!(8);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    // The string will pass though the center of the circle whenever the two consecutive numbers
    // differ in exactly half of the number of nails.
    let nums = parse_input(notes);
    let num_nails = if cfg!(test) { 8usize } else { 32usize };
    let mut counter = 0usize;
    for window in nums.windows(2) {
        let (curr, next) = match window {
            [x, y] => Some((x, y)),
            _ => None,
        }
        .unwrap();
        let diff = curr.abs_diff(*next);
        if diff as usize == num_nails / 2 {
            counter += 1
        }
    }
    Some(counter.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    let nums = parse_input(notes);

    // Cross detection: Given segments A and B, segment A divides the circle in two regions. If
    // both ends of B are contained in the same region, then there's no crossing. If B's start and
    // end point are contained in different regions, then there's a crossing.
    //
    // NOTE: when two segments have a common endpoint, they do not cross.
    // Two consecutive segments cannot cross.

    // Check each segment against all previous segments.
    let mut counter = 0usize;
    for i in 2..nums.len() - 1 {
        let a = nums[i].min(nums[i + 1]);
        let b = nums[i].max(nums[i + 1]);
        let region = a..=b;
        for j in 0..i {
            // Skip if one of the endpoints coincides.
            if nums[j] == nums[i]
                || nums[j] == nums[i + 1]
                || nums[j + 1] == nums[i]
                || nums[j + 1] == nums[i + 1]
            {
                continue;
            }
            if region.contains(&nums[j]) ^ region.contains(&nums[j + 1]) {
                // NOTE: rhs XOR lhs will only be TRUE if rhs and lhs are different.
                counter += 1;
            }
        }
    }
    Some(counter.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let nums = parse_input(notes);
    let num_nails = if cfg!(test) { 8u32 } else { 256u32 };

    // Quite similar to part 2, but instead of comparing one segment to another form the imput, we
    // have to compare a cut iterating from first to last nail against all segments of the input.
    let mut max = 0usize;
    // NOTE: Important! Indices in the input vec are BASE 1!!
    for i in 1..num_nails {
        for j in i + 1..num_nails + 1 {
            let region = i..=j;
            let mut counter = 0usize;
            'inner: for k in 0..nums.len() - 1 {
                // NOTE: In this case, if a thread is aligned with a cut, then it's also cut.
                if i == nums[k].min(nums[k + 1]) && j == nums[k].max(nums[k + 1]) {
                    counter += 1;
                } else if i == nums[k] || i == nums[k + 1] || j == nums[k] || j == nums[k + 1] {
                    continue 'inner;
                } else if region.contains(&nums[k]) ^ region.contains(&nums[k + 1]) {
                    counter += 1;
                }
            }
            if counter > max {
                max = counter;
            };
        }
    }
    Some(max.to_string())
}

fn parse_input(notes: &str) -> Vec<u32> {
    notes
        .trim()
        .split(",")
        .map(|u| u.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(8, 1));
        assert_eq!(result, Some(4.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(8, 2));
        assert_eq!(result, Some(21.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(8, 3));
        assert_eq!(result, Some(7.to_string()));
    }
}
