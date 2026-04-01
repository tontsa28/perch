use std::ops::Not;

use crate::{bitboard::Bitboard, error::Error, mov::PieceKind};

const WHITE_PIECES: &str = "PNBRQK";
const BLACK_PIECES: &str = "pnbrqk";

#[derive(Debug, Clone)]
pub(crate) struct Board {
    pieces: [Bitboard; 12],
    white: Bitboard,
    black: Bitboard,
    occupied: Bitboard,
}

impl Board {
    pub fn new() -> Self {
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
            white: Bitboard(0xffff),
            black: Bitboard(0xffff_0000_0000_0000),
            occupied: Bitboard(0xffff_0000_0000_ffff),
        }
    }

    #[inline]
    fn bit_is_set(bb: u64, sq: u8) -> bool {
        ((bb >> sq) & 1) != 0
    }

    #[inline]
    fn sq(file: i8, rank: i8) -> Option<u8> {
        if (0..8).contains(&file) && (0..8).contains(&rank) {
            Some((rank as u8) * 8 + (file as u8))
        } else {
            None
        }
    }

    #[inline]
    fn file_rank(sq: u8) -> (i8, i8) {
        ((sq % 8) as i8, (sq / 8) as i8)
    }

    #[inline]
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
        let (f, r) = Self::file_rank(target_sq);
        let knights = match by {
            Color::White => self.pieces[1].0,
            Color::Black => self.pieces[7].0,
        };

        const OFFSETS: [(i8, i8); 8] = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        OFFSETS.into_iter().any(|(df, dr)| {
            Self::sq(f + df, r + dr)
                .map(|sq| Self::bit_is_set(knights, sq))
                .unwrap_or(false)
        })
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

                if let Some(sq) = Self::sq(f + df, r + dr) {
                    if Self::bit_is_set(king, sq) {
                        return true;
                    }
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

    pub(crate) fn occupied_bitboard(&self) -> Bitboard {
        self.occupied
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

    pub(crate) fn piece_at(&self, target_sq: u8) -> Option<(Color, PieceKind)> {
        assert!(target_sq < 64);
        let mask = 1u64 << target_sq;

        let kinds = &[
            PieceKind::Pawn,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
            PieceKind::Queen,
            PieceKind::King,
        ];

        for &color in &[Color::White, Color::Black] {
            for &kind in kinds {
                let idx = Self::bitboard_index(color, kind);
                if (self.pieces[idx].0 & mask) != 0 {
                    return Some((color, kind));
                }
            }
        }

        None
    }

    pub(crate) fn remove_piece(&mut self, color: Color, kind: PieceKind, target_sq: u8) {
        assert!(target_sq < 64);
        let mask = 1u64 << target_sq;
        let idx = Self::bitboard_index(color, kind);
        self.pieces[idx].0 &= !mask;

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

        match color {
            Color::White => self.white.0 |= mask,
            Color::Black => self.black.0 |= mask,
        }

        self.occupied.0 |= mask;
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(pos: &str) -> Result<Self, Self::Error> {
        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        let mut pieces = [Bitboard(0); 12];
        let mut white = Bitboard(0);
        let mut black = Bitboard(0);
        let mut occupied = Bitboard(0);

        for c in pos.chars() {
            if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as u8;
            } else if c.is_ascii_alphabetic() {
                match c {
                    'P' => pieces[0].0 |= 1u64 << (rank * 8 + file),
                    'N' => pieces[1].0 |= 1u64 << (rank * 8 + file),
                    'B' => pieces[2].0 |= 1u64 << (rank * 8 + file),
                    'R' => pieces[3].0 |= 1u64 << (rank * 8 + file),
                    'Q' => pieces[4].0 |= 1u64 << (rank * 8 + file),
                    'K' => pieces[5].0 |= 1u64 << (rank * 8 + file),
                    'p' => pieces[6].0 |= 1u64 << (rank * 8 + file),
                    'n' => pieces[7].0 |= 1u64 << (rank * 8 + file),
                    'b' => pieces[8].0 |= 1u64 << (rank * 8 + file),
                    'r' => pieces[9].0 |= 1u64 << (rank * 8 + file),
                    'q' => pieces[10].0 |= 1u64 << (rank * 8 + file),
                    'k' => pieces[11].0 |= 1u64 << (rank * 8 + file),
                    _ => return Err("invalid character in FEN: {c}")?,
                }

                if WHITE_PIECES.contains(c) {
                    white.0 |= 1u64 << (rank * 8 + file);
                }
                if BLACK_PIECES.contains(c) {
                    black.0 |= 1u64 << (rank * 8 + file);
                }
                occupied = white | black;

                file += 1;
            } else if c == '/' {
                rank -= 1;
                file = 0;
            }
        }

        Ok(Self {
            pieces,
            white,
            black,
            occupied,
        })
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
