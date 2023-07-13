use common::{
    lines_from_file,
    pair::{Pair, DOWN, LEFT, RIGHT, UP},
};
use std::{borrow::Borrow, collections::HashSet};

type Error = &'static str;

pub fn run(filename: &str) -> Result<(), Error> {
    let steps = parser(lines_from_file(filename)?).collect::<Result<Vec<_>, _>>()?;
    let solution_p1 = solve_p1(steps.iter());
    println!("P1 Solution: {}", solution_p1);
    let solution_p2 = solve_p2(steps.iter());
    println!("P2 Solution: {}", solution_p2);
    Ok(())
}

fn solve_p1(steps: impl Iterator<Item = impl Borrow<Step>>) -> usize {
    let mut head = Pair { x: 0, y: 0 };
    let mut tail = Pair { x: 0, y: 0 };
    let mut tail_positions = HashSet::new();
    for step in steps {
        let step = step.borrow();
        for _ in 0..step.repeat {
            head = head + step.direction;
            follow(head, &mut tail);
            tail_positions.insert(tail);
        }
    }

    tail_positions.len()
}

fn follow(head: Pair, tail: &mut Pair) {
    let diff = *tail - head;
    if diff.y == 0 && diff.x.abs() > 1 {
        tail.x -= diff.x.signum();
    }
    if diff.x == 0 && diff.y.abs() > 1 {
        tail.y -= diff.y.signum();
    }
    if diff.x.abs() + diff.y.abs() > 2 {
        tail.x -= diff.x.signum();
        tail.y -= diff.y.signum();
    }
}

const SIZE: usize = 10;

fn solve_p2(steps: impl Iterator<Item = impl Borrow<Step>>) -> usize {
    let mut rope: Vec<Pair> = vec![Pair { x: 0, y: 0 }; SIZE];
    let mut tail_positions = HashSet::new();
    for step in steps {
        let step = step.borrow();
        for _ in 0..step.repeat {
            rope[0] = rope[0] + step.direction;
            for i in 0..SIZE - 1 {
                follow(rope[i], &mut rope[i + 1]);
            }
            tail_positions.insert(rope.last().copied().unwrap());
        }
    }
    tail_positions.len()
}

#[derive(Debug)]
struct Step {
    direction: Pair,
    repeat: i16,
}

fn parser(
    input: impl Iterator<Item = impl Borrow<str>>,
) -> impl Iterator<Item = Result<Step, Error>> {
    input.map(|line| {
        let mut line = line.borrow().split_whitespace();

        let direction = line
            .next()
            .and_then(|c| c.chars().nth(0))
            .ok_or("Couldn't find direction in line")?;

        let direction = match direction {
            'U' => Ok(UP),
            'L' => Ok(LEFT),
            'R' => Ok(RIGHT),
            'D' => Ok(DOWN),
            _ => Err("Unrecognized letter"),
        }?;

        let repeat = line
            .next()
            .ok_or("Number of steps not found in line")?
            .parse()
            .or(Err("Couldn't parse number of steps"))?;

        Ok(Step { direction, repeat })
    })
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn solution_p1() {
        let steps = parser(INPUT.lines()).map(Result::unwrap);
        let solution = solve_p1(steps);
        assert_eq!(solution, 13);
    }

    #[test]
    fn solution_p2() {
        let steps = parser(INPUT.lines()).map(Result::unwrap);
        let solution = solve_p2(steps);
        assert_eq!(solution, 1);
    }

    const LARGER_INPUT: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn solution_p2_larger() {
        let steps = parser(LARGER_INPUT.lines()).map(Result::unwrap);
        let solution = solve_p2(steps);
        assert_eq!(solution, 36);
    }
}
