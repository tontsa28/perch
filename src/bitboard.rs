use std::ops::{BitAnd, BitOr};

/// A set of squares represented as an unsigned 64-bit integer.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Bitboard(pub(crate) u64);

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
