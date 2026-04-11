use crate::board::Color;

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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PieceOnSquare {
    Empty = 0,
    WhitePawn = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteRook = 4,
    WhiteQueen = 5,
    WhiteKing = 6,
    BlackPawn = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackRook = 10,
    BlackQueen = 11,
    BlackKing = 12,
}

impl From<PieceOnSquare> for char {
    fn from(value: PieceOnSquare) -> Self {
        match value {
            PieceOnSquare::Empty => ' ',
            PieceOnSquare::WhitePawn => 'P',
            PieceOnSquare::WhiteKnight => 'N',
            PieceOnSquare::WhiteBishop => 'B',
            PieceOnSquare::WhiteRook => 'R',
            PieceOnSquare::WhiteQueen => 'Q',
            PieceOnSquare::WhiteKing => 'K',
            PieceOnSquare::BlackPawn => 'p',
            PieceOnSquare::BlackKnight => 'n',
            PieceOnSquare::BlackBishop => 'b',
            PieceOnSquare::BlackRook => 'r',
            PieceOnSquare::BlackQueen => 'q',
            PieceOnSquare::BlackKing => 'k',
        }
    }
}

impl From<(Color, PieceKind)> for PieceOnSquare {
    fn from(value: (Color, PieceKind)) -> Self {
        match value {
            (Color::White, PieceKind::Pawn) => Self::WhitePawn,
            (Color::White, PieceKind::Knight) => Self::WhiteKnight,
            (Color::White, PieceKind::Bishop) => Self::WhiteBishop,
            (Color::White, PieceKind::Rook) => Self::WhiteRook,
            (Color::White, PieceKind::Queen) => Self::WhiteQueen,
            (Color::White, PieceKind::King) => Self::WhiteKing,
            (Color::Black, PieceKind::Pawn) => Self::BlackPawn,
            (Color::Black, PieceKind::Knight) => Self::BlackKnight,
            (Color::Black, PieceKind::Bishop) => Self::BlackBishop,
            (Color::Black, PieceKind::Rook) => Self::BlackRook,
            (Color::Black, PieceKind::Queen) => Self::BlackQueen,
            (Color::Black, PieceKind::King) => Self::BlackKing,
        }
    }
}

impl From<PieceOnSquare> for (Color, PieceKind) {
    fn from(value: PieceOnSquare) -> Self {
        match value {
            PieceOnSquare::WhitePawn => (Color::White, PieceKind::Pawn),
            PieceOnSquare::WhiteKnight => (Color::White, PieceKind::Knight),
            PieceOnSquare::WhiteBishop => (Color::White, PieceKind::Bishop),
            PieceOnSquare::WhiteRook => (Color::White, PieceKind::Rook),
            PieceOnSquare::WhiteQueen => (Color::White, PieceKind::Queen),
            PieceOnSquare::WhiteKing => (Color::White, PieceKind::King),
            PieceOnSquare::BlackPawn => (Color::Black, PieceKind::Pawn),
            PieceOnSquare::BlackKnight => (Color::Black, PieceKind::Knight),
            PieceOnSquare::BlackBishop => (Color::Black, PieceKind::Bishop),
            PieceOnSquare::BlackRook => (Color::Black, PieceKind::Rook),
            PieceOnSquare::BlackQueen => (Color::Black, PieceKind::Queen),
            PieceOnSquare::BlackKing => (Color::Black, PieceKind::King),
            PieceOnSquare::Empty => {
                unreachable!("cannot convert PieceOnSquare::Empty into Color and PieceKind")
            }
        }
    }
}

pub(crate) fn parse_piece(c: char) -> Option<(PieceOnSquare, Color, PieceKind)> {
    match c {
        'P' => Some((PieceOnSquare::WhitePawn, Color::White, PieceKind::Pawn)),
        'N' => Some((PieceOnSquare::WhiteKnight, Color::White, PieceKind::Knight)),
        'B' => Some((PieceOnSquare::WhiteBishop, Color::White, PieceKind::Bishop)),
        'R' => Some((PieceOnSquare::WhiteRook, Color::White, PieceKind::Rook)),
        'Q' => Some((PieceOnSquare::WhiteQueen, Color::White, PieceKind::Queen)),
        'K' => Some((PieceOnSquare::WhiteKing, Color::White, PieceKind::King)),
        'p' => Some((PieceOnSquare::BlackPawn, Color::Black, PieceKind::Pawn)),
        'n' => Some((PieceOnSquare::BlackKnight, Color::Black, PieceKind::Knight)),
        'b' => Some((PieceOnSquare::BlackBishop, Color::Black, PieceKind::Bishop)),
        'r' => Some((PieceOnSquare::BlackRook, Color::Black, PieceKind::Rook)),
        'q' => Some((PieceOnSquare::BlackQueen, Color::Black, PieceKind::Queen)),
        'k' => Some((PieceOnSquare::BlackKing, Color::Black, PieceKind::King)),
        _ => None,
    }
}
