use tracing::info;

fn process(input: &str, num_lines: usize) -> u64 {
    let iter = &mut input.lines().map(|line| line.split_ascii_whitespace());

    let numbers: Vec<Vec<_>> = iter
        .take(num_lines)
        .map(|line| {
            line.map(|e| e.parse::<u64>().expect("not a number"))
                .collect()
        })
        .collect();

    iter.next()
        .expect("op row")
        .enumerate()
        .map(|(idx, line)| match line {
            "+" => numbers.iter().map(|line| line[idx]).sum::<u64>(),
            "*" => numbers
                .iter()
                .map(|line| line[idx])
                .reduce(|acc, v| acc * v)
                .expect("Empty iterator"),
            op => panic!("Unknown op {op}"),
        })
        .sum()
}

#[tracing::instrument(ret, skip(lines))]
fn extract_numbers(lines: &[Vec<char>], start: usize, len: usize) -> impl Iterator<Item = u64> {
    (start..(start + len)).map(|idx| {
        lines
            .iter()
            .map(|line| line[idx])
            .filter(|c| c != &' ')
            .collect::<String>()
            .parse::<u64>()
            .expect("not a number")
    })
}

#[tracing::instrument(skip(input))]
fn process2(input: &str) -> u64 {
    let lines: Vec<Vec<_>> = input
        .chars()
        .rev()
        .collect::<String>()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut last_numbers = vec![];
    let mut result: u64 = 0;

    info!(ops = ?lines[0]);
    for (idx, op) in lines[0].iter().enumerate() {
        let number = (1..lines.len())
            .map(|row| lines[row][idx])
            .filter(|v| v != &' ')
            .rev()
            .collect::<String>();

        if !number.is_empty() {
            info!(number);
            last_numbers.push(number.parse::<u64>().expect("not a number"));
            match op {
                ' ' => continue,
                '+' => result += last_numbers.iter().sum::<u64>(),
                '*' => result += last_numbers.iter().product::<u64>(),
                c => unreachable!("Unknown op {c}"),
            }
            info!(?op, ?last_numbers, result);
            last_numbers.clear();
        }
    }
    info!(result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_log::test]
    fn test_part1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(process(input, 3), 4277556);
        assert_eq!(
            process(include_str!("../input/day6_1.txt"), 4),
            6725216329103
        );
    }

    #[test_log::test]
    fn test_part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        // assert_eq!(process2(input), 3263827);
        assert_eq!(
            process2(include_str!("../input/day6_1.txt")),
            10600728112865
        );
    }
}
