use tracing::info;

#[tracing::instrument(skip(bank, size))]
fn extract_joltage(bank: &str, size: usize) -> usize {
    fn find_max(line: &str) -> (char, usize) {
        if line.is_empty() {
            panic!("No chars found!");
        }

        let c = line.chars().max().unwrap();

        (c, line.find(c).unwrap())
    }
    if bank.len() < size {
        panic!("Not enough chars!");
    }

    let mut result: usize = 0;
    let mut pos: usize = 0;

    for i in 1..=size {
        let (digit, rest) = find_max(&bank[pos..bank.len() - (size - i)]);
        result = result * 10 + digit.to_digit(10).unwrap() as usize;
        pos += rest + 1;
    }

    info!(result);
    result
}

fn process(input: &str, size: usize) -> String {
    input
        .lines()
        .map(|v| extract_joltage(v, size))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("111511611516111", 66)]
    #[test_log::test]
    fn test_extract_max_joltage(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(extract_joltage(input, 2), expected);
    }

    #[test_log::test]
    fn test_part1() {
        assert_eq!(process(include_str!("../input/day3_0.txt"), 2), "357");
        assert_eq!(process(include_str!("../input/day3_1.txt"), 2), "16854");

        assert_eq!(
            process(include_str!("../input/day3_0.txt"), 12),
            "3121910778619"
        );
        assert_eq!(
            process(include_str!("../input/day3_1.txt"), 12),
            "167526011932478"
        );
    }
}
