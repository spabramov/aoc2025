use std::collections::{BinaryHeap, HashSet};

use glam::I64Vec3;
use itertools::Itertools;
use tracing::info;

#[derive(Debug, Clone, Copy)]
struct Pair {
    a: I64Vec3,
    b: I64Vec3,
}

impl Pair {
    fn distance(&self) -> i64 {
        (self.a - self.b).length_squared()
    }
    fn is_connected(&self, other: Pair) -> bool {
        self.a == other.a || self.a == other.b || self.b == other.a
    }
}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}
impl Eq for Pair {}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance().cmp(&self.distance())
    }
}

#[tracing::instrument(skip(input))]
fn process_part1(input: &str, connections: usize) -> u64 {
    let boxes: Vec<_> = input
        .lines()
        .map(|line| {
            I64Vec3::from_slice(
                &line
                    .split(',')
                    .filter_map(|pos| pos.parse::<i64>().ok())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect();

    let mut heap: BinaryHeap<Pair> = boxes
        .iter()
        .combinations(2)
        .map(|pair| Pair {
            a: *pair[0],
            b: *pair[1],
        })
        .collect();

    let mut clusters: Vec<HashSet<I64Vec3>> = vec![];
    let mut conns_made = 0;
    while conns_made < connections
        && let Some(pair) = heap.pop()
    {
        let clus1 = clusters
            .iter()
            .position(|cluster| cluster.contains(&pair.a));
        let clus2 = clusters
            .iter()
            .position(|cluster| cluster.contains(&pair.b));
        match (clus1, clus2) {
            (Some(c1), Some(c2)) if c1 != c2 => {
                let first = c1.min(c2);
                let second = c1.max(c2);
                info!(merge = ?(first, second));

                let values = clusters.remove(second);
                clusters[first].extend(values);
            }
            (None, None) => {
                info!(new_cluster = ?pair);
                clusters.push(HashSet::from([pair.a, pair.b]))
            }
            (None, Some(c)) => {
                info!(position = c, append = ?pair);
                clusters[c].insert(pair.a);
            }
            (Some(c), None) => {
                info!(position = c, append = ?pair);
                clusters[c].insert(pair.b);
            }
            _ => {}
        }
        conns_made += 1;
    }
    clusters.sort_by_key(|cluster| cluster.len());
    clusters
        .iter()
        .rev()
        .map(|c| c.len() as u64)
        .filter(|l| *l > 0)
        .take(3)
        .inspect(|l| info!(l))
        .product()
}

#[tracing::instrument(skip(input))]
fn process_part2(input: &str) -> i64 {
    let boxes: Vec<_> = input
        .lines()
        .map(|line| {
            I64Vec3::from_slice(
                &line
                    .split(',')
                    .filter_map(|pos| pos.parse::<i64>().ok())
                    .collect::<Vec<i64>>(),
            )
        })
        .collect();
    let size = boxes.len();

    let mut heap: BinaryHeap<Pair> = boxes
        .iter()
        .combinations(2)
        .map(|pair| Pair {
            a: *pair[0],
            b: *pair[1],
        })
        .collect();

    let mut clusters: Vec<HashSet<I64Vec3>> = vec![];
    while let Some(pair) = heap.pop() {
        let clus1 = clusters
            .iter()
            .position(|cluster| cluster.contains(&pair.a));
        let clus2 = clusters
            .iter()
            .position(|cluster| cluster.contains(&pair.b));
        match (clus1, clus2) {
            (Some(c1), Some(c2)) if c1 != c2 => {
                info!(merge = ?(c1, c2));
                let values2 = clusters[c2].clone();
                clusters[c1].extend(values2);
                clusters.remove(c2);
            }
            (None, None) => {
                info!(new_cluster = ?pair);
                clusters.push(HashSet::from([pair.a, pair.b]))
            }
            (None, Some(c)) => {
                info!(position = c, append = ?pair);
                clusters[c].insert(pair.a);
            }
            (Some(c), None) => {
                info!(position = c, append = ?pair);
                clusters[c].insert(pair.b);
            }
            _ => {}
        }
        info!(clen = clusters[0].len());
        if clusters[0].len() == size {
            return pair.a.x * pair.b.x;
        }
    }
    info!(size);
    clusters.iter().for_each(|c| info!(len = c.len()));
    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_eq() {
        let pair1 = Pair {
            a: I64Vec3::new(425, 690, 689),
            b: I64Vec3::new(162, 817, 812),
        };
        let pair2 = Pair {
            a: I64Vec3::new(162, 817, 812),
            b: I64Vec3::new(425, 690, 689),
        };
        assert!(pair1 == pair2)
    }

    #[test_log::test]
    fn test_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(process_part1(input, 10), 40);
        assert_eq!(
            process_part1(include_str!("../input/day8_1.txt"), 1000),
            96672
        );
    }

    #[test_log::test]
    fn test_part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(process_part2(input), 25272);
        assert_eq!(process_part2(include_str!("../input/day8_1.txt")), 22517595);
    }
}
