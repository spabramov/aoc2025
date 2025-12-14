use std::ops::RangeInclusive;

use itertools::Itertools;
use tracing::info;

#[tracing::instrument(skip(input))]
fn process(input: &str) -> usize {
    let iter = &mut input.lines();

    let ranges: Vec<RangeInclusive<usize>> = iter
        .map_while(|line| {
            if line.trim().is_empty() {
                None
            } else {
                let (start, end) = line.split_once('-')?;
                Some(RangeInclusive::new(start.parse().ok()?, end.parse().ok()?))
            }
        })
        .collect();

    iter.filter_map(|line| {
        let id = line.parse().ok()?;
        ranges
            .iter()
            .find(|range| range.contains(&id))
            .and(Some(()))
    })
    .count()
}

#[tracing::instrument(skip(input))]
fn process2(input: &str) -> usize {
    let iter = input
        .lines()
        .map_while(|line| {
            if line.trim().is_empty() {
                None
            } else {
                let (start, end) = line.split_once('-')?;
                Some(RangeInclusive::new(
                    start.parse::<usize>().ok()?,
                    end.parse::<usize>().ok()?,
                ))
            }
        })
        .sorted_by_key(|range| *range.start());

    let mut combined: usize = 0;
    let mut last_range: Option<RangeInclusive<_>> = None;
    for range in iter {
        info!(?range);
        if let Some(last) = last_range {
            if range.start() <= last.end() {
                info!(mode = "extend", ?last, ?range);
                last_range = Some(RangeInclusive::new(
                    *last.start(),
                    *(last.end().max(range.end())),
                ));
            } else {
                info!(mode = "next", ?last);
                combined += last.end() - last.start() + 1;
                last_range = Some(range);
            }
        } else {
            info!(collect = ?range);
            last_range = Some(range)
        }
    }
    if let Some(last) = last_range {
        info!(done = ?last);
        combined += last.end() - last.start() + 1;
    }

    combined
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_log::test]
    fn test_solution() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(process(input), 3);
        assert_eq!(process(include_str!("../input/day05.txt")), 635);
        assert_eq!(process2(input), 14);
    }

    #[test_log::test]
    fn test_part2() {
        assert_eq!(
            process2(include_str!("../input/day05.txt")),
            369761800782619
        );
    }
}
