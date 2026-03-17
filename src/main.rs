mod uci;

use crate::uci::Uci;

fn main() {
    let uci = Uci::new();
    uci.run();
}
