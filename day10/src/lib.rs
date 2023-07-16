use common::lines_from_file;
use std::borrow::Borrow;

type Error = &'static str;

pub fn run(filename: &str) -> Result<(), Error> {
    let steps = lines_from_file(filename)?
        .map(Instruction::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let solution_p1 = solve_p1(steps.iter());
    println!("P1 Solution: {}", solution_p1);
    println!("P2 Solution:");
    solve_p2(steps.iter());
    Ok(())
}

enum Instruction {
    ADDX(i32),
    NOOP,
}

impl Instruction {
    fn parse(line: impl Borrow<str>) -> Result<Self, Error> {
        let mut terms = line.borrow().split_whitespace();
        let instruction = terms.next().ok_or("Missing instruction")?;
        match instruction {
            "addx" => {
                let operand = terms
                    .next()
                    .ok_or("Missing addx operand")?
                    .parse()
                    .or(Err("Couldn't parse addx operand"))?;
                Ok(Self::ADDX(operand))
            }
            "noop" => Ok(Self::NOOP),
            _ => Err("Unknown instruction"),
        }
    }
    fn get_duration(self: &Self) -> i32 {
        match self {
            Instruction::ADDX(_) => 2,
            Instruction::NOOP => 1,
        }
    }
    fn execute(self: &Self, register: &mut i32) {
        match self {
            Instruction::ADDX(x) => *register += x,
            Instruction::NOOP => {}
        };
    }
}

fn solve_p1(instructions: impl Iterator<Item = impl Borrow<Instruction>>) -> i32 {
    let mut sum = 0;
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;

    for instruction in instructions {
        let instruction = instruction.borrow();

        for _ in 0..instruction.get_duration() {
            cycle += 1;
            if cycle % 40 == 20 {
                sum += cycle * register;
            }
        }

        instruction.execute(&mut register);
    }

    sum
}

fn solve_p2(instructions: impl Iterator<Item = impl Borrow<Instruction>>) -> () {
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;

    for instruction in instructions {
        let instruction = instruction.borrow();
        for _ in 0..instruction.get_duration() {
            let column = cycle % 40;
            let sprite = register - 1..=register + 1;
            print!("{}", if sprite.contains(&column) { '#' } else { '.' });
            cycle += 1;
            if cycle % 40 == 0 {
                print!("\n");
            }
        }
        instruction.execute(&mut register);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn solution_p1() {
        let solution = solve_p1(INPUT.lines().map(Instruction::parse).map(Result::unwrap));
        assert_eq!(solution, 13140);
    }
}
