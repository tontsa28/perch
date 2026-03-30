use std::num::NonZeroU16;
use std::result::Result as StdResult;

use crate::{
    board::{Board, Color},
    error::Error,
};

#[derive(Debug)]
pub(crate) struct Position {
    board: Board,
    turn: Color,
    castling: u8,
    en_passant: Option<u8>,
    halfmoves: u16,
    fullmoves: NonZeroU16,
}

impl Position {
    const WK: u8 = 1 << 0;
    const WQ: u8 = 1 << 1;
    const BK: u8 = 1 << 2;
    const BQ: u8 = 1 << 3;

    pub(crate) fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
            castling: 0xf,
            en_passant: None,
            halfmoves: 0,
            fullmoves: NonZeroU16::MIN,
        }
    }

    fn can_castle_kingside(&self) -> bool {
        match self.turn {
            Color::White => self.castling & Self::WK != 0,
            Color::Black => self.castling & Self::BK != 0,
        }
    }

    fn can_castle_queenside(&self) -> bool {
        match self.turn {
            Color::White => self.castling & Self::WQ != 0,
            Color::Black => self.castling & Self::BQ != 0,
        }
    }

    fn is_check(&self, color: Color) -> bool {
        let king_sq = self.board.king_square(color);
        let attacker = !color;
        self.board.is_square_attacked(king_sq, attacker)
    }
}

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(value: &str) -> StdResult<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let pos_str = *parts.get(0).unwrap();
        let turn_str = *parts.get(1).unwrap();
        let castling_str = *parts.get(2).unwrap();
        let en_passant_str = *parts.get(3).unwrap();
        let halfmoves_str = *parts.get(4).unwrap();
        let fullmoves_str = *parts.get(5).unwrap();

        let mut castling: u8 = 0;
        if castling_str.contains('K') {
            castling |= 1u8 << 0;
        }
        if castling_str.contains('Q') {
            castling |= 1u8 << 1;
        }
        if castling_str.contains('k') {
            castling |= 1u8 << 2;
        }
        if castling_str.contains('q') {
            castling |= 1u8 << 3;
        }

        let mut en_passant: Option<u8> = None;
        if en_passant_str != "-" {
            let bytes = en_passant_str.as_bytes();

            let file = bytes[0] - b'a';
            let rank = bytes[1] - b'1';

            en_passant = Some(rank * 8 + file)
        }

        Ok(Self {
            board: Board::try_from(pos_str)?,
            turn: Color::try_from(turn_str)?,
            castling,
            en_passant,
            halfmoves: halfmoves_str.parse::<u16>()?,
            fullmoves: fullmoves_str.parse::<NonZeroU16>()?,
        })
    }
}
