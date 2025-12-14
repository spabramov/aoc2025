use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    sequence::delimited,
};
use tracing::{info, warn};

type Button = Vec<usize>;

#[derive(Debug)]
struct Problem {
    diagram: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Vec<usize>,
}

fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
    (separated_list1(line_ending, parse_problem)).parse(input)
}

#[tracing::instrument]
fn parse_problem(input: &str) -> IResult<&str, Problem> {
    let (input, diagram) = delimited(
        complete::char('['),
        many1(alt((
            complete::char('.').map(|_| false),
            complete::char('#').map(|_| true),
        ))),
        complete::char(']'),
    )
    .parse(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, buttons) = separated_list1(
        complete::char(' '),
        delimited(
            complete::char('('),
            separated_list1(complete::char(','), complete::usize),
            complete::char(')'),
        ),
    )
    .parse(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, joltage) = delimited(
        complete::char('{'),
        separated_list1(complete::char(','), complete::usize),
        complete::char('}'),
    )
    .parse(input)?;

    let (input, _) = complete::space0(input)?;

    Ok((
        input,
        Problem {
            diagram,
            buttons,
            joltage,
        },
    ))
}

fn check_diagram(buttons: &[&Vec<usize>], diagram: &[bool]) -> bool {
    buttons
        .iter()
        .flat_map(|&v| v)
        .fold(diagram.to_owned(), |mut state, bulb| {
            state[*bulb] = !state[*bulb];
            state
        })
        .iter()
        .all(|&v| !v)
}

#[tracing::instrument(skip(input))]
fn process_part1(input: &str) -> usize {
    let (_, problems) = parse(input).unwrap();
    problems
        .iter()
        .map(|problem| {
            info!(diagram = ?problem.diagram);
            problem
                .buttons
                .iter()
                .powerset()
                .filter_map(|buttons| {
                    check_diagram(&buttons, &problem.diagram).then_some(buttons.len())
                })
                .min()
                .expect("min() failed")
        })
        .sum()
}

fn find_joltage_solution_rec<'a>(
    buttons: &'a [Button],
    joltage: &[usize],
    cache: &mut HashMap<Vec<bool>, Vec<Vec<&'a Button>>>,
) -> usize {
    if joltage.iter().sum::<usize>() == 0 {
        return 0;
    }
    info!(?joltage);
    let diagram: Vec<_> = joltage.iter().map(|&j| !j.is_multiple_of(2)).collect();

    let result = cache
        .entry(diagram.clone())
        .or_insert_with_key(|diagram| {
            buttons
                .iter()
                .powerset()
                .filter(|buttons| check_diagram(buttons, diagram))
                .collect()
        })
        .clone() // need exclusive access to cache
        .iter()
        .filter_map(|variant| -> Option<usize> {
            let rem_half_joltage: Vec<_> = joltage
                .iter()
                .enumerate()
                .map(|(bulb, value)| -> Result<usize, ()> {
                    Ok(value
                        .checked_sub(
                            variant
                                .iter()
                                .filter(|button| button.contains(&bulb))
                                .count(),
                        )
                        .ok_or(())?
                        / 2)
                })
                .collect::<Result<Vec<usize>, ()>>()
                .ok()?; // skipping this variant of failed to subtract buttons from joltage
            info!(?variant, ?rem_half_joltage);
            Some(variant.len() + 2 * find_joltage_solution_rec(buttons, &rem_half_joltage, cache))
        })
        .min()
        .unwrap_or(1_000_000);

    info!(?joltage, result);
    result
}

fn find_joltage_solution(problem: &Problem) -> usize {
    let mut cache = HashMap::new();
    find_joltage_solution_rec(&problem.buttons, &problem.joltage, &mut cache)
}

#[tracing::instrument(skip(input))]
fn process_part2(input: &str) -> usize {
    let (_, problems) = parse(input).unwrap();
    problems
        .iter()
        .enumerate()
        .map(|(i, problem)| {
            let value = find_joltage_solution(problem);
            info!("Problem#{i}: {value}");
            value
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test_log::test]
    fn test_part1() {
        let input = "[.###] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(process_part1(input), 7);
        assert_eq!(process_part1(include_str!("../input/day10.txt")), 396);
    }

    #[test_log::test]
    fn test_part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(process_part2(input), 33);
        assert_eq!(process_part2(include_str!("../input/day10.txt")), 15688);
    }
}
