use std::num::NonZeroU16;
use std::result::Result as StdResult;

use crate::{
    bitboard::Bitboard,
    board::{Board, Color},
    error::{Error, Result},
    mov::{Move, Undo},
    piece::{PieceKind, PieceOnSquare},
};

/// A representation of a full chess position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Position {
    board: Board,
    turn: Color,
    castling: u8,
    en_passant: Option<u8>,
    halfmoves: u16,
    fullmoves: NonZeroU16,
}

impl Position {
    // Constants for handling castling rights
    const WK: u8 = 1 << 0;
    const WQ: u8 = 1 << 1;
    const BK: u8 = 1 << 2;
    const BQ: u8 = 1 << 3;

    /// Initialize the standard chess starting position.
    ///
    /// # Returns
    /// A `Position` set to the standard starting position.
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

    /// Convert a file-rank pair into a square.
    ///
    /// # Parameters
    /// - `file`: File index (0..7).
    /// - `rank`: Rank index (0..7).
    ///
    /// # Returns
    /// The square index, or `None` if out of bounds.
    #[inline(always)]
    fn sq(file: i8, rank: i8) -> Option<u8> {
        if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
            Some((rank as u8) * 8 + (file as u8))
        } else {
            None
        }
    }

    /// Convert a square into a file-rank pair.
    ///
    /// # Parameters
    /// - `sq`: Square index in the range 0..64.
    ///
    /// # Returns
    /// A `(file, rank)` pair.
    #[inline(always)]
    fn file_rank(sq: u8) -> (i8, i8) {
        ((sq % 8) as i8, (sq / 8) as i8)
    }

    /// Push promotion moves into the move buffer.
    ///
    /// # Parameters
    /// - `moves`: Move buffer to append to.
    /// - `from`: Origin square.
    /// - `to`: Destination square.
    /// - `is_en_passant`: Whether the move is flagged as en passant.
    ///
    /// # Returns
    /// Nothing.
    #[inline(always)]
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

    /// Check if we can castle kingside.
    ///
    /// # Returns
    /// `true` if the side to move still has kingside castling rights.
    fn can_castle_kingside(&self) -> bool {
        // Compute AND between castling rights and kingside castling right
        // to find if move is still legal to play
        match self.turn {
            Color::White => self.castling & Self::WK != 0,
            Color::Black => self.castling & Self::BK != 0,
        }
    }

    /// Check if we can castle queenside.
    ///
    /// # Returns
    /// `true` if the side to move still has queenside castling rights.
    fn can_castle_queenside(&self) -> bool {
        // Compute AND between castling rights and queenside castling right
        // to find if move is still legal to play
        match self.turn {
            Color::White => self.castling & Self::WQ != 0,
            Color::Black => self.castling & Self::BQ != 0,
        }
    }

    /// Generate all slider moves (bishop + rook + queen).
    ///
    /// # Parameters
    /// - `color`: Color whose pieces are moved.
    /// - `bb`: Bitboard of sliding pieces.
    /// - `directions`: Direction vectors to step along.
    /// - `moves`: Move buffer to append to.
    ///
    /// # Returns
    /// Nothing. Generated moves are appended to `moves`.
    fn gen_slider_moves(
        &self,
        color: Color,
        mut bb: Bitboard,
        directions: &[(i8, i8)],
        moves: &mut Vec<Move>,
    ) {
        while !bb.is_empty() {
            // Pop square with lowest index and get its initial file-rank coordinates
            let from = bb.pop_lsb();
            let (f0, r0) = Self::file_rank(from);

            // Process all directions
            for &(df, dr) in directions {
                // Compute first destination square in current direction
                let mut f = f0 + df;
                let mut r = r0 + dr;

                // Process further squares in same direction
                while let Some(to) = Self::sq(f, r) {
                    // If current destination square contains a friendly piece, stop processing
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

                    // If current destination square contains an enemy piece, stop processing
                    if self.board.has_enemy(to, color) {
                        break;
                    }

                    f += df;
                    r += dr;
                }
            }
        }
    }

    /// Generate all pawn moves.
    ///
    /// # Parameters
    /// - `color`: Color whose pawns are moved.
    /// - `moves`: Move buffer to append to.
    ///
    /// # Returns
    /// Nothing. Generated moves are appended to `moves`.
    fn gen_pawn_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let mut pawns = self.board.piece_bitboard(color, PieceKind::Pawn);

        // Determine direction of pushing, initial rank, and promotion rank
        let (push_delta, start_rank, promo_rank) = match color {
            Color::White => (1i8, 1i8, 7i8),
            Color::Black => (-1i8, 6i8, 0i8),
        };

        while !pawns.is_empty() {
            // Pop pawn with lowest index and get its initial file-rank coordinates
            let from = pawns.pop_lsb();
            let (f, r) = Self::file_rank(from);

            // Check if pawn can be pushed once
            if let Some(one_step) = Self::sq(f, r + push_delta)
                && self.board.is_empty(one_step)
            {
                let (_, to_rank) = Self::file_rank(one_step);

                if to_rank == promo_rank {
                    // Generate all promotion moves
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

                    // Check if pawn is at its initial rank and can be pushed twice
                    if r == start_rank
                        && let Some(two_step) = Self::sq(f, r + 2 * push_delta)
                        && self.board.is_empty(two_step)
                    {
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

            // Process both capturable squares
            for df in [-1i8, 1i8] {
                if let Some(to) = Self::sq(f + df, r + push_delta) {
                    let (_, to_rank) = Self::file_rank(to);

                    // Check if destination square contains an enemy piece
                    if self.board.has_enemy(to, color) {
                        if to_rank == promo_rank {
                            // Generate all promotion moves
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

                        // Skip to next capture as a normal capture cannot be en passant
                        continue;
                    }

                    // Check if destination square is en passant square
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

    /// Generate all knight moves.
    ///
    /// # Parameters
    /// - `color`: Color whose knights are moved.
    /// - `moves`: Move buffer to append to.
    ///
    /// # Returns
    /// Nothing. Generated moves are appended to `moves`.
    fn gen_knight_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let mut knights = self.board.piece_bitboard(color, PieceKind::Knight);

        // Construct move coordinate offsets
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

        while !knights.is_empty() {
            // Pop knight with lowest index and get its initial file-rank coordinates
            let from = knights.pop_lsb();
            let (f, r) = Self::file_rank(from);

            // Process all offsets
            for (df, dr) in OFFSETS {
                if let Some(to) = Self::sq(f + df, r + dr) {
                    // If current destination square contains a friendly piece, skip it
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

    /// Generate all bishop moves.
    ///
    /// # Parameters
    /// - `color`: Color whose bishops are moved.
    /// - `moves`: Move buffer to append to.
    ///
    /// # Returns
    /// Nothing. Generated moves are appended to `moves`.
    fn gen_bishop_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let bishops = self.board.piece_bitboard(color, PieceKind::Bishop);

        // Construct diagonal move directions
        const DIAG: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        self.gen_slider_moves(color, bishops, &DIAG, moves);
    }

    /// Generate all rook moves.
    ///
    /// # Parameters
    /// - `color`: Color whose rooks are moved.
    /// - `moves`: Move buffer to append to.
    ///
    /// # Returns
    /// Nothing. Generated moves are appended to `moves`.
    fn gen_rook_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let rooks = self.board.piece_bitboard(color, PieceKind::Rook);

        // Construct orthogonal move directions
        const ORTHO: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        self.gen_slider_moves(color, rooks, &ORTHO, moves);
    }

    /// Generate all queen moves.
    ///
    /// # Parameters
    /// - `color`: Color whose queens are moved.
    /// - `moves`: Move buffer to append to.
    ///
    /// # Returns
    /// Nothing. Generated moves are appended to `moves`.
    fn gen_queen_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let queens = self.board.piece_bitboard(color, PieceKind::Queen);

        // Construct orthodiagonal move directions
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

    /// Generate all king moves.
    ///
    /// # Parameters
    /// - `color`: Color whose king is moved.
    /// - `moves`: Move buffer to append to.
    ///
    /// # Returns
    /// Nothing. Generated moves are appended to `moves`.
    fn gen_king_moves(&self, color: Color, moves: &mut Vec<Move>) {
        let king = self.board.piece_bitboard(color, PieceKind::King);

        // This is a safeguard and should not be possible in any legal chess position
        if king.is_empty() {
            return;
        }

        // Get king square and its initial file-rank coordinates
        let from = king.lsb_sq();
        let (f, r) = Self::file_rank(from);

        // Process all directions
        for df in -1..=1 {
            for dr in -1..=1 {
                // Skip center square in which king is currently positioned
                if df == 0 && dr == 0 {
                    continue;
                }

                if let Some(to) = Self::sq(f + df, r + dr) {
                    // If current destination square contains a friendly piece, skip it
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

        // Process castling moves
        match color {
            Color::White => {
                // Square 4 = e1
                if from == 4 {
                    // Squares 5 = f1, 6 = g1
                    // Compute AND between rook bitboard and a bitboard with square 7 = h1 set
                    // to find if there is a rook in h1
                    if self.can_castle_kingside()
                        && self.board.is_empty(5)
                        && self.board.is_empty(6)
                        && (self.board.piece_bitboard(Color::White, PieceKind::Rook)
                            & Bitboard::new(1u64 << 7))
                        .is_not_empty()
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

                    // Squares 3 = d1, 2 = c1, 1 = b1
                    // Compute AND between rook bitboard and a bitboard with square 0 = a1 set
                    // to find if there is a rook in a1
                    if self.can_castle_queenside()
                        && self.board.is_empty(3)
                        && self.board.is_empty(2)
                        && self.board.is_empty(1)
                        && (self.board.piece_bitboard(Color::White, PieceKind::Rook)
                            & Bitboard::new(1u64 << 0))
                        .is_not_empty()
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
                // Square 60 = e8
                if from == 60 {
                    // Squares 61 = f8, 62 = g8
                    // Compute AND between rook bitboard and a bitboard with square 63 = h8 set
                    // to find if there is a rook in h8
                    if self.can_castle_kingside()
                        && self.board.is_empty(61)
                        && self.board.is_empty(62)
                        && (self.board.piece_bitboard(Color::Black, PieceKind::Rook)
                            & Bitboard::new(1u64 << 63))
                        .is_not_empty()
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

                    // Squares 59 = d8, 58 = c8, 57 = b8
                    // Compute AND between rook bitboard and a bitboard with square 56 = a8 set
                    // to find if there is a rook in a8
                    if self.can_castle_queenside()
                        && self.board.is_empty(59)
                        && self.board.is_empty(58)
                        && self.board.is_empty(57)
                        && (self.board.piece_bitboard(Color::Black, PieceKind::Rook)
                            & Bitboard::new(1u64 << 56))
                        .is_not_empty()
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

    /// Generate all pseudo-legal moves (ignoring checks).
    ///
    /// # Returns
    /// A vector of pseudo-legal moves for the side to move.
    fn pseudo_legal_moves(&self) -> Vec<Move> {
        // Preallocate memory for 64 moves to avoid allocation overhead
        let mut moves = Vec::with_capacity(64);

        self.gen_pawn_moves(self.turn, &mut moves);
        self.gen_knight_moves(self.turn, &mut moves);
        self.gen_bishop_moves(self.turn, &mut moves);
        self.gen_rook_moves(self.turn, &mut moves);
        self.gen_queen_moves(self.turn, &mut moves);
        self.gen_king_moves(self.turn, &mut moves);

        moves
    }

    /// Get a reference to the associated `Board`.
    ///
    /// # Returns
    /// A shared reference to the underlying `Board`.
    pub(crate) fn board(&self) -> &Board {
        &self.board
    }

    /// Check whose turn it is.
    ///
    /// # Returns
    /// The side to move.
    pub(crate) fn turn(&self) -> Color {
        self.turn
    }

    /// Check if the king is in check.
    ///
    /// # Parameters
    /// - `color`: Side whose king is tested.
    ///
    /// # Returns
    /// `true` if the king of `color` is in check.
    pub(crate) fn is_check(&self, color: Color) -> bool {
        // Checker is always our opponent
        let attacker = !color;
        let king_sq = self.board.king_square(color);

        self.board.is_square_attacked(king_sq, attacker)
    }

    /// Play a move in the current position.
    ///
    /// # Parameters
    /// - `mv`: Move to make.
    ///
    /// # Returns
    /// An `Undo` record that can restore the position.
    pub(crate) fn make_move(&mut self, mv: Move) -> Undo {
        // Gather all information required to restore this position
        let us = self.turn;
        let mut undo = Undo {
            captured: None,
            castling: self.castling,
            en_passant: self.en_passant,
            halfmoves: self.halfmoves,
            fullmoves: self.fullmoves,
        };

        let (moving_color, moving_kind) = self.board.piece_at(mv.from).into();
        debug_assert_eq!(moving_color, us);

        let mut is_capture = false;

        if mv.is_en_passant {
            // Get capture square by adding or subtracting a rank from destination square
            let cap_sq = match us {
                Color::White => mv.to - 8,
                Color::Black => mv.to + 8,
            };

            let cap_ps = self.board.piece_at(cap_sq);
            if cap_ps != PieceOnSquare::Empty {
                let (cap_color, cap_kind) = cap_ps.into();
                debug_assert_eq!(cap_color, !us);
                debug_assert_eq!(cap_kind, PieceKind::Pawn);

                // Remove captured piece from board
                self.board.remove_piece(cap_color, cap_kind, cap_sq);
                undo.captured = Some((cap_color, cap_kind, cap_sq));
                is_capture = true;
            }
        }

        let cap_ps = self.board.piece_at(mv.to);
        if cap_ps != PieceOnSquare::Empty {
            let (cap_color, cap_kind) = cap_ps.into();
            debug_assert_eq!(cap_color, !us);

            // Remove captured piece from board
            self.board.remove_piece(cap_color, cap_kind, mv.to);
            undo.captured = Some((cap_color, cap_kind, mv.to));
            is_capture = true;
        }

        // Move piece to its new position or place a new piece on board upon promotion
        self.board.remove_piece(us, moving_kind, mv.from);
        let placed_kind = mv.promotion.unwrap_or(moving_kind);
        self.board.add_piece(us, placed_kind, mv.to);

        if mv.is_castle_kingside {
            match us {
                Color::White => {
                    // Move rook from 7 = h1 to 5 = f1 upon castling
                    self.board.remove_piece(Color::White, PieceKind::Rook, 7);
                    self.board.add_piece(Color::White, PieceKind::Rook, 5);
                }
                Color::Black => {
                    // Move rook from 63 = h8 to 61 = f8 upon castling
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 63);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 61);
                }
            }
        } else if mv.is_castle_queenside {
            match us {
                Color::White => {
                    // Move rook from 0 = a1 to 3 = d1 upon castling
                    self.board.remove_piece(Color::White, PieceKind::Rook, 0);
                    self.board.add_piece(Color::White, PieceKind::Rook, 3);
                }
                Color::Black => {
                    // Move rook from 56 = a8 to 59 = d8 upon castling
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 56);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 59);
                }
            }
        }

        match us {
            Color::White => {
                // If king is moved, revoke its castling rights
                if moving_kind == PieceKind::King {
                    self.castling &= !(Self::WK | Self::WQ);
                }

                // If a rook is moved, revoke castling rights depending on its side
                if moving_kind == PieceKind::Rook {
                    if mv.from == 7 {
                        self.castling &= !Self::WK;
                    } else if mv.from == 0 {
                        self.castling &= !Self::WQ;
                    }
                }
            }
            Color::Black => {
                // If king is moved, revoke its castling rights
                if moving_kind == PieceKind::King {
                    self.castling &= !(Self::BK | Self::BQ);
                }

                // If a rook is moved, revoke castling rights depending on its side
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
            // If a rook is captured on its starting corner, revoke respective castling right
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
            // If a pawn is pushed two squares, set en passant square to intermediate square
            let delta = (mv.to as i16) - (mv.from as i16);
            if delta == 16 || delta == -16 {
                let ep = ((mv.from as u16 + mv.to as u16) / 2) as u8;
                self.en_passant = Some(ep);
            }
        }

        // If a pawn is moved or a piece is captured,
        // restart halfmove counter, otherwise increment it
        if moving_kind == PieceKind::Pawn || is_capture {
            self.halfmoves = 0;
        } else {
            self.halfmoves = self.halfmoves.saturating_add(1);
        }

        // Increment fullmove counter upon black's move
        if us == Color::Black {
            self.fullmoves = self.fullmoves.saturating_add(1);
        }

        self.turn = !self.turn;

        undo
    }

    /// Restore the position after a move.
    ///
    /// # Parameters
    /// - `mv`: Move that was made.
    /// - `undo`: Undo record returned by `make_move`.
    ///
    /// # Returns
    /// Nothing.
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
                    // Revert castling by moving rook from 5 = f1 to 7 = h1
                    self.board.remove_piece(Color::White, PieceKind::Rook, 5);
                    self.board.add_piece(Color::White, PieceKind::Rook, 7);
                }
                Color::Black => {
                    // Revert castling by moving rook from 61 = f8 to 63 = h8
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 61);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 63);
                }
            }
        } else if mv.is_castle_queenside {
            match us {
                Color::White => {
                    // Revert castling by moving rook from 3 = d1 to 0 = a1
                    self.board.remove_piece(Color::White, PieceKind::Rook, 3);
                    self.board.add_piece(Color::White, PieceKind::Rook, 0);
                }
                Color::Black => {
                    // Revert castling by moving rook from 59 = d8 to 56 = a8
                    self.board.remove_piece(Color::Black, PieceKind::Rook, 59);
                    self.board.add_piece(Color::Black, PieceKind::Rook, 56);
                }
            }
        }

        if let Some(promoted_to) = mv.promotion {
            // Revert promotion by removing promoted piece
            // and adding promoted pawn back to its previous square
            self.board.remove_piece(us, promoted_to, mv.to);
            self.board.add_piece(us, PieceKind::Pawn, mv.from);
        } else {
            let (c, k) = self.board.piece_at(mv.to).into();
            debug_assert_eq!(c, us);

            // Revert move by moving piece back to its previous square
            self.board.remove_piece(us, k, mv.to);
            self.board.add_piece(us, k, mv.from);
        }

        if let Some((cap_color, cap_kind, cap_sq)) = undo.captured {
            // Add a captured piece back to its original square
            self.board.add_piece(cap_color, cap_kind, cap_sq);
        }
    }

    /// Play a null move in the current position.
    ///
    /// # Returns
    /// The previous en passant square, if any.
    pub(crate) fn make_null_move(&mut self) -> Option<u8> {
        let ep = self.en_passant;
        self.en_passant = None;
        self.turn = !self.turn;
        ep
    }

    /// Restore the position after a null move.
    ///
    /// # Parameters
    /// - `ep`: En passant square returned by `make_null_move`.
    ///
    /// # Returns
    /// Nothing.
    pub(crate) fn unmake_null_move(&mut self, ep: Option<u8>) {
        self.turn = !self.turn;
        self.en_passant = ep;
    }

    /// Generate all legal moves in the current position.
    ///
    /// # Returns
    /// A vector of legal moves for the side to move.
    pub(crate) fn legal_moves(&mut self) -> Vec<Move> {
        // Preallocate memory for 64 moves to avoid allocation overhead
        let mut moves = Vec::with_capacity(64);
        let pseudo = self.pseudo_legal_moves();
        let us = self.turn;

        for mv in pseudo {
            if mv.is_castle_kingside || mv.is_castle_queenside {
                // Determine required free squares when castling
                let (start_sq, transit_sq, dest_sq) =
                    match (us, mv.is_castle_kingside, mv.is_castle_queenside) {
                        (Color::White, true, false) => (4, 5, 6),
                        (Color::Black, true, false) => (60, 61, 62),
                        (Color::White, false, true) => (4, 3, 2),
                        (Color::Black, false, true) => (60, 59, 58),
                        _ => unreachable!("castle move must be exactly one side"),
                    };

                // If any square is attacked, skip castling move as it is illegal
                if self.board.is_square_attacked(start_sq, !us)
                    || self.board.is_square_attacked(transit_sq, !us)
                    || self.board.is_square_attacked(dest_sq, !us)
                {
                    continue;
                }
            }

            // Add move to legal moves if it doesn't expose our king to a check
            let undo = self.make_move(mv);
            if !self.is_check(us) {
                moves.push(mv);
            }
            self.unmake_move(mv, undo);
        }

        moves
    }

    /// Convert the evaluation to the side-to-move perspective.
    ///
    /// # Returns
    /// Evaluation score in centipawns from the side-to-move perspective.
    pub(crate) fn evaluate(&self) -> i32 {
        match self.turn {
            Color::White => self.board.evaluate_material_pst(),
            Color::Black => -self.board.evaluate_material_pst(),
        }
    }

    /// Convert a UCI-formatted move string into `Move` if it is legal in the current position.
    ///
    /// # Parameters
    /// - `s`: UCI move string to parse.
    ///
    /// # Returns
    /// A legal `Move`, or an error if parsing fails or the move is illegal.
    pub(crate) fn parse_uci_move(&mut self, s: &str) -> Result<Move> {
        let raw = Move::try_from(s)?;

        // Look for a matching move within legal moves
        self.legal_moves()
            .into_iter()
            .find(|m| m.from == raw.from && m.to == raw.to && m.promotion == raw.promotion)
            .ok_or_else(|| "Illegal move".into())
    }

    /// Check if a `Move` is a capture in the current position.
    ///
    /// # Parameters
    /// - `mv`: Move to test.
    ///
    /// # Returns
    /// `true` if `mv` captures an enemy piece (including en passant).
    pub(crate) fn is_capture(&self, mv: Move) -> bool {
        mv.is_en_passant || self.board.has_enemy(mv.to, self.turn)
    }
}

impl TryFrom<&str> for Position {
    type Error = Error;

    /// Convert a FEN string into `Position`.
    ///
    /// # Parameters
    /// - `value`: Full FEN string.
    ///
    /// # Returns
    /// A parsed `Position`, or an error if the FEN is invalid.
    fn try_from(value: &str) -> StdResult<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let pos_str = *parts.first().unwrap();
        let turn_str = *parts.get(1).unwrap();
        let castling_str = *parts.get(2).unwrap();
        let en_passant_str = *parts.get(3).unwrap();
        let halfmoves_str = *parts.get(4).unwrap_or(&"0");
        let fullmoves_str = *parts.get(5).unwrap_or(&"1");

        // Construct castling rights
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

        // Construct en passant
        let mut en_passant: Option<u8> = None;
        if en_passant_str != "-" {
            let bytes = en_passant_str.as_bytes();

            // Get file and rank by subtracting integer representations
            // of 'a' and '1' from first two bytes
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

#[cfg(test)]
mod tests {
    use super::Position;
    use crate::{board::Color, search::perft};

    fn pos(fen: &str) -> Position {
        Position::try_from(fen).expect("valid FEN")
    }

    // ── Basic ────────────────────────────────────────────────────────────────

    #[test]
    fn startpos_has_20_legal_moves() {
        assert_eq!(Position::new().legal_moves().len(), 20);
    }

    #[test]
    fn startpos_fen_equivalent_to_new() {
        let mut p1 = Position::new();
        let mut p2 = pos("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(perft(&mut p1, 3), perft(&mut p2, 3));
    }

    #[test]
    fn make_unmake_is_invertible() {
        // After making and immediately unmaking every legal move,
        // position must be byte-for-byte identical to before, verified via perft
        let mut p = Position::new();
        let before = perft(&mut p, 3);
        for mv in p.legal_moves() {
            let undo = p.make_move(mv);
            p.unmake_move(mv, undo);
        }
        assert_eq!(perft(&mut p, 3), before);
    }

    #[test]
    fn parse_uci_move_accepts_legal_move() {
        let mut p = Position::new();
        assert!(p.parse_uci_move("e2e4").is_ok());
    }

    #[test]
    fn parse_uci_move_rejects_illegal_move() {
        let mut p = Position::new();
        assert!(p.parse_uci_move("e2e5").is_err());
    }

    // ── Terminal positions ────────────────────────────────────────────────────

    #[test]
    fn checkmate_has_no_legal_moves() {
        // Back-rank mate: white rooks on a1 and b2, white king on h1
        let mut p = pos("k7/8/8/8/8/8/1R6/R6K b - - 0 1");
        assert!(p.is_check(Color::Black));
        assert!(p.legal_moves().is_empty());
    }

    #[test]
    fn stalemate_is_not_check_and_has_no_legal_moves() {
        // Queen + king cage: black king on a8, white queen on b6, white king on a6
        let mut p = pos("k7/8/KQ6/8/8/8/8/8 b - - 0 1");
        assert!(!p.is_check(Color::Black));
        assert!(p.legal_moves().is_empty());
    }

    // ── Castling ─────────────────────────────────────────────────────────────

    #[test]
    fn white_castles_both_sides() {
        let mut p = pos("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let moves = p.legal_moves();
        assert!(moves.iter().any(|m| m.is_castle_kingside));
        assert!(moves.iter().any(|m| m.is_castle_queenside));
    }

    #[test]
    fn no_castling_without_rights() {
        let mut p = pos("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1");
        assert!(
            !p.legal_moves()
                .iter()
                .any(|m| m.is_castle_kingside || m.is_castle_queenside)
        );
    }

    #[test]
    fn no_castling_through_attacked_square() {
        // Black rook on f8 covers f1, blocking white kingside castle transit
        let mut p = pos("5r2/8/8/8/8/8/8/4K2R w K - 0 1");
        assert!(!p.legal_moves().iter().any(|m| m.is_castle_kingside));
    }

    #[test]
    fn rook_move_removes_queenside_castling_right() {
        let mut p = pos("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

        // Move a1 rook; queenside castling should be lost
        let mv = p.parse_uci_move("a1a2").unwrap();
        p.make_move(mv);

        // Make a black move so it's White to move again
        let reply = p.parse_uci_move("a8a7").unwrap();
        p.make_move(reply);

        let moves = p.legal_moves();

        assert!(!moves.iter().any(|m| m.is_castle_queenside));
        assert!(moves.iter().any(|m| m.is_castle_kingside));
    }

    #[test]
    fn king_move_removes_castling_rights_even_if_king_returns() {
        let mut p = pos("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

        // Move king away and back
        let mv1 = p.parse_uci_move("e1e2").unwrap();
        p.make_move(mv1);
        let mv2 = p.parse_uci_move("a8a7").unwrap();
        p.make_move(mv2);
        let mv3 = p.parse_uci_move("e2e1").unwrap();
        p.make_move(mv3);
        let mv4 = p.parse_uci_move("h8h7").unwrap();
        p.make_move(mv4);

        // Castling should be permanently lost for White
        let moves = p.legal_moves();
        assert!(
            !moves
                .iter()
                .any(|m| m.is_castle_kingside || m.is_castle_queenside)
        );
    }

    // ── En passant ────────────────────────────────────────────────────────────

    #[test]
    fn en_passant_is_generated_when_target_set() {
        // White pawn on e5, black just played d7-d5 — en passant target is d6
        let mut p = pos("8/8/8/3pP3/8/8/8/4K2k w - d6 0 1");
        assert!(p.legal_moves().iter().any(|m| m.is_en_passant));
    }

    #[test]
    fn en_passant_not_generated_without_target() {
        let mut p = pos("8/8/8/3pP3/8/8/8/4K2k w - - 0 1");
        assert!(!p.legal_moves().iter().any(|m| m.is_en_passant));
    }

    #[test]
    fn en_passant_illegal_if_it_exposes_king() {
        // White pawn on e5, black pawn on d5, black rook on e8
        // En passant e5xd6 would open e-file -> illegal
        let mut p = pos("4r3/8/8/3pP3/8/8/8/4K3 w - d6 0 1");
        assert!(!p.legal_moves().iter().any(|m| m.is_en_passant));
    }

    // ── Promotions ────────────────────────────────────────────────────────────

    #[test]
    fn pawn_push_to_back_rank_gives_four_promotions() {
        let mut p = pos("8/P7/8/8/8/8/8/4K2k w - - 0 1");
        assert_eq!(
            p.legal_moves().iter().filter(|m| m.is_promotion()).count(),
            4
        );
    }

    #[test]
    fn pawn_capture_promotion_gives_eight_promotions() {
        // White pawn on a7, black knight on b8 — 4 push promotions + 4 capture promotions
        let mut p = pos("1n6/P7/8/8/8/8/8/4K2k w - - 0 1");
        assert_eq!(
            p.legal_moves().iter().filter(|m| m.is_promotion()).count(),
            8
        );
    }

    // ── FEN parsing ───────────────────────────────────────────────────────────

    #[test]
    fn fen_parses_side_to_move() {
        assert_eq!(
            pos("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").turn(),
            Color::White
        );
        assert_eq!(
            pos("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").turn(),
            Color::Black
        );
    }

    #[test]
    fn evaluate_is_negated_for_black_to_move() {
        // White has an extra pawn — evaluate() must be positive for white to move,
        // negative for black to move, and magnitudes must be equal
        let w = pos("8/8/8/8/8/8/P7/4K2k w - - 0 1");
        let b = pos("8/8/8/8/8/8/P7/4K2k b - - 0 1");
        assert!(w.evaluate() > 0);
        assert!(b.evaluate() < 0);
        assert_eq!(w.evaluate(), -b.evaluate());
    }

    // ── Perft ─────────────────────────────────────────────────────────────────

    mod perft_tests {
        use crate::{
            position::{Position, tests::pos},
            search::perft,
        };

        #[test]
        fn startpos_depth_1() {
            assert_eq!(perft(&mut Position::new(), 1), 20);
        }

        #[test]
        fn startpos_depth_2() {
            assert_eq!(perft(&mut Position::new(), 2), 400);
        }

        #[test]
        fn startpos_depth_3() {
            assert_eq!(perft(&mut Position::new(), 3), 8_902);
        }

        #[test]
        fn startpos_depth_4() {
            assert_eq!(perft(&mut Position::new(), 4), 197_281);
        }

        #[test]
        fn startpos_depth_5() {
            assert_eq!(perft(&mut Position::new(), 5), 4_865_609);
        }

        // Kiwipete — exercises castling, en passant, and promotions together
        #[test]
        fn kiwipete_depth_1() {
            assert_eq!(
                perft(
                    &mut pos(
                        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
                    ),
                    1
                ),
                48
            );
        }

        #[test]
        fn kiwipete_depth_2() {
            assert_eq!(
                perft(
                    &mut pos(
                        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
                    ),
                    2
                ),
                2_039
            );
        }

        #[test]
        fn kiwipete_depth_3() {
            assert_eq!(
                perft(
                    &mut pos(
                        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
                    ),
                    3
                ),
                97_862
            );
        }

        // Position 3 — stresses en passant pin edge cases
        #[test]
        fn position3_depth_1() {
            assert_eq!(
                perft(&mut pos("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"), 1),
                14
            );
        }

        #[test]
        fn position3_depth_2() {
            assert_eq!(
                perft(&mut pos("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"), 2),
                191
            );
        }

        #[test]
        fn position3_depth_3() {
            assert_eq!(
                perft(&mut pos("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"), 3),
                2_812
            );
        }

        #[test]
        fn position3_depth_5() {
            assert_eq!(
                perft(&mut pos("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"), 5),
                674_624
            );
        }

        // Position 5 — stresses promotions
        #[test]
        fn position5_depth_1() {
            assert_eq!(
                perft(
                    &mut pos("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
                    1
                ),
                44
            );
        }

        #[test]
        fn position5_depth_2() {
            assert_eq!(
                perft(
                    &mut pos("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
                    2
                ),
                1_486
            );
        }

        #[test]
        fn position5_depth_3() {
            assert_eq!(
                perft(
                    &mut pos("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
                    3
                ),
                62_379
            );
        }
    }
}
