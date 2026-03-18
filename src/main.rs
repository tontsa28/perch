mod bitboard;
mod uci;

use crate::uci::Uci;

fn main() {
    let mut uci = Uci::new();
    uci.run();
}
