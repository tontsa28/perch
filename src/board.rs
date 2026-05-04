use std::{fmt::Display, ops::Not};

use crate::{
    attacks::{
        BLACK_PAWN_ATTACKS, E, KING_ATTACKS, KNIGHT_ATTACKS, N, NE, NW, RAYS, S, SE, SW, W,
        WHITE_PAWN_ATTACKS,
    },
    bitboard::Bitboard,
    error::Error,
    evals::{EG, MG},
    piece::{PieceKind, PieceOnSquare, parse_piece},
};

/// A chessboard representation consisting of bitboards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Board {
    pieces: [Bitboard; 12],
    squares: [PieceOnSquare; 64],
    white: Bitboard,
    black: Bitboard,
    occupied: Bitboard,
}

impl Board {
    /// Initialize the standard chess starting position.
    pub(crate) fn new() -> Self {
        // Set all squares to empty
        let mut squares = [PieceOnSquare::Empty; 64];

        // Set white back rank
        squares[0] = PieceOnSquare::WhiteRook;
        squares[1] = PieceOnSquare::WhiteKnight;
        squares[2] = PieceOnSquare::WhiteBishop;
        squares[3] = PieceOnSquare::WhiteQueen;
        squares[4] = PieceOnSquare::WhiteKing;
        squares[5] = PieceOnSquare::WhiteBishop;
        squares[6] = PieceOnSquare::WhiteKnight;
        squares[7] = PieceOnSquare::WhiteRook;

        // Set white pawns
        for sq in squares.iter_mut().take(16).skip(8) {
            *sq = PieceOnSquare::WhitePawn;
        }

        // Set black pawns
        for sq in squares.iter_mut().take(56).skip(48) {
            *sq = PieceOnSquare::BlackPawn;
        }

        // Set black back rank
        squares[56] = PieceOnSquare::BlackRook;
        squares[57] = PieceOnSquare::BlackKnight;
        squares[58] = PieceOnSquare::BlackBishop;
        squares[59] = PieceOnSquare::BlackQueen;
        squares[60] = PieceOnSquare::BlackKing;
        squares[61] = PieceOnSquare::BlackBishop;
        squares[62] = PieceOnSquare::BlackKnight;
        squares[63] = PieceOnSquare::BlackRook;

        // Set bitboards to precomputed values
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

    /// Check if there is a piece on the given square.
    #[inline(always)]
    fn bit_is_set(bb: u64, sq: u8) -> bool {
        // Right shift bitboard integer sq times (so that square bit becomes LSB),
        // then compute AND against a 1-bit
        ((bb >> sq) & 1) != 0
    }

    /// Get the square with the smallest index.
    #[inline(always)]
    fn lsb_sq(bb: u64) -> u8 {
        // Count trailing zeros to get LSB square
        bb.trailing_zeros() as u8
    }

    /// Get the square with the largest index.
    #[inline(always)]
    fn msb_sq(bb: u64) -> u8 {
        // Count leading zeros to get MSB square
        (63 - bb.leading_zeros()) as u8
    }

    /// Pop the LSB of the bitboard.
    #[inline(always)]
    fn pop_lsb(bb: &mut u64) -> u8 {
        // Get LSB square, then remove it
        let sq = bb.trailing_zeros() as u8;
        *bb &= *bb - 1;

        sq
    }

    /// Get the index of a piece bitboard.
    #[inline(always)]
    fn bitboard_index(color: Color, kind: PieceKind) -> usize {
        let color_offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };
        color_offset + kind as usize
    }

    /// Get the first blocker on a ray.
    #[inline(always)]
    fn first_blocker_on_ray(occupied: u64, ray: u64, increasing: bool) -> Option<u8> {
        // Compute AND between occupied pieces and ray to find blockers
        let blockers = occupied & ray;

        if blockers == 0 {
            None
        } else if increasing {
            Some(Self::lsb_sq(blockers))
        } else {
            Some(Self::msb_sq(blockers))
        }
    }

