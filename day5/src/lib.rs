use common::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;

pub fn run(filename: &str) -> Result<(), &'static str> {
    let mut lines_iter = lines_from_file(&filename)?;
    let mut stacks = parse_stacks(&mut lines_iter)?;
    apply_movements(&mut stacks, &mut lines_iter)?;
    let result = get_top_crates(&stacks)?;
    println!("Result: {}", &result);
    Ok(())
}

type Stacks = Vec<Stack>;
type Stack = Vec<char>;

fn parse_stacks(
    lines_iter: &mut impl Iterator<Item = impl Borrow<str>>,
) -> Result<Stacks, &'static str> {
    let stacks_lines: Vec<_> = lines_iter
        .by_ref()
        .take_while(|a| !a.borrow().is_empty())
        .collect();

    let mut from_bottom = stacks_lines.iter().rev();

    let stack_count = {
        let numbers_line = from_bottom.next().ok_or("Missing line with numbers.")?;
        let last_number = numbers_line.borrow().trim().split_whitespace().last();
        last_number
            .and_then(|ln| ln.parse::<usize>().ok())
            .ok_or("Couldn't parse last number")
    }?;

    let mut stacks: Stacks = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

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

fn apply_movements(
    stacks: &mut Stacks,
    lines_iter: &mut impl Iterator<Item = impl Borrow<str>>,
) -> Result<(), &'static str> {
    for line in lines_iter {
        let Movement { from, to, crates } = Movement::parse(line.borrow())?;
        for _ in 0..crates {
            let moved_crate = stacks[from - 1].pop().ok_or("Cannot take crate")?;
            stacks[to - 1].push(moved_crate);
        }
    }
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
    fn parse(line: &str) -> Result<Self, &'static str> {
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

fn expect_number(captured: Option<regex::Match>) -> Result<usize, &'static str> {
    captured
        .ok_or("Match not found")?
        .as_str()
        .parse()
        .or(Err("Couldn't parse number"))
}

fn get_top_crates(stacks: &Stacks) -> Result<String, &'static str> {
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
    fn solve() {
        let mut lines = INPUT.lines();
        let mut stacks = parse_stacks(&mut lines).unwrap();
        apply_movements(&mut stacks, &mut lines).unwrap();
        let result = get_top_crates(&stacks).unwrap();
        assert_eq!(result, "CMZ");
    }
}
