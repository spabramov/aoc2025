use tracing::info;

fn is_invalid(n: &usize) -> bool {
    let digits = n.ilog10() + 1;
    if !digits.is_multiple_of(2) {
        return false;
    }
    let half = 10_usize.pow(digits / 2);
    let upper = n / half;
    let lower = n % half;
    if upper == lower {
        dbg!(n);
        true
    } else {
        false
    }
}

#[tracing::instrument]
fn is_really_invalid(n: &usize) -> bool {
    let digits = n.ilog10() + 1;

    'outer: for len in 1..=(digits / 2) {
        if !digits.is_multiple_of(len) {
            continue;
        }

        let divisor = 10_usize.pow(len);
        let mut prev = n % divisor;
        for i in 2..=(digits / len) {
            let next = n % 10_usize.pow(i * len) / 10_usize.pow((i - 1) * len);
            if next != prev {
                continue 'outer;
            }
            prev = next
        }
        return true;
    }
    false
}

fn parse_range(range: &str) -> (usize, usize) {
    let (first, second) = range.split_once('-').unwrap();
    (first.parse().unwrap(), second.trim().parse().unwrap())
}

fn process(input: &str, predicate: fn(&usize) -> bool) -> String {
    let result: usize = input
        .split(',')
        .map(parse_range)
        .map(|(a, b)| (a..=b).filter(predicate).sum::<usize>())
        .sum();
    result.to_string()
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(11, true)]
    #[case(101, false)]
    #[case(1, false)]
    #[case(123123, true)]
    fn test_ivalid(#[case] n: usize, #[case] expected: bool) {
        assert_eq!(is_invalid(&n), expected)
    }

    #[rstest]
    #[case(11, true)]
    #[case(101, false)]
    #[case(111, true)]
    #[case(1, false)]
    #[case(123123, true)]
    #[case(1188511885, true)]
    fn test_really_ivalid(#[case] n: usize, #[case] expected: bool) {
        assert_eq!(is_really_invalid(&n), expected);
    }

    #[test_log::test]
    fn test_solution() {
        assert_eq!(
            process(include_str!("../input/day2_0.txt"), is_invalid),
            "1227775554"
        );
        assert_eq!(
            process(include_str!("../input/day2_1.txt"), is_invalid),
            "19219508902"
        );
        assert_eq!(
            process(include_str!("../input/day2_0.txt"), is_really_invalid),
            "4174379265"
        );
        assert_eq!(
            process(include_str!("../input/day2_1.txt"), is_really_invalid),
            "27180728081"
        );
    }
}
