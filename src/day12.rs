use glam::{USizeVec2, usizevec2};
use std::{borrow::Cow, iter::repeat_n};

use nom::{
    IResult, Parser, branch,
    bytes::tag,
    character::complete::{self, line_ending, space1},
    combinator::{all_consuming, eof, opt},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};
use tracing::info;

#[derive(Debug, Clone)]
struct Present {
    num: usize,
    size: usize,
    rows: Vec<Vec<bool>>,
}

#[derive(Debug)]
struct CristmassTree {
    size: (usize, usize),
    needed: Vec<usize>,
}

fn present(input: &str) -> IResult<&str, Present> {
    (
        terminated(
            complete::usize,
            (complete::char(':'), complete::line_ending),
        ),
        terminated(
            separated_list1(
                complete::line_ending,
                many1(branch::alt((
                    complete::char('#').map(|_| true),
                    complete::char('.').map(|_| false),
                ))),
            ),
            line_ending,
        ),
    )
        .map(|(num, rows)| {
            let size = rows.iter().flatten().filter(|&v| *v).count();
            Present { num, size, rows }
        })
        .parse(input)
}

fn cristmass_tree(input: &str) -> IResult<&str, CristmassTree> {
    terminated(
        separated_pair(
            separated_pair(complete::usize, tag("x"), complete::usize),
            tag(": "),
            separated_list1(space1, complete::usize),
        ),
        opt(eof),
    )
    .map(|(size, presents)| CristmassTree {
        size,
        needed: presents,
    })
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Present>, Vec<CristmassTree>)> {
    all_consuming(terminated(
        separated_pair(
            separated_list1(line_ending, present),
            line_ending,
            separated_list1(line_ending, cristmass_tree),
        ),
        opt(line_ending),
    ))
    .parse(input)
}

impl Present {
    fn rotate(self, rotation: u8) -> Self {
        if rotation == 0 {
            return self;
        };
        let mut out = self.clone();
        for i in 0..3usize {
            for j in 0..3usize {
                out.rows[j][2 - i] = self.rows[i][j]
            }
        }
        out.rotate(rotation - 1)
    }
}

type Field = Vec<Vec<bool>>;

fn pack(field: &Field, present: &Present, offset: USizeVec2) -> Option<Field> {
    let mut out = Cow::from(field);

    for i in 0..3 {
        for j in 0..3 {
            if *out.get(offset.y + i)?.get(offset.x + j)? && present.rows[i][j] {
                // already occupied
                return None;
            }
            out.to_mut()[offset.y + i][offset.x + j] = present.rows[i][j]
        }
    }
    Some(out.into())
}

fn stuff_tree(field: &Field, needed: &[usize], presents: &Vec<Present>) -> Option<Field> {
    if let Some(present_id) = needed.iter().position(|amt| *amt > 0) {
        // info!(placing = present_id, from = ?needed);
        let mut new_needed = needed.to_owned();
        new_needed[present_id] -= 1;

        for r in 0..=3 {
            let present = presents[present_id].clone().rotate(r);
            for y in 0..(field.len() - 2) {
                for x in 0..(field[y].len() - 2) {
                    // info!(present.num, x, y, r);

                    // try to pack current present
                    if let Some(new_field) = pack(field, &present, usizevec2(x, y)) {
                        // try to pack other presents
                        if let Some(out) = stuff_tree(&new_field, &new_needed, presents) {
                            info!("fit {present_id} {needed:?} @{x},{y} r={r}!");
                            return Some(out);
                        }

                        // failed to pack other presents, lets move current present and try again
                    }
                }
            }
        }
        // failed at each possible position. Not possible to fit all presents
        // info!("couldn't fit {present_id}, {:?}", needed);
        return None;
    }

    // all placed already
    Some(field.to_owned())
}

fn pre_check(tree: &CristmassTree, presents: &[Present]) -> bool {
    let space_available = tree.size.0 * tree.size.1;
    let space_needed: usize = tree
        .needed
        .iter()
        .enumerate()
        .map(|(idx, amt)| presents[idx].size * amt)
        .sum();

    space_needed < space_available
}

fn process(input: &str) -> usize {
    let (_, (presents, trees)) = parse(input).expect("Failed to parse input");

    trees
        .iter()
        .enumerate()
        // .take(10)
        .filter(|&(idx, tree)| {
            let (width, length) = tree.size;
            let field = Vec::from_iter(repeat_n(Vec::from_iter(repeat_n(false, width)), length));
            if pre_check(tree, &presents) {
                // this is not needed for actual input
                if stuff_tree(&field, &tree.needed, &presents).is_some() {
                    info!("tree #{idx}: Success!");
                    true
                } else {
                    info!("tree #{idx}: Failure!");
                    false
                }
            } else {
                info!("tree #{idx}: Precheck failure!");
                false
            }
        })
        .count()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test_log::test]
    fn test_parse_present() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        assert_eq!(process(input), 2);
        assert_eq!(process(include_str!("../input/day12.txt")), 0);
    }
}

// 60
//
