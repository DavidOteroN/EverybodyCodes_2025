use std::ops::{AddAssign, BitOr, BitOrAssign, Shl, Shr};

ec::solution!(10);

fn parse_input(notes: &str) -> Vec<Vec<char>> {
    notes
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

// NOTE: using generics because in part two the input does not fit in a usize, so a u128 must be
// used, and I'd like to avoid using such a big int in part one since it's not needed.

/// I feel this bit of code needs some explaination for my future self:
/// This function takes a slice of bit masks (generic unsigned integers) that represent the status
/// of the grid, each element for a row, with each 1 representing a possible dragon location.
/// It produces a new slice of bitmasks with a 1 in each position where a dragon could be in the
/// next move.
/// For example, iven the starting position:
///
/// 00000000
/// 00000000
/// 00010000
/// 00000000
/// 00000000
///
/// It will produce the following output:
///
/// 00101000
/// 01000100
/// 00000000
/// 01000100
/// 00101000
///
/// A lot of trait bounds are required, which makes the code look a bit messy, but it works great
/// and will work with any unsigned int type.
fn move_dragon<T>(dragon: &[T]) -> Vec<T>
where
    T: Default
        + Clone
        + Copy
        + From<u64>
        + Shl<T, Output = T>
        + Shr<T, Output = T>
        + BitOr<Output = T>
        + BitOrAssign,
{
    let len = dragon.len();
    let mut next_move = vec![T::default(); len];
    for (i, curr) in dragon.iter().enumerate() {
        // Each "1" in the current line will affect lines i-1, i-2, i+1, i+2
        for j in -2..=2 {
            if j == 0 {
                continue;
            }
            let k = i.wrapping_add_signed(j);
            if (0..len).contains(&k) {
                let shift = T::from(3 - j.unsigned_abs() as u64); // 2 for j=2; 1 for j=1.
                let next = (*curr << shift) | (*curr >> shift);
                next_move[k] |= next;
            }
        }
    }
    next_move
}

/// Convert a grid of chars to an array of bit masks (unsigned ints) of generic size.
/// Each line in the grid will be an element of the output vector.
/// For example, given the following line:
///     ..S.SS.S
/// The output will be (using 'S' as the second argument):
///     10110100
fn bitmap_from_char<T>(grid: &[Vec<char>], c: char) -> Vec<T>
where
    T: Default + Clone + Copy + From<u64> + Shl<T, Output = T> + AddAssign,
{
    let len = grid.len();
    let mut output = vec![T::default(); len];
    for i in 0..len {
        let line = &grid[i];
        line.iter().enumerate().for_each(|(j, &x)| {
            if x == c {
                output[i] += T::from(1u64) << T::from(j as u64);
            }
        })
    }
    output
}

/// Sheep only move down the board.
fn move_sheep<T: Default + Copy>(sheep: &mut [T]) {
    let mut _tmp = sheep[0];
    for x in sheep.iter_mut() {
        (_tmp, *x) = (*x, _tmp);
    }
    sheep[0] = T::default();
}

/// Removes eaten ship from the board and returns the count of eaten sheep.
/// Can't do this with generics because there's no trait bound for count_ones method.
fn eat_sheep(dragon: &[u128], sheep: &mut [u128], hides: &[u128]) -> u32 {
    let eaten = dragon
        .iter()
        .zip(sheep.iter().zip(hides.iter()))
        .map(|(&d, (&s, &h))| d & s & !h)
        .collect::<Vec<u128>>();

    for i in 0..sheep.len() {
        sheep[i] &= !eaten[i];
    }

    eaten.iter().map(|x| x.count_ones()).sum::<u32>()
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    const TURNS: u32 = if cfg!(test) { 3 } else { 4 };
    let grid = parse_input(notes);
    let len = grid.len();

    let sheep = bitmap_from_char::<u64>(&grid, 'S');
    let mut dragon = bitmap_from_char::<u64>(&grid, 'D');
    for _ in 0..TURNS {
        let next_move = move_dragon::<u64>(&dragon);
        for i in 0..len {
            dragon[i] |= next_move[i];
        }
    }
    // dragon.iter().for_each(|&x| println!("{:032b}", x));

    let count = sheep
        .iter()
        .zip(dragon.iter())
        .map(|(&s, &d)| (s & d).count_ones())
        .sum::<u32>();
    Some(count.to_string())
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    const TURNS: u32 = if cfg!(test) { 3 } else { 20 };
    let grid = parse_input(notes);
    let len = grid.len();

    let mut sheep = bitmap_from_char::<u128>(&grid, 'S');
    let mut dragon = bitmap_from_char::<u128>(&grid, 'D');
    let hides = bitmap_from_char::<u128>(&grid, '#');

    let mut count: u32 = 0;
    for _ in 0..TURNS {
        // Move dragon and count the number of eaten sheep.
        // NOTE: Sheep in hideouts do not get eaten.
        // NOTE: Eaten sheep are removed from the board!!
        dragon = move_dragon(&dragon);
        count += eat_sheep(&dragon, &mut sheep, &hides);

        // Move sheep and count eaten sheep.
        move_sheep(&mut sheep);
        count += eat_sheep(&dragon, &mut sheep, &hides);
    }
    Some(count.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(10, 1));
        assert_eq!(result, Some(27.to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(10, 2));
        assert_eq!(result, Some(27.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(10, 3));
        assert_eq!(result, None);
    }
}
