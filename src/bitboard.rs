use std::{fmt::Display, ops::BitOr};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bitboard(pub(crate) u64);

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank * 8 + file;
                let bit = (self.0 >> sq) & 1;
                write!(f, "{}", bit)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
