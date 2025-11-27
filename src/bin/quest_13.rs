ec::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Range {
    init: usize,
    rev: bool,
    len: usize,
}

impl Range {
    fn new(init: usize, end: usize, rev: bool, incl: bool) -> Range {
        Range {
            init,
            rev,
            len: end - init + if incl { 1 } else { 0 },
        }
    }

    fn nth(&self, n: usize) -> Option<usize> {
        if n > self.len - 1 {
            return None;
        }
        if self.rev {
            Some(self.init + self.len - n - 1)
        } else {
            Some(self.init + n)
        }
    }

    fn rev(&self) -> Range {
        let mut out = *self;
        out.rev = true;
        out
    }
}

fn solve<const TURNS: usize>(notes: &str) -> Option<String> {
    // Instead of storing the full array in memory, store an array of ranges.
    let input: Vec<Range> = notes
        .trim()
        .lines()
        .map(|l| {
            let v = l
                .split("-")
                .map(|z| z.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (x, y) = (v[0], v[1]);
            Range::new(x, y, false, true)
        })
        .collect();

    // Then, create two vecs: front and back. Iterate through the input vec and push to front or
    // back alternatively.
    let mut front = vec![Range::new(1, 1, false, true)];
    let mut back = Vec::new();
    for (i, r) in input.iter().enumerate() {
        if i % 2 == 0 {
            front.push(*r);
        } else {
            back.push(r.rev());
        }
    }

    // Finally, join the two vecs, reversing the back one.
    front.extend(back.iter().rev());

    // At last we can get the result.
    let n = front.iter().map(|r| r.len).sum::<usize>();
    let idx = TURNS % n;
    let mut accum: usize = 0;
    let mut result: usize = 0;
    for r in front {
        accum += r.len;
        if accum > idx {
            result = r.nth(idx + r.len - accum).unwrap();
            break;
        }
    }

    Some(result.to_string())
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    const TURNS: usize = 2025;
    // The even indices of the input array are inserted from the front, and the odd ones are
    // inserted from the back. The resulting array should be:
    // [0, 2, 4, ..., 3,, 1];
    // This array will be appended to a [1]
    // That means that, indexing the first N/2 elements will yield the even indices of the original
    // array, while indexing from N/2+1 to N will yield the odd indices in reeerse order.
    let mut numbers = vec![1_usize];
    notes.trim().lines().for_each(|l| {
        numbers.push(l.parse::<usize>().unwrap());
    });
    let mut idx = TURNS % numbers.len();
    if idx == 0 {
        return Some(1.to_string());
    } else if idx <= numbers.len().div_ceil(2) {
        idx = 2 * idx - 1;
    } else {
        idx = numbers.len() - 1 - 2 * (idx - numbers.len().div_ceil(2));
    }
    Some(numbers[idx].to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    solve::<20252025>(notes)
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    solve::<202520252025>(notes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(13, 1));
        assert_eq!(result, Some(67.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(13, 2));
        assert_eq!(result, Some(30.to_string()));
    }
}
