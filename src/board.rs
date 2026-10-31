use std::ops::{Index};

use crate::piece::Class;
use crate::piece::Piece;
use crate::neighbor::Neighbor;
use crate::location::Location;

pub struct Board {
    pub space: [Option<Piece>; Self::SPACE_COUNT],
}

impl<I> Index<I> for Board 
where
    I: Into<usize>
{
    type Output = Option<Piece>;
    fn index(&self, index: I) -> &Self::Output {
        &self.space[index.into()]
    }
}

impl Board {
    const SPACE_COUNT_0: usize = Self::DIAGONAL;
    const SPACE_COUNT_1: usize = Self::DIAGONAL;
    const SPACE_COUNT_2: usize = 8;
    pub const DIAGONAL: usize = 32;
    const SPACE_COUNT: usize = Self::SPACE_COUNT_0 * Self::SPACE_COUNT_1 * Self::SPACE_COUNT_2;
    pub fn compute_moves_for_grasshopper(&self, starting_location: Location) {
        for neighbor in Neighbor::all() {
            let mut diagonal = 0;
            let mut location = starting_location;
            while diagonal < Self::DIAGONAL && self[location] != None {
                location = location.neighbor(neighbor);
                diagonal += 1;
            }
            assert!(diagonal < Board::DIAGONAL);
            if diagonal > 1 {
            
            }
        }
    }
}
