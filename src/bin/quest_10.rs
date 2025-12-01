use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::Hash;
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Turn {
    Dragon,
    Sheep,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct GameState {
    sheep: u64,
    hides: u64,
    dragon: u64,
    _shape: (u8, u8),
    turn: Turn,
}

impl GameState {
    const MAX_BYTE: u64 = u8::MAX as u64;

    fn new(dragon: u64, sheep: u64, hides: u64, turn: Turn, shape: (u8, u8)) -> GameState {
        GameState {
            dragon,
            sheep,
            hides,
            _shape: shape,
            turn,
        }
    }

    fn from_notes(notes: &str) -> GameState {
        let height = notes.trim().lines().count() as u8;
        let width = notes.trim().lines().next().unwrap().chars().count() as u8;
        let mut dragon: u64 = 0;
        let mut sheep: u64 = 0;
        let mut hides: u64 = 0;
        for (i, line) in notes.trim().lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    'D' => dragon |= (1_usize << (i + 8 * j)) as u64,
                    'S' => sheep |= (1_usize << (i + 8 * j)) as u64,
                    '#' => hides |= (1_usize << (i + 8 * j)) as u64,
                    _ => (),
                }
            }
        }

        GameState::new(dragon, sheep, hides, Turn::Sheep, (height, width))
    }

    fn move_dragon_iter(&self) -> impl Iterator<Item = GameState> {
        let dirs = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        dirs.into_iter().filter_map(|(u, v)| {
            // It would be better to define x, y, width, and height outside the closure, but the
            // borrow checker doesn't like it.
            let (x, y) = self.dragon_coord();
            let (height, width) = self._shape;

            if let Some(new_x) = x.checked_add_signed(u)
                && let Some(new_y) = y.checked_add_signed(v)
                && (0..height).contains(&new_x)
                && (0..width).contains(&new_y)
            {
                // Valid dragon move.
                let new_dragon = Self::coord_to_bitmask(&[(new_x, new_y)]);

                // Check if new dragon eats any sheep.
                // NOTE: `!new_dragon | hides` will be all 1s except for a 0 in the new dagon's
                // position, unless there's a hide in the same bit, in which case it will just be
                // all ones.
                // If a sheeo happens to be in the same bit as the dragon and there's no hide, then
                // it will be removed, and the rest will be unaffected.
                let new_sheep = self.sheep & (!new_dragon | self.hides);

                let mut state = *self;
                state.dragon = new_dragon;
                state.sheep = new_sheep;
                state.turn = Turn::Sheep;

                Some(state)
            } else {
                // Move outside the board.
                None
            }
        })
    }

    fn move_sheep_iter(&self) -> impl Iterator<Item = GameState> {
        let (_, width) = self._shape;

        // Iterate on each byte (representing a column)
        (0..width).filter_map(|j| {
            let (height, _) = self._shape;
            // Get the byte for the sheep in colum j:
            let s = self.sheep & (Self::MAX_BYTE << (8 * j));
            if s == 0 {
                // No sheep in this column.
                None
            } else if s == (1 << (height - 1 + 8 * j)) {
                // If theep is at the last square in the column, then it will excape. Set the
                // dragon to 0.
                let mut new_state = *self;
                new_state.dragon = 0;
                new_state.turn = Turn::Dragon; // shouldn't really matter since match is lost.
                Some(new_state)
            } else {
                // Get the next square.
                // If the dragon is on the next sauqre and there's no hideout, then don't move
                // (return None). Else, move to the next square.
                // Get the sheep to the next square.
                let single_sheep = s << 1;

                // Check if it can safely move:
                if single_sheep & (!self.dragon | self.hides) == 0 {
                    None
                } else {
                    let mut new_state = *self;
                    new_state.sheep = (self.sheep & !(Self::MAX_BYTE << (8 * j))) | single_sheep;
                    new_state.turn = Turn::Dragon;

                    Some(new_state)
                }
            }
        })
    }

    #[inline]
    fn dragon_coord(&self) -> (u8, u8) {
        // Each byte represents a column.
        // Each bit in a byte represents a single square in the column.
        //
        // First, get the position of the 1 in the dragon's bitmask:
        let mut idx = 0_u8;
        for i in 0..64 {
            idx += (((self.dragon >> i) & 1) * i) as u8;
        }

        // Then, convert the index to coordinates.
        let y = idx / 8;
        let x = idx % 8;

        (x, y)
    }

    fn coord_to_bitmask(coord: &[(u8, u8)]) -> u64 {
        let mut b: u64 = 0;
        for (x, y) in coord {
            b |= 1_u64 << (x + 8 * y);
        }
        b
    }
}

/// Recursively compute all unique move sequences that make the dragon eat all the sheep.
/// Uses a very crude cache / memoization technique.
fn _compute_sequences_mem(state: GameState, cache: &mut HashMap<GameState, usize>) -> usize {
    // If there are no sheep remaining and no sheep have escaped, that means the dragon has eaten
    // all the sheep, so return 1.
    if state.sheep == 0 {
        return 1;
    }

    // Special termination condition: if any sheep escapes, the dragon will be set to 0.
    if state.dragon == 0 {
        return 0;
    }

    // Return value from cache if present.
    if let Some(r) = cache.get(&state) {
        return *r;
    }

    // If termination conditions are not reached and there's no cached result, then perform the
    // actual computation.
    let mut count: usize = 0;
    match state.turn {
        Turn::Dragon => {
            for s in state.move_dragon_iter() {
                count += _compute_sequences_mem(s, cache);
            }
        }
        Turn::Sheep => {
            if state.move_sheep_iter().count() == 0 {
                let mut new_state = state;
                new_state.turn = Turn::Dragon;
                for s in new_state.move_dragon_iter() {
                    count += _compute_sequences_mem(s, cache);
                }
            } else {
                for s in state.move_sheep_iter() {
                    count += _compute_sequences_mem(s, cache);
                }
            }
        }
    }

    // Store result in cache
    cache.insert(state, count);
    count
}

fn compute_sequences(state: GameState) -> usize {
    // NOTE: A very crude memoization implementation. There are probably tons of things to improve.
    let mut cache: HashMap<GameState, usize> = HashMap::new();
    _compute_sequences_mem(state, &mut cache)
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    // All the boards in the examples and the real input are smaller than 8x8. That means we can
    // use u64 bitmasks to represent the game state.
    // Each byte in the u64 word will represent a column of the board. Each bit in the byte will
    // represent an individual square.
    // NOTE: The dragon could be represented by a single byte indicating the index of the square,
    // and the sheep could be packed into a u32. But that approach complicates the calculations.

    // Parse input to game state:
    let state = GameState::from_notes(notes);

    // Flush cache bedore calling compute_sequences:
    let count = compute_sequences(state);
    Some(count.to_string())
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
    fn test_part_three_1() {
        let result = part_three(&read_example_file(10, 3));
        assert_eq!(result, Some(15.to_string()));
    }

    #[test]
    fn test_part_three_2() {
        let result = part_three(&read_example_file(10, 4));
        assert_eq!(result, Some(8.to_string()));
    }

    #[test]
    fn test_part_three_3() {
        let result = part_three(&read_example_file(10, 5));
        assert_eq!(result, Some(44.to_string()));
    }

    #[test]
    fn test_part_three_4() {
        let result = part_three(&read_example_file(10, 6));
        assert_eq!(result, Some(4406.to_string()));
    }

    #[test]
    fn test_part_three_5() {
        let result = part_three(&read_example_file(10, 7));
        assert_eq!(result, Some(13033988838_usize.to_string()));
    }
}
