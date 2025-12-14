use std::collections::{HashMap, HashSet};

use tracing::info;

#[tracing::instrument(skip(input))]
fn process_part1(input: &str) -> u32 {
    let mut beams = HashSet::new();
    let mut num_splits = 0;

    for line in input.lines() {
        for (pos, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    beams.insert(pos);
                }
                '^' if beams.contains(&pos) => {
                    num_splits += 1;
                    beams.remove(&pos);
                    beams.insert(pos + 1);
                    beams.insert(pos - 1);
                }
                _ => {}
            }
        }
        info!(?beams, num_splits, line);
    }
    num_splits
}

#[tracing::instrument(skip(input))]
fn process_part2(input: &str) -> u64 {
    let mut beams = HashMap::new();

    for line in input.lines() {
        for (pos, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    beams.insert(pos, 1);
                }
                '^' => {
                    if let Some(cnt) = beams.remove(&pos) {
                        beams
                            .entry(pos + 1)
                            .and_modify(|v| *v += cnt)
                            .or_insert(cnt);
                        beams
                            .entry(pos - 1)
                            .and_modify(|v| *v += cnt)
                            .or_insert(cnt);
                    }
                }
                _ => {}
            }
        }
        info!(?beams, line);
    }
    beams.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_log::test]
    fn test_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(process_part1(input), 21);
        assert_eq!(process_part1(include_str!("../input/day07.txt")), 1646);
    }
    #[test_log::test]
    fn test_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(process_part2(input), 40);
        assert_eq!(
            process_part2(include_str!("../input/day07.txt")),
            32451134474991
        );
    }
}
