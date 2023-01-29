use std::{borrow::Borrow, process};

pub fn run() {
    let rounds = parse_rounds(lines_from_stdin()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let score = solve(&rounds);
    println!("{score}");
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent: Move,
    me: Move,
}

#[derive(Clone, Copy)]
enum RoundResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl RoundResult {
    fn get_points(self) -> u32 {
        self as u32
    }
}

impl Round {
    fn result(&self) -> RoundResult {
        let me = self.me as i8;
        let opponent = self.opponent as i8;
        match me - opponent {
            0 => RoundResult::Draw,
            1 => RoundResult::Win,
            -1 => RoundResult::Loss,
            2 => RoundResult::Loss,
            -2 => RoundResult::Win,
            _ => panic!("Undefined behavior"),
        }
    }
    fn get_points(&self) -> u32 {
        self.result().get_points() + self.me.get_usage_points()
    }
}

fn solve(rounds: &Vec<Round>) -> u32 {
    rounds
        .iter()
        .map(|m| m.get_points())
        .fold(0, |acc, p| acc + p)
}

fn lines_from_stdin() -> impl Iterator<Item = String> {
    std::io::stdin().lines().map(|e| e.expect("Invalid line"))
}

fn parse_rounds<I>(lines: I) -> Result<Vec<Round>, &'static str>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    lines
        .into_iter()
        .map(|line| {
            let line: &str = line.borrow();
            let a = line.chars().nth(0).ok_or("No value")?;
            let b = line.chars().nth(2).ok_or("No value")?;

            let opponent = Move::parse(&a)?;
            let me = Move::parse(&b)?;
            Ok(Round { opponent, me })
        })
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn get_usage_points(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn parse(character: &char) -> Result<Self, &'static str> {
        match character {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err("Unknown character: {character}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn parser() {
        let rounds = parse_rounds(INPUT.lines()).unwrap();
        assert_eq!(
            rounds,
            vec![
                Round {
                    opponent: Move::Rock,
                    me: Move::Paper
                },
                Round {
                    opponent: Move::Paper,
                    me: Move::Rock
                },
                Round {
                    opponent: Move::Scissors,
                    me: Move::Scissors
                },
            ]
        );
    }

    #[test]
    fn solution() {
        let rounds = parse_rounds(INPUT.lines()).unwrap();
        let solution = solve(&rounds);
        assert_eq!(solution, 15);
    }
}
