use std::num::NonZeroU16;
use std::result::Result as StdResult;

use crate::{
    board::{Board, Color},
    error::Error,
    mov::{Move, PieceKind},
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
    fn push_promotion_set(moves: &mut Vec<Move>, from: u8, to: u8, is_en_passant: bool) {
        for promo in [
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
            PieceKind::Queen,
        ] {
            moves.push(Move {
                from,
                to,
                promotion: Some(promo),
                is_en_passant,
                is_castle_kingside: false,
                is_castle_queenside: false,
            });
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
        let king_sq = self.board.piece_square(color, PieceKind::King);
        let attacker = !color;
        self.board.is_square_attacked(king_sq, attacker)
    }

    fn gen_pawn_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let mut pawns = self.board.piece_bitboard(color, PieceKind::Pawn).0;

        let (push_delta, start_rank, promo_rank) = match color {
            Color::White => (1i8, 1i8, 7i8),
            Color::Black => (-1i8, 6i8, 0i8),
        };

        while pawns != 0 {
            let from = pawns.trailing_zeros() as u8;
            pawns &= pawns - 1;

            let (f, r) = Self::file_rank(from);

            if let Some(one_step) = Self::sq(f, r + push_delta) {
                if self.board.is_empty(one_step) {
                    let (_, to_rank) = Self::file_rank(one_step);

                    if to_rank == promo_rank {
                        Self::push_promotion_set(moves, from, one_step, false);
                    } else {
                        moves.push(Move {
                            from,
                            to: one_step,
                            promotion: None,
                            is_en_passant: false,
                            is_castle_kingside: false,
                            is_castle_queenside: false,
                        });

                        if r == start_rank {
                            if let Some(two_step) = Self::sq(f, r + 2 * push_delta) {
                                if self.board.is_empty(two_step) {
                                    moves.push(Move {
                                        from,
                                        to: two_step,
                                        promotion: None,
                                        is_en_passant: false,
                                        is_castle_kingside: false,
                                        is_castle_queenside: false,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            for df in [-1i8, 1i8] {
                if let Some(to) = Self::sq(f + df, r + push_delta) {
                    let (_, to_rank) = Self::file_rank(to);

                    if self.board.has_enemy(to, color) {
                        if to_rank == promo_rank {
                            Self::push_promotion_set(moves, from, to, false);
                        } else {
                            moves.push(Move {
                                from,
                                to,
                                promotion: None,
                                is_en_passant: false,
                                is_castle_kingside: false,
                                is_castle_queenside: false,
                            });
                        }
                        continue;
                    }

                    if self.en_passant == Some(to) {
                        moves.push(Move {
                            from,
                            to,
                            promotion: None,
                            is_en_passant: true,
                            is_castle_kingside: false,
                            is_castle_queenside: false,
                        });
                    }
                }
            }
        }
    }

    pub(crate) fn gen_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        self.gen_pawn_moves(self.turn, &mut moves);

        moves
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
