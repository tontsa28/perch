use std::ops::BitOr;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bitboard(pub(crate) u64);

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
