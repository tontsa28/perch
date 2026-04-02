use std::{fmt::Display, num::NonZeroU16};

use crate::{board::Color, error::Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Move {
    pub(crate) from: u8,
    pub(crate) to: u8,
    pub(crate) promotion: Option<PieceKind>,
    pub(crate) is_en_passant: bool,
    pub(crate) is_castle_kingside: bool,
    pub(crate) is_castle_queenside: bool,
}

impl Move {
    #[inline]
    fn sq_to_uci(sq: u8) -> (char, char) {
        let file = (b'a' + (sq % 8)) as char;
        let rank = (b'1' + (sq / 8)) as char;
        (file, rank)
    }

    #[inline]
    fn promo_char(p: PieceKind) -> Option<char> {
        match p {
            PieceKind::Pawn | PieceKind::King => None,
            PieceKind::Knight => Some('n'),
            PieceKind::Bishop => Some('b'),
            PieceKind::Rook => Some('r'),
            PieceKind::Queen => Some('q'),
        }
    }

    #[inline]
    fn file_char_to_u8(c: u8) -> Option<u8> {
        if (b'a'..=b'h').contains(&c) {
            Some(c - b'a')
        } else {
            None
        }
    }

    #[inline]
    fn rank_char_to_u8(c: u8) -> Option<u8> {
        if (b'1'..=b'8').contains(&c) {
            Some(c - b'1')
        } else {
            None
        }
    }

    #[inline]
    fn promo_from_char(c: u8) -> Option<PieceKind> {
        match c {
            b'n' => Some(PieceKind::Knight),
            b'b' => Some(PieceKind::Bishop),
            b'r' => Some(PieceKind::Rook),
            b'q' => Some(PieceKind::Queen),
            _ => None,
        }
    }

    pub(crate) fn is_promotion(&self) -> bool {
        self.promotion.is_some()
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (ff, fr) = Self::sq_to_uci(self.from);
        let (tf, tr) = Self::sq_to_uci(self.to);

        write!(f, "{ff}{fr}{tf}{tr}")?;

        if let Some(p) = self.promotion {
            if let Some(c) = Self::promo_char(p) {
                write!(f, "{c}")?;
            }
        }

        Ok(())
    }
}

impl TryFrom<&str> for Move {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let b = value.as_bytes();

        if b.len() != 4 && b.len() != 5 {
            return Err("Invalid UCI move length")?;
        }

        let ff = Self::file_char_to_u8(b[0]).ok_or("Invalid from file")?;
        let fr = Self::rank_char_to_u8(b[1]).ok_or("Invalid from rank")?;
        let tf = Self::file_char_to_u8(b[2]).ok_or("Invalid to file")?;
        let tr = Self::rank_char_to_u8(b[3]).ok_or("Invalid to rank")?;

        let from = fr * 8 + ff;
        let to = tr * 8 + tf;

        let promotion = if b.len() == 5 {
            Some(Self::promo_from_char(b[4]).ok_or("Invalid promotion piece")?)
        } else {
            None
        };

        Ok(Self {
            from,
            to,
            promotion,
            is_en_passant: false,
            is_castle_kingside: false,
            is_castle_queenside: false,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Undo {
    pub(crate) captured: Option<(Color, PieceKind, u8)>,
    pub(crate) castling: u8,
    pub(crate) en_passant: Option<u8>,
    pub(crate) halfmoves: u16,
    pub(crate) fullmoves: NonZeroU16,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PieceKind {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}
