use std::ops::{Add, Sub, AddAssign, SubAssign};
use crate::board::Board;
use crate::neighbor::Neighbor;

#[derive(Debug, Clone, Copy)]
pub struct Location(pub isize, pub isize);

impl Location {
    fn all() -> [Location; 6] {
        [
            Self(  0, -1),
            Self(  1, -1),
            Self(  1,  0),
            Self(  0,  1),
            Self( -1,  1),
            Self( -1,  0)
        ]    
    }
    pub fn neighbor(self, neighbor: Neighbor) -> Location {
        self + Location::all()[neighbor as usize]
    }
}

impl Add for Location {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Location(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Location {
    type Output = Location;
    fn sub(self, other: Self) -> Self::Output {
        Location(self.0 - other.0, self.1 - other.1)
    }
}

impl AddAssign for Location {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl SubAssign for Location {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl From<Location> for usize {
    fn from(location: Location) -> usize {
        let x: usize = location.0.rem_euclid(Board::DIAGONAL as isize) as usize;
        let y: usize = location.1.rem_euclid(Board::DIAGONAL as isize) as usize;
        x + y * Board::DIAGONAL
    }
}
