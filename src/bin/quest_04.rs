ec::solution!(4);

#[inline]
fn parse_input(notes: &str) -> Vec<(u32, u32)> {
    // In part 3, gear trains are expressed as:
    //  A
    //  B|C
    //  D|E
    //  ...
    //  N
    // Where the pairs repparated by "|" are coaxial gears.
    // Parse all gears into tuples as follows:
    //  vec![
    //      (A, A),
    //      (B, C),
    //      (D, E),
    //      ...
    //      (N, N),
    //  ]
    notes
        .trim_end()
        .split("\n")
        .map(|s| parse_one_gear(s).unwrap())
        .collect()
}

#[inline]
fn parse_one_gear(gear: &str) -> Option<(u32, u32)> {
    match gear.split("|").collect::<Vec<&str>>()[..] {
        [a] => Some((a.parse::<u32>().unwrap(), a.parse::<u32>().unwrap())),
        [a, b] => Some((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())),
        _ => None,
    }
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    const TURNS_FIRST: u32 = 2025;
    let gears = parse_input(notes);

    // The gearing ration will be the quotient between the first and last gears.
    Some(format!(
        "{}",
        TURNS_FIRST * gears[0].0 / gears[gears.len() - 1].0
    ))
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    // Convert numbers to floats.
    const TURNS_LAST: f64 = 1e13;
    let gears = parse_input(notes);
    Some(format!(
        "{}",
        (TURNS_LAST * (gears[gears.len() - 1].0 as f64) / (gears[0].0 as f64)).ceil() as u64
    ))
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    let gears = parse_input(notes);
    const TURNS_FIRST: u32 = 100;
    let mut ratio = gears
        .iter()
        .fold(1f64, |acc, g| (g.1 as f64) / (g.0 as f64));
    ratio *= gears[gears.len() - 1].0 as f64 / gears[0].1 as f64;

    Some(format!("{}", (ratio * (TURNS_FIRST as f64)) as u32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(4, 1));
        assert_eq!(result, Some("15888".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(4, 2));
        assert_eq!(result, Some("1274509803922".to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(4, 3));
        assert_eq!(result, Some("6818".to_string()));
    }
}
