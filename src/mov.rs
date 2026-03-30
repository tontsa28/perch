pub(crate) struct Move {
    from: u8,
    to: u8,
    promotion: Option<PieceKind>,
    is_en_passant: bool,
    is_castle_kingside: bool,
    is_castle_queenside: bool,
}

#[derive(Debug)]
pub(crate) enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
