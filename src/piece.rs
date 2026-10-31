#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    WA1 = 0,
    BA1 = 1,
    WB1 = 2,
    BB1 = 3,
    WG1 = 4,
    BG1 = 5,
    WL1 = 6,
    BL1 = 7,
    WM1 = 8,
    BM1 = 9,
    WP1 = 10,
    BP1 = 11,
    WQ1 = 12,
    BQ1 = 13,
    WS1 = 14,
    BS1 = 15,
    WA2 = 16,
    BA2 = 17,
    WB2 = 18,
    BB2 = 19,
    WG2 = 20,
    BG2 = 21,

    WS2 = 30,
    BS2 = 31,

    WA3 = 32,
    BA3 = 33,
        
    WG3 = 36,
    BG3 = 37
}

#[derive(Debug)]
pub enum Color {
    White,
    Black
}

#[derive(Debug, PartialEq)]
pub enum Class {
    Ant,
    Beetle,
    Grasshopper,
    Ladybug,
    Mosquito,
    Pillbug,
    Queen,
    Spider
}

impl Piece {
    pub fn color(self) -> Color {
        let value = self as usize;
        match value & 1 {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!()
        }
    }
    pub fn class(self) -> Class {
        let value = self as usize;
        match (value & 15) >> 1 {
            0 => Class::Ant,
            1 => Class::Beetle,
            2 => Class::Grasshopper,
            3 => Class::Ladybug,
            4 => Class::Mosquito,
            5 => Class::Pillbug,
            6 => Class::Queen,
            7 => Class::Spider,
            _ => unreachable!()
        }
    }
    pub fn ordinal(self) -> usize {
        let value = self as usize;
        value >> 4
    }
}
