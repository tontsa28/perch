mod bitboard;
mod board;
mod error;
mod mov;
mod piece;
mod position;
mod search;
mod uci;

use crate::uci::Uci;

fn main() {
    let mut uci = Uci::new();
    uci.run();
}
