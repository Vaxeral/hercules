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

mod piece;
mod location;
mod board;
mod neighbor;

use piece::Piece;
use location::Location;
use board::Board;

fn main() {
    // just me messing around.
    let piece = Piece::WB2;
    let location = Location(0, 0);
    let board = Board {
        space: [None; _]
    };
    println!("Hello, world! {:?}", piece.ordinal());
    println!("Hello, world! {:?}", piece.class());
    board.compute_moves_for_grasshopper(location);
}
