use super::common::{Letters, Move, Round, RoundResult};

pub struct RoundPart2 {
    opponent: Move,
    result: RoundResult,
}

const MODULO: i8 = 3;

impl RoundPart2 {
    fn my_move(&self) -> Move {
        let offset: i8 = match self.result {
            RoundResult::Win => 1,
            RoundResult::Draw => 0,
            RoundResult::Loss => -1,
        };
        let mut a: i8 = i8::from(self.opponent) + offset;
        if a < 0 {
            a += MODULO;
        }
        if a >= MODULO {
            a -= MODULO;
        }
        Move::from(a)
    }
}

impl Round for RoundPart2 {
    fn points(&self) -> u32 {
        self.my_move().usage_points() + self.result.points()
    }
}

impl RoundPart2 {
    pub fn parse(letters: &Letters) -> Result<Self, &'static str> {
        Ok(Self {
            opponent: Move::parse(&letters.0)?,
            result: RoundResult::parse(&letters.1)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::{parse_lines, solve, tests::INPUT};
    use super::*;

    #[test]
    fn solution() {
        let letters = parse_lines(INPUT.lines()).unwrap();
        let solution = solve(&letters, RoundPart2::parse).unwrap();
        assert_eq!(solution, 12);
    }
}
