use std::{fmt::Display, num::NonZeroU16};

use crate::{board::Color, error::Error, piece::PieceKind};

/// A representation of a chess move.
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
    /// Convert a square into UCI format (e.g. 0 -> a1).
    ///
    /// # Parameters
    /// - `sq`: Square index in the range 0..64.
    ///
    /// # Returns
    /// A `(file, rank)` pair as characters.
    #[inline(always)]
    fn sq_to_uci(sq: u8) -> (char, char) {
        let file = (b'a' + (sq % 8)) as char;
        let rank = (b'1' + (sq / 8)) as char;
        (file, rank)
    }

    /// Get the UCI promotion character of a piece.
    ///
    /// # Parameters
    /// - `p`: Piece kind to convert.
    ///
    /// # Returns
    /// The UCI promotion character (`n`, `b`, `r`, `q`), or `None` if invalid.
    #[inline(always)]
    fn promo_char(p: PieceKind) -> Option<char> {
        match p {
            PieceKind::Pawn | PieceKind::King => None,
            PieceKind::Knight => Some('n'),
            PieceKind::Bishop => Some('b'),
            PieceKind::Rook => Some('r'),
            PieceKind::Queen => Some('q'),
        }
    }

    /// Convert a UCI file character to a file number.
    ///
    /// # Parameters
    /// - `c`: ASCII file character (`a`..`h`).
    ///
    /// # Returns
    /// The file index (0..7), or `None` if invalid.
    #[inline(always)]
    fn file_char_to_u8(c: u8) -> Option<u8> {
        if (b'a'..=b'h').contains(&c) {
            Some(c - b'a')
        } else {
            None
        }
    }

    /// Convert a UCI rank character to a rank number.
    ///
    /// # Parameters
    /// - `c`: ASCII rank character (`1`..`8`).
    ///
    /// # Returns
    /// The rank index (0..7), or `None` if invalid.
    #[inline(always)]
    fn rank_char_to_u8(c: u8) -> Option<u8> {
        if (b'1'..=b'8').contains(&c) {
            Some(c - b'1')
        } else {
            None
        }
    }

    /// Convert a UCI promotion character into `PieceKind`.
    ///
    /// # Parameters
    /// - `c`: ASCII promotion character (`n`, `b`, `r`, `q`).
    ///
    /// # Returns
    /// The corresponding `PieceKind`, or `None` if invalid.
    #[inline(always)]
    fn promo_from_char(c: u8) -> Option<PieceKind> {
        match c {
            b'n' => Some(PieceKind::Knight),
            b'b' => Some(PieceKind::Bishop),
            b'r' => Some(PieceKind::Rook),
            b'q' => Some(PieceKind::Queen),
            _ => None,
        }
    }

    /// Check if this move is a promotion move.
    ///
    /// # Returns
    /// `true` if the move contains a promotion piece.
    pub(crate) fn is_promotion(&self) -> bool {
        self.promotion.is_some()
    }
}

impl Display for Move {
    /// Display the `Move` as an UCI-formatted move.
    ///
    /// # Parameters
    /// - `f`: Formatter sink.
    ///
    /// # Returns
    /// A formatting result.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (ff, fr) = Self::sq_to_uci(self.from);
        let (tf, tr) = Self::sq_to_uci(self.to);

        write!(f, "{ff}{fr}{tf}{tr}")?;

        if let Some(p) = self.promotion
            && let Some(c) = Self::promo_char(p)
        {
            write!(f, "{c}")?;
        }

        Ok(())
    }
}

impl TryFrom<&str> for Move {
    type Error = Error;

    /// Convert a UCI-formatted move into `Move`.
    ///
    /// # Parameters
    /// - `value`: UCI move string (e.g. `e2e4`, `e7e8q`).
    ///
    /// # Returns
    /// A parsed `Move`, or an error if the string is invalid.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let b = value.as_bytes();

        // If string length is not 4 or 5, it cannot be a valid UCI move
        if b.len() != 4 && b.len() != 5 {
            return Err("Invalid UCI move length")?;
        }

        // Get file-rank pairs for origin square and destionation square
        let ff = Self::file_char_to_u8(b[0]).ok_or("Invalid from file")?;
        let fr = Self::rank_char_to_u8(b[1]).ok_or("Invalid from rank")?;
        let tf = Self::file_char_to_u8(b[2]).ok_or("Invalid to file")?;
        let tr = Self::rank_char_to_u8(b[3]).ok_or("Invalid to rank")?;

        // Compute square indices from file-rank pairs
        let from = fr * 8 + ff;
        let to = tr * 8 + tf;

        // If string length is 5, extract its promotion piece
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

/// Undo a chess move.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Undo {
    pub(crate) captured: Option<(Color, PieceKind, u8)>,
    pub(crate) castling: u8,
    pub(crate) en_passant: Option<u8>,
    pub(crate) halfmoves: u16,
    pub(crate) fullmoves: NonZeroU16,
}

#[cfg(test)]
mod tests {
    use crate::piece::PieceKind;

    use super::Move;

    #[test]
    fn parse_simple_move() {
        let mv = Move::try_from("e2e4").unwrap();
        assert_eq!(mv.from, 12); // e2: rank 1, file 4 -> 1*8+4
        assert_eq!(mv.to, 28); // e4: rank 3, file 4 -> 3*8+4
        assert!(!mv.is_promotion());
        assert!(!mv.is_en_passant);
        assert!(!mv.is_castle_kingside);
        assert!(!mv.is_castle_queenside);
    }

    #[test]
    fn parse_all_four_promotion_pieces() {
        assert_eq!(
            Move::try_from("a7a8q").unwrap().promotion,
            Some(PieceKind::Queen)
        );
        assert_eq!(
            Move::try_from("a7a8r").unwrap().promotion,
            Some(PieceKind::Rook)
        );
        assert_eq!(
            Move::try_from("a7a8b").unwrap().promotion,
            Some(PieceKind::Bishop)
        );
        assert_eq!(
            Move::try_from("a7a8n").unwrap().promotion,
            Some(PieceKind::Knight)
        );
    }

    #[test]
    fn display_roundtrip() {
        for s in ["a1h8", "e2e4", "d7d8q", "b2b1n", "h7h8r"] {
            assert_eq!(Move::try_from(s).unwrap().to_string(), s);
        }
    }

    #[test]
    fn reject_wrong_length() {
        assert!(Move::try_from("e2e").is_err());
        assert!(Move::try_from("e2e4qq").is_err());
    }

    #[test]
    fn reject_bad_file() {
        assert!(Move::try_from("i2e4").is_err());
        assert!(Move::try_from("e2z4").is_err());
    }

    #[test]
    fn reject_bad_rank() {
        assert!(Move::try_from("e0e4").is_err());
        assert!(Move::try_from("e2e9").is_err());
    }

    #[test]
    fn reject_king_and_pawn_as_promotion() {
        // Neither king nor pawn are valid promotion targets
        assert!(Move::try_from("e7e8k").is_err());
        assert!(Move::try_from("e7e8p").is_err());
        assert!(Move::try_from("e7e8x").is_err());
    }
}
