use std::borrow::Borrow;

use common::lines_from_file;
use pair::Pair;

mod pair;
mod part1;
mod part2;

type Error = &'static str;

pub fn run(filename: &str) -> Result<(), Error> {
    let forest = Forest::parse(lines_from_file(filename)?)?;
    println!("Size: {}", &forest.size);
    let solution_p1 = part1::solve(&forest);
    println!("Solution P1: {solution_p1}");
    let solution_p2 = part2::solve(&forest);
    println!("Solution P2: {solution_p2}");
    Ok(())
}

#[derive(Debug)]
pub struct Forest<T> {
    size: Pair,
    buffer: Vec<T>,
}

impl Forest<u8> {
    fn parse(mut input: impl Iterator<Item = impl Borrow<str>>) -> Result<Forest<u8>, Error> {
        let first = input.next().ok_or("Empty")?;
        let mut height = 1;
        let width = first.borrow().len() as i32;

        let mut buffer = Vec::with_capacity((width * width) as usize);
        buffer.extend(parse_line(first)?);

        for line in input {
            buffer.extend(parse_line(line)?);
            height += 1;
        }

        Ok(Forest {
            buffer,
            size: Pair {
                x: width,
                y: height,
            },
        })
    }
}

impl<T: Copy> Forest<T> {
    fn get(&self, Pair { x, y }: Pair) -> T {
        self.buffer[(y * self.size.x + x) as usize]
    }
    fn set(&mut self, Pair { x, y }: Pair, value: T) {
        self.buffer[(y * self.size.x + x) as usize] = value;
    }
}

fn parse_line(line: impl Borrow<str>) -> Result<Vec<u8>, Error> {
    line.borrow()
        .chars()
        .map(|c| Some(c.to_digit(10)? as u8))
        .collect::<Option<Vec<_>>>()
        .ok_or("Couldn't parse input")
}

#[cfg(test)]
mod tests {

    pub const INPUT: &str = "\
30373
25512
65332
33549
35390";
}
