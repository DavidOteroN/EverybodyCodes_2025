ec::solution!(2);

use ec::utils::complex::Complex;
use std::str::FromStr;

fn cycle(start: Complex, step: Complex, scale: i64) -> Complex {
    let mut out = start;
    out *= out;
    out /= Complex::new(scale, 0);
    out += step;

    out
}

fn run<const CYCLES: usize, const SCALE: i64>(z: Complex) -> Option<Complex> {
    const LIMIT: i64 = 1000000;
    let limit_range = -LIMIT..=LIMIT;
    let mut out = Complex::new(0, 0);
    for _ in 0..CYCLES {
        out = cycle(out, z, SCALE);
        if !limit_range.contains(&out.real()) || !limit_range.contains(&out.imag()) {
            return None;
        }
    }

    Some(out)
}

#[allow(unused_variables)]
pub fn part_one(notes: &str) -> Option<String> {
    let z = Complex::from_str(notes.trim()).unwrap();
    let r = run::<3, 10>(z).unwrap();
    Some(format!("[{},{}]", r.real(), r.imag()))
}

#[allow(unused_variables)]
pub fn part_two(notes: &str) -> Option<String> {
    const SIZE: usize = 1001;
    const STEP: usize = 10;
    let start = Complex::from_str(notes.trim()).unwrap();
    let count = (0..SIZE)
        .step_by(STEP)
        .flat_map(|i| (0..SIZE).step_by(STEP).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            let point = start + Complex::new(i as i64, j as i64);
            run::<100, 100000>(point)
        })
        .count();
    Some(count.to_string())
}

#[allow(unused_variables)]
pub fn part_three(notes: &str) -> Option<String> {
    const SIZE: usize = 1001;
    const STEP: usize = 1;
    let start = Complex::from_str(notes.trim()).unwrap();

    let count = (0..SIZE)
        .step_by(STEP)
        .flat_map(|i| (0..SIZE).step_by(STEP).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            let point = start + Complex::new(i as i64, j as i64);
            run::<100, 100000>(point)
        })
        .count();
    Some(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ec::read_example_file;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example_file(2, 1));
        assert_eq!(result, Some("[357,862]".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example_file(2, 2));
        assert_eq!(result, Some(4076.to_string()));
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&read_example_file(2, 3));
        assert_eq!(result, Some(406954.to_string()));
    }
}