    /// Check if a square is attacked by a pawn.
    fn is_attacked_by_pawn(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);

        // Get attacker pawns and possible pawn attacks
        let pawns = self.piece_bitboard(by, PieceKind::Pawn);
        let mask = match by {
            Color::White => BLACK_PAWN_ATTACKS[target_sq as usize],
            Color::Black => WHITE_PAWN_ATTACKS[target_sq as usize],
        };

        // Compute AND between pawn attacks and pawns to find attacking pawns
        (mask & pawns) != Bitboard(0)
    }

    /// Check if a square is attacked by a knight.
    fn is_attacked_by_knight(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);

        // Get attacker knights and possible knight attacks
        let knights = self.piece_bitboard(by, PieceKind::Knight);
        let mask = KNIGHT_ATTACKS[target_sq as usize];

        // Compute AND between knight attacks and knights to find attacking knights
        (mask & knights) != Bitboard(0)
    }

    /// Check if a square is attacked by a diagonally moving piece (bishop or queen).
    fn is_attacked_by_bishop_or_queen(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);

        // Get attacker bishops and queens, then compute OR to combine them
        let bishops = self.piece_bitboard(by, PieceKind::Bishop);
        let queens = self.piece_bitboard(by, PieceKind::Queen);
        let sliders = bishops | queens;

        // Get all rays, then filter out orthogonal rays
        let rays = &RAYS[target_sq as usize];
        let diagonals = [
            (rays[NE], true),
            (rays[NW], true),
            (rays[SE], false),
            (rays[SW], false),
        ];

        for (ray, increasing) in diagonals {
            // Check if first blocker is a diagonally moving slider piece
            if let Some(blocker_sq) = Self::first_blocker_on_ray(self.occupied.0, ray, increasing)
                && Self::bit_is_set(sliders.0, blocker_sq)
            {
                return true;
            }
        }

        false
    }

    /// Check if a square is attacked by an orthogonally moving piece (rook or queen).
    fn is_attacked_by_rook_or_queen(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);

        // Get attacker rooks and queens, then compute OR to combine them
        let rooks = self.piece_bitboard(by, PieceKind::Rook);
        let queens = self.piece_bitboard(by, PieceKind::Queen);
        let sliders = rooks | queens;

        // Get all rays, then filter out diagonal rays
        let rays = &RAYS[target_sq as usize];
        let orthogonals = [
            (rays[N], true),
            (rays[S], false),
            (rays[E], true),
            (rays[W], false),
        ];

        for (ray, increasing) in orthogonals {
            // Check if first blocker is an orthogonally moving slider piece
            if let Some(blocker_sq) = Self::first_blocker_on_ray(self.occupied.0, ray, increasing)
                && Self::bit_is_set(sliders.0, blocker_sq)
            {
                return true;
            }
        }

        false
    }

    /// Check if a square is attacked by a king.
    fn is_attacked_by_king(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);

        // Get attacker king and possible king attacks
        let king = self.piece_bitboard(by, PieceKind::King);
        let mask = KING_ATTACKS[target_sq as usize];

        // Compute AND between king attacks and king to find attacking king
        (mask & king) != Bitboard(0)
    }

    /// Get a piece bitboard by its color and kind.
    pub(crate) fn piece_bitboard(&self, color: Color, kind: PieceKind) -> Bitboard {
        self.pieces[Self::bitboard_index(color, kind)]
    }

    /// Get a color bitboard by its color.
    pub(crate) fn color_bitboard(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    /// Get the square of a king by its color.
    pub(crate) fn king_square(&self, color: Color) -> u8 {
        let king = self.piece_bitboard(color, PieceKind::King);

        // DEBUG: make sure there is exactly one king
        debug_assert_eq!(king.0.count_ones(), 1);

        // Find king square
        Self::lsb_sq(king.0)
    }

    /// Check if a square is attacked.
    pub(crate) fn is_square_attacked(&self, target_sq: u8, by: Color) -> bool {
        debug_assert!(target_sq < 64);

        self.is_attacked_by_pawn(target_sq, by)
            || self.is_attacked_by_knight(target_sq, by)
            || self.is_attacked_by_king(target_sq, by)
            || self.is_attacked_by_bishop_or_queen(target_sq, by)
            || self.is_attacked_by_rook_or_queen(target_sq, by)
    }

    /// Check if a square is empty.
    pub(crate) fn is_empty(&self, target_sq: u8) -> bool {
        debug_assert!(target_sq < 64);

        // Convert square into a bitboard
        let mask = 1u64 << target_sq;

        // Compute AND between square and occupied squares to find its occupancy
        (self.occupied.0 & mask) == 0
    }

    /// Check if a square contains a friendly piece.
    pub(crate) fn has_friend(&self, target_sq: u8, color: Color) -> bool {
        debug_assert!(target_sq < 64);

        // Convert square into a bitboard
        let mask = 1u64 << target_sq;

        // Compute AND between square and squares occupied by us to find its friendliness
        (self.color_bitboard(color).0 & mask) != 0
    }

    /// Check if a square contains an enemy piece.
    pub(crate) fn has_enemy(&self, target_sq: u8, color: Color) -> bool {
        debug_assert!(target_sq < 64);

        // Convert square into a bitboard
        let mask = 1u64 << target_sq;

        // Compute AND between square and squares occupied by our enemy to find its friendliness
        (self.color_bitboard(!color).0 & mask) != 0
    }

    /// Check if white or black has material beyond pawns and a king.
    pub(crate) fn has_non_pawns(&self, color: Color) -> bool {
        match color {
            Color::White => {
                self.pieces[1] | self.pieces[2] | self.pieces[3] | self.pieces[4] != Bitboard(0)
            }
            Color::Black => {
                self.pieces[7] | self.pieces[8] | self.pieces[9] | self.pieces[10] != Bitboard(0)
            }
        }
    }

    /// Get the piece on a square.
    pub(crate) fn piece_at(&self, target_sq: u8) -> PieceOnSquare {
        debug_assert!(target_sq < 64);

        self.squares[target_sq as usize]
    }

    /// Remove a piece from the board.
    pub(crate) fn remove_piece(&mut self, color: Color, kind: PieceKind, target_sq: u8) {
        debug_assert!(target_sq < 64);

        // Convert square into a bitboard
        let mask = 1u64 << target_sq;

        // Update piece bitboard and square
        let idx = Self::bitboard_index(color, kind);
        self.pieces[idx].0 &= !mask;
        self.squares[target_sq as usize] = PieceOnSquare::Empty;

        // Update color bitboard
        match color {
            Color::White => self.white.0 &= !mask,
            Color::Black => self.black.0 &= !mask,
        }

        // Update occupied bitboard
        self.occupied.0 &= !mask;
    }

    /// Add a piece to the board.
    pub(crate) fn add_piece(&mut self, color: Color, kind: PieceKind, target_sq: u8) {
        debug_assert!(target_sq < 64);

        // Convert square into a bitboard
        let mask = 1u64 << target_sq;

        // Update piece bitboard and square
        let idx = Self::bitboard_index(color, kind);
        self.pieces[idx].0 |= mask;
        self.squares[target_sq as usize] = PieceOnSquare::from((color, kind));

        // Update color bitboard
        match color {
            Color::White => self.white.0 |= mask,
            Color::Black => self.black.0 |= mask,
        }

        // Update occupied bitboard
        self.occupied.0 |= mask;
    }

    /// Evaluate the board based on material and piece-square tables (PSTs).
    pub(crate) fn evaluate_material_pst(&self) -> i32 {
        // Construct numerical piece values, phase weights and phase window size
        const VALUES: [i32; 6] = [100, 320, 330, 500, 900, 0];
        const PHASE_WEIGHTS: [i32; 6] = [0, 1, 1, 2, 4, 0]; // P, N, B, R, Q, K
        const TOTAL_PHASE: i32 = 24;

        let mut mg_score = 0;
        let mut eg_score = 0;
        let mut phase = 0;

        // Process white pieces
        for i in 0..=5 {
            // Get piece bitboard, piece value and PSTs
            let mut bb = self.pieces[i].0;
            let value = VALUES[i];
            let mg_pst = &MG[i];
            let eg_pst = &EG[i];

            while bb != 0 {
                // Take out LSB square
                let sq = Self::pop_lsb(&mut bb) as usize;

                // Update middlegame and endgame scores,
                // sq ^ 56 mirrors square index as PSTs are indexed from black's perspective
                mg_score += value + mg_pst[sq ^ 56];
                eg_score += value + eg_pst[sq ^ 56];
                phase += PHASE_WEIGHTS[i];
            }
        }

        // Process black pieces
        for i in 6..=11 {
            // Get piece bitboard, piece value and PSTs,
            // subtract 6 to match piece kind
            let mut bb = self.pieces[i].0;
            let value = VALUES[i - 6];
            let mg_pst = &MG[i - 6];
            let eg_pst = &EG[i - 6];

            while bb != 0 {
                // Take out LSB square
                let sq = Self::pop_lsb(&mut bb) as usize;

                // Update middlegame and endgame scores
                mg_score -= value + mg_pst[sq];
                eg_score -= value + eg_pst[sq];
                phase += PHASE_WEIGHTS[i - 6];
            }
        }

        // Cap phase value at TOTAL_PHASE
        let phase = phase.min(TOTAL_PHASE);

        // Compute evaluation while taking phase into account
        (mg_score * phase + eg_score * (TOTAL_PHASE - phase)) / TOTAL_PHASE
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    /// Convert the board part of a FEN string
    /// (e.g. `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR`)
    /// into `Board`.
    fn try_from(pos: &str) -> Result<Self, Self::Error> {
        // Process ranks from left to right (files a-h), starting from 8th rank
        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        // Set all bitboards and square to empty
        let mut pieces = [Bitboard(0); 12];
        let mut squares = [PieceOnSquare::Empty; 64];
        let mut white = Bitboard(0);
        let mut black = Bitboard(0);
        let mut occupied = Bitboard(0);

        for c in pos.chars() {
            if c.is_ascii_digit() {
                // Skip c files
                file += c.to_digit(10).unwrap() as u8;
            } else if c == '/' {
                // Go down by a rank and reset file to a
                rank -= 1;
                file = 0;
            } else if let Some((ps, color, kind)) = parse_piece(c) {
                // Compute square from rank and file, then convert it into a bitboard
                let sq = rank * 8 + file;
                let mask = 1u64 << sq;

                // Insert piece to square and update piece bitboard
                squares[sq as usize] = ps;
                pieces[Self::bitboard_index(color, kind)].0 |= mask;

                // Update color bitboard
                match color {
                    Color::White => white.0 |= mask,
                    Color::Black => black.0 |= mask,
                }

                // Update occupied bitboard
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
    /// Display the `Board` as an ASCII chessboard.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BORDER: &str = "+---+---+---+---+---+---+---+---+";

        // Process ranks in reverse order
        for rank in (0..8).rev() {
            writeln!(f, "{BORDER}")?;
            write!(f, "|")?;

            for file in 0..8 {
                // Compute square from rank and file, then convert potential piece into char
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

/// A piece color, white or black.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    /// Convert a color string into `Color`.
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
        // Both sides are mirror images of each other -> net score must be zero
        assert_eq!(Board::new().evaluate_material_pst(), 0);
    }

    #[test]
    fn extra_white_pawn_scores_positive() {
        // Starting position with black's a-pawn removed — white is up one pawn
        assert!(board("rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").evaluate_material_pst() > 0);
    }

    #[test]
    fn extra_black_pawn_scores_negative() {
        // Starting position with white's a-pawn removed — black is up one pawn
        assert!(board("rnbqkbnr/pppppppp/8/8/8/8/1PPPPPPP/RNBQKBNR").evaluate_material_pst() < 0);
    }
}
