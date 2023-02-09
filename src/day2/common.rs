use std::borrow::Borrow;

pub trait Round {
    fn points(&self) -> u32;
}

#[derive(Clone, Copy)]
pub enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl RoundResult {
    pub fn points(self) -> u32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }
    pub fn parse(character: &char) -> Result<Self, &'static str> {
        match character {
            'X' => Ok(RoundResult::Loss),
            'Y' => Ok(RoundResult::Draw),
            'Z' => Ok(RoundResult::Win),
            _ => Err("Unknown character"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn usage_points(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    pub fn parse(character: &char) -> Result<Self, &'static str> {
        match character {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err("Unknown character"),
        }
    }
}

impl From<Move> for i8 {
    fn from(value: Move) -> Self {
        match value {
            Move::Rock => 0,
            Move::Paper => 1,
            Move::Scissors => 2,
        }
    }
}

impl From<i8> for Move {
    fn from(value: i8) -> Self {
        match value {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => {
                panic!("No Move value matching number");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Letters(pub char, pub char);

pub fn parse_lines<I>(lines: I) -> Result<Vec<Letters>, &'static str>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    lines
        .into_iter()
        .map(|line| {
            let line: &str = line.borrow();
            let a = line.chars().nth(0).ok_or("No character has been found 0")?;
            let b = line.chars().nth(2).ok_or("No character has been found 2")?;
            Ok(Letters(a, b))
        })
        .collect()
}

pub fn solve<R: Round>(
    letters: &Vec<Letters>,
    parser: fn(letters: &Letters) -> Result<R, &'static str>,
) -> Result<u32, &'static str> {
    let rounds = letters.iter().map(parser).collect::<Result<Vec<_>, _>>()?;

    let sum = rounds.iter().map(Round::points).sum();

    Ok(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn line_parser() {
        let letters = parse_lines(INPUT.lines()).unwrap();
        assert_eq!(
            letters,
            vec![Letters('A', 'Y'), Letters('B', 'X'), Letters('C', 'Z'),]
        )
    }
}
