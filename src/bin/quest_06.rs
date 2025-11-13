ec::solution!(6);

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    // Given a string of 'A', 'a', 'B', 'b', 'C', 'c' in some order:
    // For each upper case letter, find the number of corresponding lower-case letters to its
    // right.
    // For example: in the string "ABabACacBCbca", there are 3 'a' after the first 'A', and 2 'a'
    // after the second.
    // Sum the numbers obtained for the 'Aa' to get the answer.
    let mut counter = 0usize;
    let mut i = 0usize;
    while let Some(j) = notes[i..].find('A') {
        i += j + 1;
        counter += notes[i..].chars().filter(|c| *c == 'a').count();
    }
    Some(counter.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // Same as part one, but match 'A' to 'a', 'B' to 'b' and 'C' to 'c'.
    // Could copy the same loop two more times, but probably there's a more elegant, "rustic"
    // solution.
    let mut counter = 0usize;
    let mut notes_copy = notes.to_string();
    while let Some(c) = notes_copy.pop() {
        let x = match c {
            'a' => 'A',
            'b' => 'B',
            'c' => 'C',
            _ => {
                continue;
            }
        };
        counter += notes_copy.chars().filter(|c| *c == x).count();
    }
    Some(counter.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let notes = notes.trim().as_bytes();
    // The input string is to be repeated 1000 times.
    // Each char of 'abc' can be paired with the corresponding one in 'ABC' within 1000 places.
    const MAX_DIST: i32 = 1000;
    const REPEATS: usize = 1000;
    let mut counter: usize = 0;
    for (i, &b) in notes.iter().enumerate() {
        if b.is_ascii_lowercase() {
            let upper = b.to_ascii_uppercase();
            for j in -MAX_DIST..MAX_DIST + 1 {
                let offset = i as i32 + j;

                // NOTE: repeating the input N times is the same as counting the occurances by
                // letting the index wrap around and then multipying by N, but the result has to be
                // adjusted to take into account the ends, where the index does not wrap.
                let wrapped: usize = offset.rem_euclid(notes.len() as i32) as usize;
                if notes[wrapped] == upper {
                    // NOTE: Each match will add N repeats, except where offset is outside the
                    // array.
                    let adjust = if offset < 0 {
                        notes.len() as i32 - 1 - offset
                    } else {
                        offset
                    };
                    counter += REPEATS - (adjust as usize / notes.len());
                }
            }
        }
    }
    Some(counter.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(6, 1));
        assert_eq!(result, Some("5".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(6, 2));
        assert_eq!(result, Some("11".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(6, 3));
        assert_eq!(result, Some(3442321.to_string()));
    }
}
