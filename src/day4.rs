#[tracing::instrument(ret, skip(input))]
fn count_neighbours(input: &[Vec<char>], r: &usize, c: &usize) -> usize {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(i, j)| !(*i == 0 && *j == 0))
        .filter_map(move |(i, j)| -> Option<()> {
            if input
                .get(r.checked_add_signed(i)?)?
                .get(c.checked_add_signed(j)?)?
                == &'@'
            {
                Some(())
            } else {
                None
            }
        })
        .count()
}
use itertools::Itertools;
fn process(input: &str) -> usize {
    let input: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = input.len();
    let width = input[0].len();

    (0..height)
        .cartesian_product(0..width)
        .filter(|(r, c)| input[*r][*c] == '@')
        .filter(|(r, c)| count_neighbours(&input, r, c) < 4)
        .count()
}

fn process2(input: &str) -> usize {
    let mut input: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = input.len();
    let width = input[0].len();

    let mut total = 0;
    let mut removed = 0;
    let mut initial = true;
    while initial || removed > 0 {
        initial = false;
        total += removed;
        removed = 0;

        for r in 0..height {
            for c in 0..width {
                if input[r][c] == '@' && count_neighbours(&input, &r, &c) < 4 {
                    removed += 1;
                    input[r][c] = 'x';
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod test {

    use super::*;

    #[test_log::test]
    fn test_solution() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(process(input), 13);
        assert_eq!(process(include_str!("../input/day4_1.txt")), 1505);

        assert_eq!(process2(input), 43);
        assert_eq!(process2(include_str!("../input/day4_1.txt")), 1505);
    }
}
