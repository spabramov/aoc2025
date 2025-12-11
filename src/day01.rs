use std::{num::ParseIntError, str::FromStr};

enum Rotation {
    Left(i32),
    Rigth(i32),
}

impl FromStr for Rotation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(value) = s.strip_prefix('L') {
            Ok(Self::Left(value.parse()?))
        } else if let Some(value) = s.strip_prefix('R') {
            Ok(Self::Rigth(value.parse()?))
        } else {
            panic!("Not a valid rotation!")
        }
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| line.parse::<Rotation>().expect("Valid rotation"))
        .collect()
}

pub fn count_rotations_point_at_zeroes(input: &str) -> usize {
    let mut pos: i32 = 50;
    let mut zeroes: usize = 0;
    for turn in parse_input(input) {
        pos = match turn {
            Rotation::Left(val) => pos - val,
            Rotation::Rigth(val) => pos + val,
        } % 100;
        if pos == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

pub fn count_rotations_click_at_zeroes(input: &str) -> usize {
    let mut pos: usize = 50;
    let mut zeroes: usize = 0;
    for turn in parse_input(input) {
        let mut new_pos = match turn {
            Rotation::Left(val) => pos as i32 - val,
            Rotation::Rigth(val) => pos as i32 + val,
        };

        if new_pos == 0 {
            zeroes += 1
        } else if new_pos < 0 {
            let clicks = (new_pos / 100).abs();
            zeroes += clicks as usize;
            if pos > 0 {
                zeroes += 1;
            }
            new_pos += (clicks + 1) * 100;
        } else {
            zeroes += (new_pos as usize) / 100;
        }
        pos = (new_pos as usize) % 100;
    }
    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1_0() {
        assert_eq!(
            count_rotations_point_at_zeroes(include_str!("../input/day1_0.txt")),
            3
        );
        assert_eq!(
            count_rotations_click_at_zeroes(include_str!("../input/day1_0.txt")),
            6
        );
    }
    #[test]
    fn test_solution_1_1() {
        assert_eq!(
            count_rotations_point_at_zeroes(include_str!("../input/day1_1.txt")),
            984
        );
        assert_eq!(
            count_rotations_click_at_zeroes(include_str!("../input/day1_1.txt")),
            5657
        );
    }
}
