mod bitboard;
mod board;
mod error;
mod position;
mod uci;

use crate::uci::Uci;

fn main() {
    let mut uci = Uci::new();
    uci.run();
}
