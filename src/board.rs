use std::{fmt::Display, ops::Not};

use crate::{
    attacks::KNIGHT_ATTACKS,
    bitboard::Bitboard,
    error::Error,
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

        for sq in 8..16 {
            squares[sq] = PieceOnSquare::WhitePawn;
        }

        for sq in 48..56 {
            squares[sq] = PieceOnSquare::BlackPawn;
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
                Bitboard(0x10),
                Bitboard(0x8),
                Bitboard(0x00ff_0000_0000_0000),
                Bitboard(0x4200_0000_0000_0000),
                Bitboard(0x2400_0000_0000_0000),
                Bitboard(0x8100_0000_0000_0000),
                Bitboard(0x1000_0000_0000_0000),
                Bitboard(0x0800_0000_0000_0000),
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
    fn sq(file: i8, rank: i8) -> Option<u8> {
        if (0..8).contains(&file) && (0..8).contains(&rank) {
            Some((rank as u8) * 8 + (file as u8))
        } else {
            None
        }
    }

    #[inline(always)]
    fn file_rank(sq: u8) -> (i8, i8) {
        ((sq % 8) as i8, (sq / 8) as i8)
    }

    #[inline(always)]
    fn bitboard_index(color: Color, kind: PieceKind) -> usize {
        let color_offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };
        color_offset + kind as usize
    }

    fn ray_hits_slider(
        &self,
        target_sq: u8,
        by: Color,
        directions: &[(i8, i8)],
        diagonal: bool,
    ) -> bool {
        let (f0, r0) = Self::file_rank(target_sq);
        let (bishops, rooks, queens) = match by {
            Color::White => (self.pieces[2].0, self.pieces[3].0, self.pieces[4].0),
            Color::Black => (self.pieces[8].0, self.pieces[9].0, self.pieces[10].0),
        };

        for &(df, dr) in directions {
            let mut f = f0 + df;
            let mut r = r0 + dr;

            while let Some(sq) = Self::sq(f, r) {
                let mask = 1u64 << sq;

                if (self.occupied.0 & mask) != 0 {
                    if diagonal {
                        if (bishops & mask) != 0 || (queens & mask) != 0 {
                            return true;
                        }
                    } else if (rooks & mask) != 0 || (queens & mask) != 0 {
                        return true;
                    }
                    break;
                }

                f += df;
                r += dr;
            }
        }

        false
    }

    fn is_attacked_by_pawn(&self, target_sq: u8, by: Color) -> bool {
        let (f, r) = Self::file_rank(target_sq);
        let pawns = match by {
            Color::White => self.pieces[0].0,
            Color::Black => self.pieces[6].0,
        };

        let candidate_squares = match by {
            Color::White => [Self::sq(f - 1, r - 1), Self::sq(f + 1, r - 1)],
            Color::Black => [Self::sq(f - 1, r + 1), Self::sq(f + 1, r + 1)],
        };

        candidate_squares
            .into_iter()
            .flatten()
            .any(|sq| Self::bit_is_set(pawns, sq))
    }

    fn is_attacked_by_knight(&self, target_sq: u8, by: Color) -> bool {
        let enemy_knights = self.piece_bitboard(by, PieceKind::Knight);
        KNIGHT_ATTACKS[target_sq as usize] & enemy_knights != Bitboard(0)
    }

    fn is_attacked_by_bishop_or_queen(&self, target_sq: u8, by: Color) -> bool {
        const DIAG: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        self.ray_hits_slider(target_sq, by, &DIAG, true)
    }

    fn is_attacked_by_rook_or_queen(&self, target_sq: u8, by: Color) -> bool {
        const ORTHO: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        self.ray_hits_slider(target_sq, by, &ORTHO, false)
    }

    fn is_attacked_by_king(&self, target_sq: u8, by: Color) -> bool {
        let (f, r) = Self::file_rank(target_sq);
        let king = match by {
            Color::White => self.pieces[5].0,
            Color::Black => self.pieces[11].0,
        };

        for df in -1..=1 {
            for dr in -1..=1 {
                if df == 0 && dr == 0 {
                    continue;
                }

                if let Some(sq) = Self::sq(f + df, r + dr)
                    && Self::bit_is_set(king, sq)
                {
                    return true;
                }
            }
        }

        false
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
        self.is_attacked_by_pawn(target_sq, by)
            || self.is_attacked_by_knight(target_sq, by)
            || self.is_attacked_by_king(target_sq, by)
            || self.is_attacked_by_bishop_or_queen(target_sq, by)
            || self.is_attacked_by_rook_or_queen(target_sq, by)
    }

    pub(crate) fn is_empty(&self, target_sq: u8) -> bool {
        assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        (self.occupied.0 & mask) == 0
    }

    pub(crate) fn has_friend(&self, target_sq: u8, color: Color) -> bool {
        assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        (self.color_bitboard(color).0 & mask) != 0
    }

    pub(crate) fn has_enemy(&self, target_sq: u8, color: Color) -> bool {
        assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        (self.color_bitboard(!color).0 & mask) != 0
    }

    pub(crate) fn piece_at(&self, target_sq: u8) -> PieceOnSquare {
        assert!(target_sq < 64);
        self.squares[target_sq as usize]
    }

    pub(crate) fn remove_piece(&mut self, color: Color, kind: PieceKind, target_sq: u8) {
        assert!(target_sq < 64);
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
        assert!(target_sq < 64);
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

    pub(crate) fn evaluate_material(&self) -> i32 {
        let mut score = 0;
        let values = [100, 320, 330, 500, 900, 0];

        for (i, &bb) in self.pieces[0..=5].iter().enumerate() {
            score += (bb.0.count_ones() as i32) * values[i];
        }

        for (i, &bb) in self.pieces[6..=11].iter().enumerate() {
            score -= (bb.0.count_ones() as i32) * values[i];
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
