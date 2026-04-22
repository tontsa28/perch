use std::{fmt::Display, ops::Not};

use crate::{
    attacks::{
        BLACK_PAWN_ATTACKS, E, KING_ATTACKS, KNIGHT_ATTACKS, N, NE, NW, RAYS, S, SE, SW, W,
        WHITE_PAWN_ATTACKS,
    },
    bitboard::Bitboard,
    error::Error,
    evals::MG,
    piece::{PieceKind, PieceOnSquare, parse_piece},
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Board {
    pieces: [Bitboard; 12],
    squares: [PieceOnSquare; 64],
    white: Bitboard,
    black: Bitboard,
    occupied: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [PieceOnSquare::Empty; 64];

        squares[0] = PieceOnSquare::WhiteRook;
        squares[1] = PieceOnSquare::WhiteKnight;
        squares[2] = PieceOnSquare::WhiteBishop;
        squares[3] = PieceOnSquare::WhiteQueen;
        squares[4] = PieceOnSquare::WhiteKing;
        squares[5] = PieceOnSquare::WhiteBishop;
        squares[6] = PieceOnSquare::WhiteKnight;
        squares[7] = PieceOnSquare::WhiteRook;

        for sq in squares.iter_mut().take(16).skip(8) {
            *sq = PieceOnSquare::WhitePawn;
        }

        for sq in squares.iter_mut().take(56).skip(48) {
            *sq = PieceOnSquare::BlackPawn;
        }

        squares[56] = PieceOnSquare::BlackRook;
        squares[57] = PieceOnSquare::BlackKnight;
        squares[58] = PieceOnSquare::BlackBishop;
        squares[59] = PieceOnSquare::BlackQueen;
        squares[60] = PieceOnSquare::BlackKing;
        squares[61] = PieceOnSquare::BlackBishop;
        squares[62] = PieceOnSquare::BlackKnight;
        squares[63] = PieceOnSquare::BlackRook;

        Self {
            pieces: [
                Bitboard(0xff00),
                Bitboard(0x42),
                Bitboard(0x24),
                Bitboard(0x81),
                Bitboard(0x8),
                Bitboard(0x10),
                Bitboard(0x00ff_0000_0000_0000),
                Bitboard(0x4200_0000_0000_0000),
                Bitboard(0x2400_0000_0000_0000),
                Bitboard(0x8100_0000_0000_0000),
                Bitboard(0x0800_0000_0000_0000),
                Bitboard(0x1000_0000_0000_0000),
            ],
            squares,
            white: Bitboard(0xffff),
            black: Bitboard(0xffff_0000_0000_0000),
            occupied: Bitboard(0xffff_0000_0000_ffff),
        }
    }

    #[inline(always)]
    fn bit_is_set(bb: u64, sq: u8) -> bool {
        ((bb >> sq) & 1) != 0
    }

    #[inline(always)]
    fn lsb_sq(bb: u64) -> u8 {
        bb.trailing_zeros() as u8
    }

    #[inline(always)]
    fn msb_sq(bb: u64) -> u8 {
        (63 - bb.leading_zeros()) as u8
    }

    #[inline(always)]
    fn pop_lsb(bb: &mut u64) -> u8 {
        let sq = bb.trailing_zeros() as u8;
        *bb &= *bb - 1;
        sq
    }

    #[inline(always)]
    fn bitboard_index(color: Color, kind: PieceKind) -> usize {
        let color_offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };
        color_offset + kind as usize
    }

    #[inline(always)]
    fn first_blocker_on_ray(occupied: u64, ray: u64, increasing: bool) -> Option<u8> {
        let blockers = occupied & ray;
        if blockers == 0 {
            None
        } else if increasing {
            Some(Self::lsb_sq(blockers))
        } else {
            Some(Self::msb_sq(blockers))
        }
    }

    fn is_attacked_by_pawn(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);
        let pawns = self.piece_bitboard(by, PieceKind::Pawn);
        let mask = match by {
            Color::White => BLACK_PAWN_ATTACKS[target_sq as usize],
            Color::Black => WHITE_PAWN_ATTACKS[target_sq as usize],
        };
        (mask & pawns) != Bitboard(0)
    }

    fn is_attacked_by_knight(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);
        let knights = self.piece_bitboard(by, PieceKind::Knight);
        (KNIGHT_ATTACKS[target_sq as usize] & knights) != Bitboard(0)
    }

    fn is_attacked_by_bishop_or_queen(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);
        let (bishops, queens) = match by {
            Color::White => (self.pieces[2].0, self.pieces[4].0),
            Color::Black => (self.pieces[8].0, self.pieces[10].0),
        };
        let sliders = bishops | queens;

        let rays = &RAYS[target_sq as usize];

        let checks = [
            (rays[NE], true),
            (rays[NW], true),
            (rays[SE], false),
            (rays[SW], false),
        ];

        for (ray, increasing) in checks {
            if let Some(blocker_sq) = Self::first_blocker_on_ray(self.occupied.0, ray, increasing)
                && Self::bit_is_set(sliders, blocker_sq)
            {
                return true;
            }
        }

        false
    }

    fn is_attacked_by_rook_or_queen(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);
        let (rooks, queens) = match by {
            Color::White => (self.pieces[3].0, self.pieces[4].0),
            Color::Black => (self.pieces[9].0, self.pieces[10].0),
        };
        let sliders = rooks | queens;

        let rays = &RAYS[target_sq as usize];

        let checks = [
            (rays[N], true),
            (rays[S], false),
            (rays[E], true),
            (rays[W], false),
        ];

        for (ray, increasing) in checks {
            if let Some(blocker_sq) = Self::first_blocker_on_ray(self.occupied.0, ray, increasing)
                && Self::bit_is_set(sliders, blocker_sq)
            {
                return true;
            }
        }

        false
    }

    fn is_attacked_by_king(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);
        let king = self.piece_bitboard(by, PieceKind::King);
        (KING_ATTACKS[target_sq as usize] & king) != Bitboard(0)
    }

    pub(crate) fn piece_bitboard(&self, color: Color, kind: PieceKind) -> Bitboard {
        self.pieces[Self::bitboard_index(color, kind)]
    }

    pub(crate) fn color_bitboard(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    pub(crate) fn piece_square(&self, color: Color, kind: PieceKind) -> u8 {
        self.piece_bitboard(color, kind).0.trailing_zeros() as u8
    }

    pub(crate) fn is_square_attacked(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);
        self.is_attacked_by_pawn(target_sq, by)
            || self.is_attacked_by_knight(target_sq, by)
            || self.is_attacked_by_king(target_sq, by)
            || self.is_attacked_by_bishop_or_queen(target_sq, by)
            || self.is_attacked_by_rook_or_queen(target_sq, by)
    }

    pub(crate) fn is_empty(&self, target_sq: u8) -> bool {
        debug_assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        (self.occupied.0 & mask) == 0
    }

    pub(crate) fn has_friend(&self, target_sq: u8, color: Color) -> bool {
        debug_assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        (self.color_bitboard(color).0 & mask) != 0
    }

    pub(crate) fn has_enemy(&self, target_sq: u8, color: Color) -> bool {
        debug_assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        (self.color_bitboard(!color).0 & mask) != 0
    }

    pub(crate) fn piece_at(&self, target_sq: u8) -> PieceOnSquare {
        debug_assert!(target_sq < 64);
        self.squares[target_sq as usize]
    }

    pub(crate) fn remove_piece(&mut self, color: Color, kind: PieceKind, target_sq: u8) {
        debug_assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        let idx = Self::bitboard_index(color, kind);
        self.pieces[idx].0 &= !mask;
        self.squares[target_sq as usize] = PieceOnSquare::Empty;

        match color {
            Color::White => self.white.0 &= !mask,
            Color::Black => self.black.0 &= !mask,
        }

        self.occupied.0 &= !mask;
    }

    pub(crate) fn add_piece(&mut self, color: Color, kind: PieceKind, target_sq: u8) {
        debug_assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        let idx = Self::bitboard_index(color, kind);
        self.pieces[idx].0 |= mask;
        self.squares[target_sq as usize] = PieceOnSquare::from((color, kind));

        match color {
            Color::White => self.white.0 |= mask,
            Color::Black => self.black.0 |= mask,
        }

        self.occupied.0 |= mask;
    }

    pub(crate) fn evaluate_material_pst(&self) -> i32 {
        let mut score = 0;
        const VALUES: [i32; 6] = [100, 320, 330, 500, 900, 0];

        for i in 0..=5 {
            let mut bb = self.pieces[i].0;
            let value = &VALUES[i];
            let pst = &MG[i];

            while bb != 0 {
                let sq = Self::pop_lsb(&mut bb) as usize;
                score += value + pst[sq ^ 56];
            }
        }

        for i in 6..=11 {
            let mut bb = self.pieces[i].0;
            let value = &VALUES[i - 6];
            let pst = &MG[i - 6];

            while bb != 0 {
                let sq = Self::pop_lsb(&mut bb) as usize;
                score -= value + pst[sq];
            }
        }

        score
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(pos: &str) -> Result<Self, Self::Error> {
        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        let mut pieces = [Bitboard(0); 12];
        let mut squares = [PieceOnSquare::Empty; 64];
        let mut white = Bitboard(0);
        let mut black = Bitboard(0);
        let mut occupied = Bitboard(0);

        for c in pos.chars() {
            if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as u8;
            } else if c == '/' {
                rank -= 1;
                file = 0;
            } else if let Some((ps, color, kind)) = parse_piece(c) {
                let sq = rank * 8 + file;
                let mask = 1u64 << sq;

                squares[sq as usize] = ps;
                pieces[Self::bitboard_index(color, kind)].0 |= mask;

                match color {
                    Color::White => white.0 |= mask,
                    Color::Black => black.0 |= mask,
                }
                occupied.0 |= mask;

                file += 1;
            } else {
                return Err("invalid character in FEN")?;
            }
        }

        Ok(Self {
            pieces,
            squares,
            white,
            black,
            occupied,
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BORDER: &str = "+---+---+---+---+---+---+---+---+";

        for rank in (0..8).rev() {
            writeln!(f, "{BORDER}")?;
            write!(f, "|")?;

            for file in 0..8 {
                let sq = rank * 8 + file;
                let glyph = char::from(self.piece_at(sq));

                write!(f, " {} |", glyph)?;
            }

            writeln!(f, " {}", rank + 1)?;
        }

        writeln!(f, "{BORDER}")?;
        write!(f, "  a   b   c   d   e   f   g   h  ")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err("invalid color")?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(piece_placement: &str) -> Board {
        Board::try_from(piece_placement).expect("valid piece placement")
    }

    #[test]
    fn starting_position_is_symmetric() {
        // Both sides are mirror images of each other, so the net score must be zero.
        assert_eq!(Board::new().evaluate_material_pst(), 0);
    }

    #[test]
    fn extra_white_pawn_scores_positive() {
        // Starting position with black's a-pawn removed — white is up one pawn.
        assert!(board("rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").evaluate_material_pst() > 0);
    }

    #[test]
    fn extra_black_pawn_scores_negative() {
        // Starting position with white's a-pawn removed — black is up one pawn.
        assert!(board("rnbqkbnr/pppppppp/8/8/8/8/1PPPPPPP/RNBQKBNR").evaluate_material_pst() < 0);
    }
}
