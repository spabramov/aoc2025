use std::collections::HashMap;
use tracing::info;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(':').expect("Expected `:`");
            (name.trim(), rest.split_ascii_whitespace().collect())
        })
        .collect()
}

fn count(node: &str, devices: &HashMap<&str, Vec<&str>>) -> u64 {
    if node == "out" {
        1
    } else {
        devices
            .get(node)
            .unwrap_or_else(|| panic!("node {node} is missing"))
            .iter()
            .map(|subnode| count(subnode, devices))
            .sum()
    }
}
fn process_part1(input: &str) -> u64 {
    count("you", &parse(input))
}

/// (total, dac, fft, both)
type Cache<'a> = HashMap<&'a str, (u64, u64, u64, u64)>;

#[tracing::instrument(skip(input))]
fn process_part2(input: &str) -> u64 {
    fn trace_count<'a>(
        node: &str,
        devices: &HashMap<&str, Vec<&'a str>>,
        cache: &mut Cache<'a>,
    ) -> (u64, u64, u64, u64) {
        if node == "out" {
            info!("out");
            return (1, 0, 0, 0);
        }

        let (total, with_dac, with_fft, with_both) = devices
            .get(node)
            .unwrap_or_else(|| panic!("node {node} is missing"))
            .iter()
            .map(|&subnode| {
                if let Some(cached) = cache.get(subnode) {
                    *cached
                } else {
                    let value = trace_count(subnode, devices, cache);
                    info!(subnode, ?value);

                    cache.insert(subnode, value);

                    value
                }
            })
            .reduce(|acc, v| (acc.0 + v.0, acc.1 + v.1, acc.2 + v.2, acc.3 + v.3))
            .expect("empty children list");
        (
            total,
            if node == "dac" { total } else { with_dac },
            if node == "fft" { total } else { with_fft },
            match (node == "dac", node == "fft") {
                (true, true) => total,
                (true, false) => with_fft,
                (false, true) => with_dac,
                (false, false) => with_both,
            },
        )
    }

    let mut cache: Cache = HashMap::new();
    trace_count("svr", &parse(input), &mut cache).3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(process_part1(input), 5);
        assert_eq!(process_part1(include_str!("../input/day11.txt")), 494);
    }

    #[test_log::test]
    fn test_part2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(process_part2(input), 2);
        assert_eq!(
            process_part2(include_str!("../input/day11.txt")),
            296006754704850
        );
    }
}
