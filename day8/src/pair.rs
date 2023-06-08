use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Pair) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Pair {
    fn contains(&self, pair: &Pair) -> bool {
        (0..self.x).contains(&pair.x) && (0..self.y).contains(&pair.y)
    }
}

pub type Direction = Pair;

pub const NORTH: Direction = Pair { x: 0, y: -1 };
pub const EAST: Direction = Pair { x: 1, y: 0 };
pub const SOUTH: Direction = Pair { x: 0, y: 1 };
pub const WEST: Direction = Pair { x: -1, y: 0 };

pub struct CoordsIter {
    pub size: Pair,
    pub current: Option<Pair>,
    pub direction: Direction,
}

impl Iterator for CoordsIter {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        let new = current + self.direction;
        self.current = if self.size.contains(&new) {
            Some(new)
        } else {
            None
        };
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north() {
        let mut iter = CoordsIter {
            size: Pair { x: 3, y: 5 },
            current: Some(Pair { x: 2, y: 3 }),
            direction: NORTH,
        };
        for y in [2, 1, 0] {
            assert_eq!(iter.next(), Some(Pair { x: 2, y }));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn south() {
        let mut iter = CoordsIter {
            size: Pair { x: 3, y: 5 },
            current: Some(Pair { x: 1, y: 2 }),
            direction: SOUTH,
        };
        for y in [3, 4] {
            assert_eq!(iter.next(), Some(Pair { x: 1, y }));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn east() {
        let mut iter = CoordsIter {
            size: Pair { x: 3, y: 5 },
            current: Some(Pair { x: 0, y: 0 }),
            direction: EAST,
        };
        for x in [1, 2] {
            assert_eq!(iter.next(), Some(Pair { y: 0, x }))
        }
        assert_eq!(iter.next(), None);
    }
}
