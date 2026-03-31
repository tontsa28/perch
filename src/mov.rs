pub(crate) struct Move {
    pub(crate) from: u8,
    pub(crate) to: u8,
    pub(crate) promotion: Option<PieceKind>,
    pub(crate) is_en_passant: bool,
    pub(crate) is_castle_kingside: bool,
    pub(crate) is_castle_queenside: bool,
}

#[repr(usize)]
#[derive(Debug)]
pub(crate) enum PieceKind {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}
