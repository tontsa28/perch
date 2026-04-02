use std::num::NonZeroU16;
use std::result::Result as StdResult;

use crate::{
    bitboard::Bitboard,
    board::{Board, Color},
    error::{Error, Result},
    mov::{Move, PieceKind, Undo},
};

#[derive(Debug, Clone, Copy)]
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

    fn gen_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        self.gen_pawn_moves(self.turn, &mut moves);
        self.gen_knight_moves(self.turn, &mut moves);
        self.gen_bishop_moves(self.turn, &mut moves);
        self.gen_rook_moves(self.turn, &mut moves);
        self.gen_queen_moves(self.turn, &mut moves);
        self.gen_king_moves(self.turn, &mut moves);

        moves
    }

    pub(crate) fn board(&self) -> Board {
        self.board
    }

    pub(crate) fn turn(&self) -> Color {
        self.turn
    }

    pub(crate) fn is_check(&self, color: Color) -> bool {
        let king_sq = self.board.piece_square(color, PieceKind::King);
        let attacker = !color;
        self.board.is_square_attacked(king_sq, attacker)
    }

    pub(crate) fn make_move(&mut self, mv: Move) -> Undo {
        let us = self.turn;
        let mut undo = Undo {
            captured: None,
            castling: self.castling,
            en_passant: self.en_passant,
            halfmoves: self.halfmoves,
            fullmoves: self.fullmoves,
        };

        let (moving_color, moving_kind) = self.board.piece_at(mv.from).unwrap();
        assert_eq!(moving_color, us);

        let mut is_capture = false;

        if mv.is_en_passant {
            let cap_sq = match us {
                Color::White => mv.to - 8,
                Color::Black => mv.to + 8,
            };

            if let Some((cap_color, cap_kind)) = self.board.piece_at(cap_sq) {
                assert_eq!(cap_color, !us);
                assert_eq!(cap_kind, PieceKind::Pawn);
                self.board.remove_piece(cap_color, cap_kind, cap_sq);
                undo.captured = Some((cap_color, cap_kind, cap_sq));
                is_capture = true;
            }
        } else if let Some((cap_color, cap_kind)) = self.board.piece_at(mv.to) {
            assert_eq!(cap_color, !us);
            self.board.remove_piece(cap_color, cap_kind, mv.to);
            undo.captured = Some((cap_color, cap_kind, mv.to));
            is_capture = true;
        }

        self.board.remove_piece(us, moving_kind, mv.from);
        let placed_kind = mv.promotion.unwrap_or(moving_kind);
        self.board.add_piece(us, placed_kind, mv.to);

        if mv.is_castle_kingside {
            match us {
                Color::White => {
                    self.board.remove_piece(Color::White, PieceKind::Rook, 7);
                    self.board.add_piece(Color::White, PieceKind::Rook, 5);
                }
                Color::Black => {
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 63);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 61);
                }
            }
        } else if mv.is_castle_queenside {
            match us {
                Color::White => {
                    self.board.remove_piece(Color::White, PieceKind::Rook, 0);
                    self.board.add_piece(Color::White, PieceKind::Rook, 3);
                }
                Color::Black => {
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 56);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 59);
                }
            }
        }

        match us {
            Color::White => {
                if moving_kind == PieceKind::King {
                    self.castling &= !(Self::WK | Self::WQ);
                }
                if moving_kind == PieceKind::Rook {
                    if mv.from == 7 {
                        self.castling &= !Self::WK;
                    } else if mv.from == 0 {
                        self.castling &= !Self::WQ;
                    }
                }
            }
            Color::Black => {
                if moving_kind == PieceKind::King {
                    self.castling &= !(Self::BK | Self::BQ);
                }
                if moving_kind == PieceKind::Rook {
                    if mv.from == 63 {
                        self.castling &= !Self::BK;
                    } else if mv.from == 56 {
                        self.castling &= !Self::BQ;
                    }
                }
            }
        }

        if !mv.is_en_passant {
            match mv.to {
                7 => self.castling &= !Self::WK,
                0 => self.castling &= !Self::WQ,
                63 => self.castling &= !Self::BK,
                56 => self.castling &= !Self::BQ,
                _ => {}
            }
        }

        self.en_passant = None;
        if moving_kind == PieceKind::Pawn {
            let delta = (mv.to as i16) - (mv.from as i16);
            if delta == 16 || delta == -16 {
                let ep = ((mv.from as u16 + mv.to as u16) / 2) as u8;
                self.en_passant = Some(ep);
            }
        }

        if moving_kind == PieceKind::Pawn || is_capture {
            self.halfmoves = 0;
        } else {
            self.halfmoves = self.halfmoves.saturating_add(1);
        }

        if us == Color::Black {
            self.fullmoves = self.fullmoves.saturating_add(1);
        }

        self.turn = !self.turn;

        undo
    }

    pub(crate) fn unmake_move(&mut self, mv: Move, undo: Undo) {
        self.turn = !self.turn;
        let us = self.turn;

        self.fullmoves = undo.fullmoves;
        self.halfmoves = undo.halfmoves;
        self.en_passant = undo.en_passant;
        self.castling = undo.castling;

        if mv.is_castle_kingside {
            match us {
                Color::White => {
                    self.board.remove_piece(Color::White, PieceKind::Rook, 5);
                    self.board.add_piece(Color::White, PieceKind::Rook, 7);
                }
                Color::Black => {
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 61);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 63);
                }
            }
        } else if mv.is_castle_queenside {
            match us {
                Color::White => {
                    self.board.remove_piece(Color::White, PieceKind::Rook, 3);
                    self.board.add_piece(Color::White, PieceKind::Rook, 0);
                }
                Color::Black => {
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 59);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 56);
                }
            }
        }

        if let Some(promoted_to) = mv.promotion {
            self.board.remove_piece(us, promoted_to, mv.to);
            self.board.add_piece(us, PieceKind::Pawn, mv.from);
        } else {
            let (c, k) = self.board.piece_at(mv.to).unwrap();
            assert_eq!(c, us);
            self.board.remove_piece(us, k, mv.to);
            self.board.add_piece(us, k, mv.from);
        }

        if let Some((cap_color, cap_kind, cap_sq)) = undo.captured {
            self.board.add_piece(cap_color, cap_kind, cap_sq);
        }
    }

    pub(crate) fn legal_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        let pseudo = self.gen_pseudo_legal_moves();
        let us = self.turn;

        for mv in pseudo {
            let undo = self.make_move(mv);
            if !self.is_check(us) {
                moves.push(mv);
            }
            self.unmake_move(mv, undo);
        }

        moves
    }

    pub(crate) fn evaluate(&self) -> i32 {
        match self.turn {
            Color::White => self.board.evaluate_material(),
            Color::Black => -self.board.evaluate_material(),
        }
    }

    pub(crate) fn parse_uci_move(&mut self, s: &str) -> Result<Move> {
        let raw = Move::try_from(s)?;

        self.legal_moves()
            .into_iter()
            .find(|m| m.from == raw.from && m.to == raw.to && m.promotion == raw.promotion)
            .ok_or_else(|| "Illegal move".into())
    }

    pub(crate) fn is_capture(&self, mv: Move) -> bool {
        mv.is_en_passant || self.board.has_enemy(mv.to, self.turn)
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
