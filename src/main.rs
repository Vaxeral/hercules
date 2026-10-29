// Hive is a two player game where players try to surround their opponents
// queen bee.  Players start with the same pieces each game.  A piece is either 
// black or white.  A piece is one of eight insect types.  The insect types are
// ant, beetle, grasshopper, ladybug, mosquito, pillbug, queen, spider.  A
// player starts with 3 ants, 2 beetles, 3 grasshoppers, 1 ladybug, 1 mosquito,
// 1 pillbug, 1 queen, and 2 spiders.
// Players start the game by taking turns placing pieces.  The pieces usually
// must only touch pieces of the same color, but the first turn is the exception
// to this rule.  You cannot place your bee on the first turn and you must place
// your queen bee no later than your fourth turn.

fn main() {
        println!("Hi. {:?}", Piece::WA1);
}

#[derive(Debug)]
enum Piece {
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

struct Board {
        space: [Option<Piece>; Board::SPACE_COUNT],
}

impl Board {
        const SPACE_COUNT_X: usize = 32;
        const SPACE_COUNT_Y: usize = 32;
        const SPACE_COUNT_Z: usize = 8;
        const SPACE_COUNT: usize
                = Board::SPACE_COUNT_X
                * Board::SPACE_COUNT_Y
                * Board::SPACE_COUNT_Z;
}

