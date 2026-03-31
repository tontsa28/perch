use std::num::NonZeroU16;
use std::result::Result as StdResult;

use crate::{
    bitboard::Bitboard,
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

    fn gen_slider_moves(
        &self,
        color: Color,
        bitboard: Bitboard,
        directions: &[(i8, i8)],
        moves: &mut Vec<Move>,
    ) {
        let mut bb = bitboard.0;

        while bb != 0 {
            let from = bb.trailing_zeros() as u8;
            bb &= bb - 1;

            let (f0, r0) = Self::file_rank(from);

            for &(df, dr) in directions {
                let mut f = f0 + df;
                let mut r = r0 + dr;

                while let Some(to) = Self::sq(f, r) {
                    if self.board.has_friend(to, color) {
                        break;
                    }

                    moves.push(Move {
                        from,
                        to,
                        promotion: None,
                        is_en_passant: false,
                        is_castle_kingside: false,
                        is_castle_queenside: false,
                    });

                    if self.board.has_enemy(to, color) {
                        break;
                    }

                    f += df;
                    r += dr;
                }
            }
        }
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

    fn gen_knight_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let mut knights = self.board.piece_bitboard(color, PieceKind::Knight).0;

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

        while knights != 0 {
            let from = knights.trailing_zeros() as u8;
            knights &= knights - 1;

            let (f, r) = Self::file_rank(from);

            for (df, dr) in OFFSETS {
                if let Some(to) = Self::sq(f + df, r + dr) {
                    if self.board.has_friend(to, color) {
                        continue;
                    }

                    moves.push(Move {
                        from,
                        to,
                        promotion: None,
                        is_en_passant: false,
                        is_castle_kingside: false,
                        is_castle_queenside: false,
                    });
                }
            }
        }
    }

    fn gen_bishop_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let bishops = self.board.piece_bitboard(color, PieceKind::Bishop);
        const DIAG: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        self.gen_slider_moves(color, bishops, &DIAG, moves);
    }

    fn gen_rook_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let rooks = self.board.piece_bitboard(color, PieceKind::Rook);
        const ORTHO: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        self.gen_slider_moves(color, rooks, &ORTHO, moves);
    }

    fn gen_queen_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let queens = self.board.piece_bitboard(color, PieceKind::Queen);
        const ORTHODIAG: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];
        self.gen_slider_moves(color, queens, &ORTHODIAG, moves);
    }

    fn gen_king_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let king = self.board.piece_bitboard(color, PieceKind::King).0;

        if king == 0 {
            return;
        }

        let from = king.trailing_zeros() as u8;
        let (f, r) = Self::file_rank(from);

        for df in -1..=1 {
            for dr in -1..=1 {
                if df == 0 && dr == 0 {
                    continue;
                }

                if let Some(to) = Self::sq(f + df, r + dr) {
                    if self.board.has_friend(to, color) {
                        continue;
                    }

                    moves.push(Move {
                        from,
                        to,
                        promotion: None,
                        is_en_passant: false,
                        is_castle_kingside: false,
                        is_castle_queenside: false,
                    });
                }
            }
        }

        match color {
            Color::White => {
                if from == 4 {
                    if self.can_castle_kingside()
                        && self.board.is_empty(5)
                        && self.board.is_empty(6)
                        && self.board.piece_bitboard(Color::White, PieceKind::Rook).0 & (1u64 << 7)
                            != 0
                    {
                        moves.push(Move {
                            from,
                            to: 6,
                            promotion: None,
                            is_en_passant: false,
                            is_castle_kingside: true,
                            is_castle_queenside: false,
                        });
                    }

                    if self.can_castle_queenside()
                        && self.board.is_empty(3)
                        && self.board.is_empty(2)
                        && self.board.is_empty(1)
                        && self.board.piece_bitboard(Color::White, PieceKind::Rook).0 & (1u64 << 0)
                            != 0
                    {
                        moves.push(Move {
                            from,
                            to: 2,
                            promotion: None,
                            is_en_passant: false,
                            is_castle_kingside: false,
                            is_castle_queenside: true,
                        });
                    }
                }
            }
            Color::Black => {
                if from == 60 {
                    if self.can_castle_kingside()
                        && self.board.is_empty(61)
                        && self.board.is_empty(62)
                        && self.board.piece_bitboard(Color::Black, PieceKind::Rook).0 & (1u64 << 63)
                            != 0
                    {
                        moves.push(Move {
                            from,
                            to: 62,
                            promotion: None,
                            is_en_passant: false,
                            is_castle_kingside: true,
                            is_castle_queenside: false,
                        });
                    }

                    if self.can_castle_queenside()
                        && self.board.is_empty(59)
                        && self.board.is_empty(58)
                        && self.board.is_empty(57)
                        && self.board.piece_bitboard(Color::Black, PieceKind::Rook).0 & (1u64 << 56)
                            != 0
                    {
                        moves.push(Move {
                            from,
                            to: 58,
                            promotion: None,
                            is_en_passant: false,
                            is_castle_kingside: false,
                            is_castle_queenside: true,
                        });
                    }
                }
            }
        }
    }

    pub(crate) fn gen_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        self.gen_pawn_moves(self.turn, &mut moves);
        self.gen_knight_moves(self.turn, &mut moves);
        self.gen_bishop_moves(self.turn, &mut moves);
        self.gen_rook_moves(self.turn, &mut moves);
        self.gen_queen_moves(self.turn, &mut moves);
        self.gen_king_moves(self.turn, &mut moves);

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
