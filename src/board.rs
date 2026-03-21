use crate::{bitboard::Bitboard, error::Error};

const WHITE_PIECES: &str = "PNBRQK";
const BLACK_PIECES: &str = "pnbrqk";

#[derive(Debug)]
pub(crate) struct Board {
    pieces: [Bitboard; 12],
    white: Bitboard,
    black: Bitboard,
    occupied: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [
                Bitboard(0xff00),
                Bitboard(0x42),
                Bitboard(0x24),
                Bitboard(0x81),
                Bitboard(0x10),
                Bitboard(0x8),
                Bitboard(0x00ff_0000_0000_0000),
                Bitboard(0x4200_0000_0000_0000),
                Bitboard(0x2400_0000_0000_0000),
                Bitboard(0x8100_0000_0000_0000),
                Bitboard(0x1000_0000_0000_0000),
                Bitboard(0x0800_0000_0000_0000),
            ],
            white: Bitboard(0xffff),
            black: Bitboard(0xffff_0000_0000_0000),
            occupied: Bitboard(0xffff_0000_0000_ffff),
        }
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(pos: &str) -> Result<Self, Self::Error> {
        let mut rank: u8 = 7;
        let mut file: u8 = 0;

        let mut pieces = [Bitboard(0); 12];
        let mut white = Bitboard(0);
        let mut black = Bitboard(0);
        let mut occupied = Bitboard(0);

        for c in pos.chars() {
            if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as u8;
            } else if c.is_ascii_alphabetic() {
                match c {
                    'P' => pieces[0].0 |= 1u64 << (rank * 8 + file),
                    'N' => pieces[1].0 |= 1u64 << (rank * 8 + file),
                    'B' => pieces[2].0 |= 1u64 << (rank * 8 + file),
                    'R' => pieces[3].0 |= 1u64 << (rank * 8 + file),
                    'Q' => pieces[4].0 |= 1u64 << (rank * 8 + file),
                    'K' => pieces[5].0 |= 1u64 << (rank * 8 + file),
                    'p' => pieces[6].0 |= 1u64 << (rank * 8 + file),
                    'n' => pieces[7].0 |= 1u64 << (rank * 8 + file),
                    'b' => pieces[8].0 |= 1u64 << (rank * 8 + file),
                    'r' => pieces[9].0 |= 1u64 << (rank * 8 + file),
                    'q' => pieces[10].0 |= 1u64 << (rank * 8 + file),
                    'k' => pieces[11].0 |= 1u64 << (rank * 8 + file),
                    _ => return Err("invalid character in FEN: {c}")?,
                }

                if WHITE_PIECES.contains(c) {
                    white.0 |= 1u64 << (rank * 8 + file);
                }
                if BLACK_PIECES.contains(c) {
                    black.0 |= 1u64 << (rank * 8 + file);
                }
                occupied = white | black;

                file += 1;
            } else if c == '/' {
                rank -= 1;
                file = 0;
            }
        }

        Ok(Self {
            pieces,
            white,
            black,
            occupied,
        })
    }
}

#[derive(Debug)]
pub(crate) enum Color {
    White,
    Black,
}

impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err("invalid color")?,
        }
    }
}
