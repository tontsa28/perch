use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct Bitboard(u64);

impl From<&str> for Bitboard {
    fn from(value: &str) -> Self {
        let pos = value.split_whitespace().next().unwrap();
        let mut rank: u8 = 7;
        let mut file: u8 = 8;
        let mut board: u64 = 0;

        for c in pos.chars() {
            if c.is_digit(10) {
                file -= c.to_digit(10).unwrap() as u8;
                board |= 0u64 << (rank * 8 + file);
            } else if c.is_ascii_alphabetic() {
                file -= 1;
                board |= 1u64 << (rank * 8 + file);
            } else if c == '/' {
                rank -= 1;
                file = 8;
            }
        }

        Self(board)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            writeln!(f)?;
            for file in (0..8).rev() {
                let sq = rank * 8 + file;
                let bit = (self.0 >> sq) & 1;
                write!(f, "{}", bit)?;
            }
        }
        Ok(())
    }
}
