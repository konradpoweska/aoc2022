use common::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;

type Error = &'static str;

pub fn run(filename: &str) -> Result<(), Error> {
    let lines = lines_from_file(filename)?;
    let (stacks, movements) = parse_input(lines)?;

    let result1 = solve(stacks.clone(), &movements, apply_movement_p1)?;
    let result2 = solve(stacks, &movements, apply_movement_p2)?;

    println!("Result P1: {}", &result1);
    println!("Result P2: {}", &result2);
    Ok(())
}

type Stacks = Vec<Stack>;
type Stack = Vec<char>;

fn parse_input(
    mut lines_iter: impl Iterator<Item = impl Borrow<str>>,
) -> Result<(Stacks, Vec<Movement>), Error> {
    let stacks = parse_stacks(&mut lines_iter)?;
    let movements = parse_movements(&mut lines_iter)?;
    Ok((stacks, movements))
}

fn parse_stacks(lines_iter: &mut impl Iterator<Item = impl Borrow<str>>) -> Result<Stacks, Error> {
    let stacks_lines: Vec<_> = lines_iter
        .by_ref()
        .take_while(|a| !a.borrow().is_empty())
        .collect();

    let mut from_bottom = stacks_lines.iter().rev();

    let stack_count = {
        let numbers_line = from_bottom.next().ok_or("Missing line with numbers.")?;
        let last_number = numbers_line.borrow().split_whitespace().last();
        last_number
            .and_then(|ln| ln.parse::<usize>().ok())
            .ok_or("Couldn't parse last number")
    }?;

    let mut stacks: Stacks = vec![vec![]; stack_count];

    for line in from_bottom {
        let chars = line.borrow().chars().skip(1).step_by(4).enumerate();
        for (index, letter) in chars {
            if letter.is_alphanumeric() {
                stacks[index].push(letter)
            }
        }
    }

    Ok(stacks)
}

fn parse_movements(
    lines_iter: &mut impl Iterator<Item = impl Borrow<str>>,
) -> Result<Vec<Movement>, Error> {
    lines_iter
        .map(|line| Movement::parse(line.borrow()))
        .collect()
}

fn solve(
    mut stacks: Stacks,
    movements: &Vec<Movement>,
    apply_movement: fn(&mut Stacks, &Movement) -> Result<(), Error>,
) -> Result<String, Error> {
    for movement in movements {
        apply_movement(&mut stacks, movement)?;
    }
    get_top_crates(&stacks)
}

fn apply_movement_p1(
    stacks: &mut Stacks,
    Movement { from, to, crates }: &Movement,
) -> Result<(), Error> {
    for _ in 0..*crates {
        let moved_crate = stacks[from - 1].pop().ok_or("Cannot take crate")?;
        stacks[to - 1].push(moved_crate);
    }
    Ok(())
}

fn apply_movement_p2(
    stacks: &mut Stacks,
    Movement { from, to, crates }: &Movement,
) -> Result<(), Error> {
    let idx = stacks[from - 1]
        .len()
        .checked_sub(*crates)
        .ok_or("Not enough crates on this stack")?;
    let moved_crates = stacks[from - 1].split_off(idx);
    stacks[to - 1].extend(moved_crates);
    Ok(())
}

lazy_static! {
    static ref MOVEMENT_PATTERN: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

#[derive(PartialEq, Debug)]
struct Movement {
    crates: usize,
    from: usize,
    to: usize,
}
impl Movement {
    fn parse(line: &str) -> Result<Self, Error> {
        let captures = MOVEMENT_PATTERN
            .captures(line)
            .ok_or("Couldn't parse movement.")?;
        Ok(Movement {
            crates: expect_number(captures.get(1))?,
            from: expect_number(captures.get(2))?,
            to: expect_number(captures.get(3))?,
        })
    }
}

fn expect_number(captured: Option<regex::Match>) -> Result<usize, Error> {
    captured
        .ok_or("Match not found")?
        .as_str()
        .parse()
        .or(Err("Couldn't parse number"))
}

fn get_top_crates(stacks: &Stacks) -> Result<String, Error> {
    stacks
        .iter()
        .map(|s| s.last().ok_or("Can't get top crate: stack is empty"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn stacks_parsing() {
        let stacks = parse_stacks(&mut INPUT.lines()).unwrap();
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(stacks, expected);
    }

    #[test]
    fn movement_parsing() {
        assert_eq!(
            Movement::parse("move 2 from 3 to 1").unwrap(),
            Movement {
                crates: 2,
                from: 3,
                to: 1
            }
        );
    }

    #[test]
    fn solve_p1() {
        let (stacks, movements) = parse_input(INPUT.lines()).unwrap();
        let result = solve(stacks, &movements, apply_movement_p1).unwrap();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn solve_p2() {
        let (stacks, movements) = parse_input(INPUT.lines()).unwrap();
        let result = solve(stacks, &movements, apply_movement_p2).unwrap();
        assert_eq!(result, "MCD");
    }
}
