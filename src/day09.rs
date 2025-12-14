use glam::{U64Vec2, u64vec2};
use itertools::Itertools;

fn process_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .and_then(|(x, y)| Some(u64vec2(x.parse::<u64>().ok()?, y.parse::<u64>().ok()?)))
                .expect("not a number pair")
        })
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .expect("Iterator is empty")
}

fn intersects((a, b): (&U64Vec2, &U64Vec2), (c, d): (&U64Vec2, &U64Vec2)) -> bool {
    let h1 = u64vec2(a.x.min(b.x), a.x.max(b.x));
    let h2 = u64vec2(c.x.min(d.x), c.x.max(d.x));

    let v1 = u64vec2(a.y.min(b.y), a.y.max(b.y));
    let v2 = u64vec2(c.y.min(d.y), c.y.max(d.y));

    h1.y > h2.x && h2.y > h1.x && v1.y > v2.x && v2.y > v1.x
}

#[tracing::instrument(skip(input))]
fn process_part2(input: &str) -> u64 {
    let tiles: Vec<_> = input
        .lines()
        .map(|line| {
            U64Vec2::from_slice(
                line.split(',')
                    .map(|v| v.parse::<u64>().expect("not a number"))
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
        })
        .collect();

    let lines: Vec<_> = tiles
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| (*a, *b))
        .collect();

    tiles
        .iter()
        .tuple_combinations()
        .filter(|&(a, b)| {
            !lines
                .iter()
                .filter(|(c, d)| c != a && c != b && d != a && d != b)
                .any(|(c, d)| intersects((a, b), (c, d)))
        })
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .expect("Iterator is empty")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(process_part1(input), 50);
        assert_eq!(
            process_part1(include_str!("../input/day09.txt")),
            4776487744
        );
    }

    #[test_log::test]
    fn test_part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(process_part2(input), 24);
        assert_eq!(
            process_part2(include_str!("../input/day09.txt")),
            1560299548
        );
    }
}
