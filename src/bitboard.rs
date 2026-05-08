use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// A set of squares represented as an unsigned 64-bit integer.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Bitboard(u64);

impl Bitboard {
    pub(crate) const EMPTY: Self = Self(0);

    /// Create a new bitboard from an unsigned 64-bit integer.
    pub(crate) const fn new(bb: u64) -> Self {
        Bitboard(bb)
    }

    /// Check if there is a piece on the given square.
    #[inline(always)]
    pub(crate) fn bit_is_set(self, sq: u8) -> bool {
        // Right shift bitboard integer sq times (so that square bit becomes LSB),
        // then compute AND against a 1-bit
        ((self.0 >> sq) & 1) != 0
    }

    /// Get the square with the smallest index.
    #[inline(always)]
    pub(crate) fn lsb_sq(self) -> u8 {
        // Count trailing zeros to get LSB square
        self.0.trailing_zeros() as u8
    }

    /// Get the square with the largest index.
    #[inline(always)]
    pub(crate) fn msb_sq(self) -> u8 {
        // Count leading zeros to get MSB square
        (63 - self.0.leading_zeros()) as u8
    }

    /// Pop the LSB of the bitboard.
    #[inline(always)]
    pub(crate) fn pop_lsb(&mut self) -> u8 {
        // Get LSB square, then remove it
        let sq = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;

        sq
    }

    /// Returns `true` if the bitboard contains no pieces.
    #[inline(always)]
    pub(crate) fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the bitboard contains one or more pieces.
    #[inline(always)]
    pub(crate) fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    /// Returns the number of set bits in this bitboard.
    #[inline(always)]
    pub(crate) fn count_ones(self) -> u32 {
        self.0.count_ones()
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

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

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_is_set_and_empty_flags() {
        let bb = Bitboard::new(1u64 << 7);
        assert!(bb.bit_is_set(7));
        assert!(!bb.bit_is_set(6));
        assert!(!bb.is_empty());
        assert!(bb.is_not_empty());
        assert!(Bitboard::EMPTY.is_empty());
        assert!(!Bitboard::EMPTY.is_not_empty());
    }

    #[test]
    fn lsb_and_msb_are_correct() {
        let bb = Bitboard::new((1u64 << 5) | (1u64 << 12));
        assert_eq!(bb.lsb_sq(), 5);
        assert_eq!(bb.msb_sq(), 12);
    }

    #[test]
    fn pop_lsb_pops_in_order() {
        let mut bb = Bitboard::new((1u64 << 2) | (1u64 << 5) | (1u64 << 8));
        assert_eq!(bb.pop_lsb(), 2);
        assert_eq!(bb.pop_lsb(), 5);
        assert_eq!(bb.pop_lsb(), 8);
        assert!(bb.is_empty());
    }

    #[test]
    fn bit_ops_work() {
        let a = Bitboard::new(0b1010);
        let b = Bitboard::new(0b0110);
        assert_eq!(a & b, Bitboard::new(0b0010));
        assert_eq!(a | b, Bitboard::new(0b1110));

        let mut c = a;
        c &= b;
        assert_eq!(c, Bitboard::new(0b0010));

        let mut d = a;
        d |= b;
        assert_eq!(d, Bitboard::new(0b1110));
    }
}
