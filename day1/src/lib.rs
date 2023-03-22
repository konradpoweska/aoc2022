use std::borrow::Borrow;

pub fn parse_elves<I>(lines: I) -> Vec<u32>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let mut elves: Vec<u32> = vec![];
    let mut accumulator: u32 = 0;

    for line in lines {
        match line.borrow().parse::<u32>() {
            Ok(number) => accumulator += number,
            Err(_) => {
                elves.push(accumulator);
                accumulator = 0
            }
        }
    }

    if accumulator != 0 {
        elves.push(accumulator)
    }

    elves
}

pub fn solve_part1(elves: &Vec<u32>) -> Result<u32, &'static str> {
    elves
        .iter()
        .max()
        .map(u32::clone)
        .ok_or("Max couldn't be found")
}

pub fn solve_part2(elves: &Vec<u32>) -> u32 {
    let mut elves = elves.clone();
    elves.sort_unstable_by(|a, b| b.cmp(a));
    elves[..3].iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parser() {
        let elves = parse_elves(INPUT.lines());
        assert_eq!(elves, vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn solution_part1() {
        let elves = vec![6000, 4000, 11000, 24000, 10000];
        let result = super::solve_part1(&elves).expect("Couldn't solve");
        assert_eq!(result, 24000);
    }

    #[test]
    fn solution_part2() {
        let elves = vec![6000, 4000, 11000, 24000, 10000];
        let result = super::solve_part2(&elves);
        assert_eq!(result, 45000);
    }
}
