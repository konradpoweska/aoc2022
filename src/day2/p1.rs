use super::common::{Letters, Move, Round, RoundResult};

#[derive(Debug, PartialEq)]
pub struct RoundPart1 {
    opponent: Move,
    me: Move,
}

impl RoundPart1 {
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
    pub fn parse(letters: &Letters) -> Result<Self, &'static str> {
        Ok(Self {
            opponent: Move::parse(&letters.0)?,
            me: Move::parse(&letters.1)?,
        })
    }
}

impl Round for RoundPart1 {
    fn points(&self) -> u32 {
        self.result().points() + self.me.usage_points()
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::{parse_lines, solve, tests::INPUT};
    use super::*;

    #[test]
    fn parser() {
        let letters = parse_lines(INPUT.lines()).unwrap();
        let rounds: Vec<RoundPart1> = letters
            .iter()
            .map(|l| RoundPart1::parse(l).unwrap())
            .collect();

        assert_eq!(
            rounds,
            vec![
                RoundPart1 {
                    opponent: Move::Rock,
                    me: Move::Paper
                },
                RoundPart1 {
                    opponent: Move::Paper,
                    me: Move::Rock
                },
                RoundPart1 {
                    opponent: Move::Scissors,
                    me: Move::Scissors
                },
            ]
        );
    }

    #[test]
    fn solution() {
        let letters = parse_lines(INPUT.lines()).unwrap();
        let solution = solve(&letters, RoundPart1::parse).unwrap();
        assert_eq!(solution, 15);
    }
}
