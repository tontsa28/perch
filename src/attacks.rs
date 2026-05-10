use crate::bitboard::Bitboard;

/// Precomputed attacks for white pawns indexed by square.
pub(crate) const WHITE_PAWN_ATTACKS: [Bitboard; 64] = [
    Bitboard::new(0x0000_0000_0000_0200),
    Bitboard::new(0x0000_0000_0000_0500),
    Bitboard::new(0x0000_0000_0000_0a00),
    Bitboard::new(0x0000_0000_0000_1400),
    Bitboard::new(0x0000_0000_0000_2800),
    Bitboard::new(0x0000_0000_0000_5000),
    Bitboard::new(0x0000_0000_0000_a000),
    Bitboard::new(0x0000_0000_0000_4000),
    Bitboard::new(0x0000_0000_0002_0000),
    Bitboard::new(0x0000_0000_0005_0000),
    Bitboard::new(0x0000_0000_000a_0000),
    Bitboard::new(0x0000_0000_0014_0000),
    Bitboard::new(0x0000_0000_0028_0000),
    Bitboard::new(0x0000_0000_0050_0000),
    Bitboard::new(0x0000_0000_00a0_0000),
    Bitboard::new(0x0000_0000_0040_0000),
    Bitboard::new(0x0000_0000_0200_0000),
    Bitboard::new(0x0000_0000_0500_0000),
    Bitboard::new(0x0000_0000_0a00_0000),
    Bitboard::new(0x0000_0000_1400_0000),
    Bitboard::new(0x0000_0000_2800_0000),
    Bitboard::new(0x0000_0000_5000_0000),
    Bitboard::new(0x0000_0000_a000_0000),
    Bitboard::new(0x0000_0000_4000_0000),
    Bitboard::new(0x0000_0002_0000_0000),
    Bitboard::new(0x0000_0005_0000_0000),
    Bitboard::new(0x0000_000a_0000_0000),
    Bitboard::new(0x0000_0014_0000_0000),
    Bitboard::new(0x0000_0028_0000_0000),
    Bitboard::new(0x0000_0050_0000_0000),
    Bitboard::new(0x0000_00a0_0000_0000),
    Bitboard::new(0x0000_0040_0000_0000),
    Bitboard::new(0x0000_0200_0000_0000),
    Bitboard::new(0x0000_0500_0000_0000),
    Bitboard::new(0x0000_0a00_0000_0000),
    Bitboard::new(0x0000_1400_0000_0000),
    Bitboard::new(0x0000_2800_0000_0000),
    Bitboard::new(0x0000_5000_0000_0000),
    Bitboard::new(0x0000_a000_0000_0000),
    Bitboard::new(0x0000_4000_0000_0000),
    Bitboard::new(0x0002_0000_0000_0000),
    Bitboard::new(0x0005_0000_0000_0000),
    Bitboard::new(0x000a_0000_0000_0000),
    Bitboard::new(0x0014_0000_0000_0000),
    Bitboard::new(0x0028_0000_0000_0000),
    Bitboard::new(0x0050_0000_0000_0000),
    Bitboard::new(0x00a0_0000_0000_0000),
    Bitboard::new(0x0040_0000_0000_0000),
    Bitboard::new(0x0200_0000_0000_0000),
    Bitboard::new(0x0500_0000_0000_0000),
    Bitboard::new(0x0a00_0000_0000_0000),
    Bitboard::new(0x1400_0000_0000_0000),
    Bitboard::new(0x2800_0000_0000_0000),
    Bitboard::new(0x5000_0000_0000_0000),
    Bitboard::new(0xa000_0000_0000_0000),
    Bitboard::new(0x4000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
];

/// Precomputed attacks for black pawns indexed by square.
pub(crate) const BLACK_PAWN_ATTACKS: [Bitboard; 64] = [
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0000),
    Bitboard::new(0x0000_0000_0000_0002),
    Bitboard::new(0x0000_0000_0000_0005),
    Bitboard::new(0x0000_0000_0000_000a),
    Bitboard::new(0x0000_0000_0000_0014),
    Bitboard::new(0x0000_0000_0000_0028),
    Bitboard::new(0x0000_0000_0000_0050),
    Bitboard::new(0x0000_0000_0000_00a0),
    Bitboard::new(0x0000_0000_0000_0040),
    Bitboard::new(0x0000_0000_0000_0200),
    Bitboard::new(0x0000_0000_0000_0500),
    Bitboard::new(0x0000_0000_0000_0a00),
    Bitboard::new(0x0000_0000_0000_1400),
    Bitboard::new(0x0000_0000_0000_2800),
    Bitboard::new(0x0000_0000_0000_5000),
    Bitboard::new(0x0000_0000_0000_a000),
    Bitboard::new(0x0000_0000_0000_4000),
    Bitboard::new(0x0000_0000_0002_0000),
    Bitboard::new(0x0000_0000_0005_0000),
    Bitboard::new(0x0000_0000_000a_0000),
    Bitboard::new(0x0000_0000_0014_0000),
    Bitboard::new(0x0000_0000_0028_0000),
    Bitboard::new(0x0000_0000_0050_0000),
    Bitboard::new(0x0000_0000_00a0_0000),
    Bitboard::new(0x0000_0000_0040_0000),
    Bitboard::new(0x0000_0000_0200_0000),
    Bitboard::new(0x0000_0000_0500_0000),
    Bitboard::new(0x0000_0000_0a00_0000),
    Bitboard::new(0x0000_0000_1400_0000),
    Bitboard::new(0x0000_0000_2800_0000),
    Bitboard::new(0x0000_0000_5000_0000),
    Bitboard::new(0x0000_0000_a000_0000),
    Bitboard::new(0x0000_0000_4000_0000),
    Bitboard::new(0x0000_0002_0000_0000),
    Bitboard::new(0x0000_0005_0000_0000),
    Bitboard::new(0x0000_000a_0000_0000),
    Bitboard::new(0x0000_0014_0000_0000),
    Bitboard::new(0x0000_0028_0000_0000),
    Bitboard::new(0x0000_0050_0000_0000),
    Bitboard::new(0x0000_00a0_0000_0000),
    Bitboard::new(0x0000_0040_0000_0000),
    Bitboard::new(0x0000_0200_0000_0000),
    Bitboard::new(0x0000_0500_0000_0000),
    Bitboard::new(0x0000_0a00_0000_0000),
    Bitboard::new(0x0000_1400_0000_0000),
    Bitboard::new(0x0000_2800_0000_0000),
    Bitboard::new(0x0000_5000_0000_0000),
    Bitboard::new(0x0000_a000_0000_0000),
    Bitboard::new(0x0000_4000_0000_0000),
    Bitboard::new(0x0002_0000_0000_0000),
    Bitboard::new(0x0005_0000_0000_0000),
    Bitboard::new(0x000a_0000_0000_0000),
    Bitboard::new(0x0014_0000_0000_0000),
    Bitboard::new(0x0028_0000_0000_0000),
    Bitboard::new(0x0050_0000_0000_0000),
    Bitboard::new(0x00a0_0000_0000_0000),
    Bitboard::new(0x0040_0000_0000_0000),
];

/// Precomputed attacks for knights indexed by square.
pub(crate) const KNIGHT_ATTACKS: [Bitboard; 64] = [
    Bitboard::new(0x0000_0000_0002_0400),
    Bitboard::new(0x0000_0000_0005_0800),
    Bitboard::new(0x0000_0000_000a_1100),
    Bitboard::new(0x0000_0000_0014_2200),
    Bitboard::new(0x0000_0000_0028_4400),
    Bitboard::new(0x0000_0000_0050_8800),
    Bitboard::new(0x0000_0000_00a0_1000),
    Bitboard::new(0x0000_0000_0040_2000),
    Bitboard::new(0x0000_0000_0204_0004),
    Bitboard::new(0x0000_0000_0508_0008),
    Bitboard::new(0x0000_0000_0a11_0011),
    Bitboard::new(0x0000_0000_1422_0022),
    Bitboard::new(0x0000_0000_2844_0044),
    Bitboard::new(0x0000_0000_5088_0088),
    Bitboard::new(0x0000_0000_a010_0010),
    Bitboard::new(0x0000_0000_4020_0020),
    Bitboard::new(0x0000_0002_0400_0402),
    Bitboard::new(0x0000_0005_0800_0805),
    Bitboard::new(0x0000_000a_1100_110a),
    Bitboard::new(0x0000_0014_2200_2214),
    Bitboard::new(0x0000_0028_4400_4428),
    Bitboard::new(0x0000_0050_8800_8850),
    Bitboard::new(0x0000_00a0_1000_10a0),
    Bitboard::new(0x0000_0040_2000_2040),
    Bitboard::new(0x0000_0204_0004_0200),
    Bitboard::new(0x0000_0508_0008_0500),
    Bitboard::new(0x0000_0a11_0011_0a00),
    Bitboard::new(0x0000_1422_0022_1400),
    Bitboard::new(0x0000_2844_0044_2800),
    Bitboard::new(0x0000_5088_0088_5000),
    Bitboard::new(0x0000_a010_0010_a000),
    Bitboard::new(0x0000_4020_0020_4000),
    Bitboard::new(0x0002_0400_0402_0000),
    Bitboard::new(0x0005_0800_0805_0000),
    Bitboard::new(0x000a_1100_110a_0000),
    Bitboard::new(0x0014_2200_2214_0000),
    Bitboard::new(0x0028_4400_4428_0000),
    Bitboard::new(0x0050_8800_8850_0000),
    Bitboard::new(0x00a0_1000_10a0_0000),
    Bitboard::new(0x0040_2000_2040_0000),
    Bitboard::new(0x0204_0004_0200_0000),
    Bitboard::new(0x0508_0008_0500_0000),
    Bitboard::new(0x0a11_0011_0a00_0000),
    Bitboard::new(0x1422_0022_1400_0000),
    Bitboard::new(0x2844_0044_2800_0000),
    Bitboard::new(0x5088_0088_5000_0000),
    Bitboard::new(0xa010_0010_a000_0000),
    Bitboard::new(0x4020_0020_4000_0000),
    Bitboard::new(0x0400_0402_0000_0000),
    Bitboard::new(0x0800_0805_0000_0000),
    Bitboard::new(0x1100_110a_0000_0000),
    Bitboard::new(0x2200_2214_0000_0000),
    Bitboard::new(0x4400_4428_0000_0000),
    Bitboard::new(0x8800_8850_0000_0000),
    Bitboard::new(0x1000_10a0_0000_0000),
    Bitboard::new(0x2000_2040_0000_0000),
    Bitboard::new(0x0004_0200_0000_0000),
    Bitboard::new(0x0008_0500_0000_0000),
    Bitboard::new(0x0011_0a00_0000_0000),
    Bitboard::new(0x0022_1400_0000_0000),
    Bitboard::new(0x0044_2800_0000_0000),
    Bitboard::new(0x0088_5000_0000_0000),
    Bitboard::new(0x0010_a000_0000_0000),
    Bitboard::new(0x0020_4000_0000_0000),
];

/// Precomputed attacks for kings indexed by square.
pub(crate) const KING_ATTACKS: [Bitboard; 64] = [
    Bitboard::new(0x0000_0000_0000_0302),
    Bitboard::new(0x0000_0000_0000_0705),
    Bitboard::new(0x0000_0000_0000_0e0a),
    Bitboard::new(0x0000_0000_0000_1c14),
    Bitboard::new(0x0000_0000_0000_3828),
    Bitboard::new(0x0000_0000_0000_7050),
    Bitboard::new(0x0000_0000_0000_e0a0),
    Bitboard::new(0x0000_0000_0000_c040),
    Bitboard::new(0x0000_0000_0003_0203),
    Bitboard::new(0x0000_0000_0007_0507),
    Bitboard::new(0x0000_0000_000e_0a0e),
    Bitboard::new(0x0000_0000_001c_141c),
    Bitboard::new(0x0000_0000_0038_2838),
    Bitboard::new(0x0000_0000_0070_5070),
    Bitboard::new(0x0000_0000_00e0_a0e0),
    Bitboard::new(0x0000_0000_00c0_40c0),
    Bitboard::new(0x0000_0000_0302_0300),
    Bitboard::new(0x0000_0000_0705_0700),
    Bitboard::new(0x0000_0000_0e0a_0e00),
    Bitboard::new(0x0000_0000_1c14_1c00),
    Bitboard::new(0x0000_0000_3828_3800),
    Bitboard::new(0x0000_0000_7050_7000),
    Bitboard::new(0x0000_0000_e0a0_e000),
    Bitboard::new(0x0000_0000_c040_c000),
    Bitboard::new(0x0000_0003_0203_0000),
    Bitboard::new(0x0000_0007_0507_0000),
    Bitboard::new(0x0000_000e_0a0e_0000),
    Bitboard::new(0x0000_001c_141c_0000),
    Bitboard::new(0x0000_0038_2838_0000),
    Bitboard::new(0x0000_0070_5070_0000),
    Bitboard::new(0x0000_00e0_a0e0_0000),
    Bitboard::new(0x0000_00c0_40c0_0000),
    Bitboard::new(0x0000_0302_0300_0000),
    Bitboard::new(0x0000_0705_0700_0000),
    Bitboard::new(0x0000_0e0a_0e00_0000),
    Bitboard::new(0x0000_1c14_1c00_0000),
    Bitboard::new(0x0000_3828_3800_0000),
    Bitboard::new(0x0000_7050_7000_0000),
    Bitboard::new(0x0000_e0a0_e000_0000),
    Bitboard::new(0x0000_c040_c000_0000),
    Bitboard::new(0x0003_0203_0000_0000),
    Bitboard::new(0x0007_0507_0000_0000),
    Bitboard::new(0x000e_0a0e_0000_0000),
    Bitboard::new(0x001c_141c_0000_0000),
    Bitboard::new(0x0038_2838_0000_0000),
    Bitboard::new(0x0070_5070_0000_0000),
    Bitboard::new(0x00e0_a0e0_0000_0000),
    Bitboard::new(0x00c0_40c0_0000_0000),
    Bitboard::new(0x0302_0300_0000_0000),
    Bitboard::new(0x0705_0700_0000_0000),
    Bitboard::new(0x0e0a_0e00_0000_0000),
    Bitboard::new(0x1c14_1c00_0000_0000),
    Bitboard::new(0x3828_3800_0000_0000),
    Bitboard::new(0x7050_7000_0000_0000),
    Bitboard::new(0xe0a0_e000_0000_0000),
    Bitboard::new(0xc040_c000_0000_0000),
    Bitboard::new(0x0203_0000_0000_0000),
    Bitboard::new(0x0507_0000_0000_0000),
    Bitboard::new(0x0a0e_0000_0000_0000),
    Bitboard::new(0x141c_0000_0000_0000),
    Bitboard::new(0x2838_0000_0000_0000),
    Bitboard::new(0x5070_0000_0000_0000),
    Bitboard::new(0xa0e0_0000_0000_0000),
    Bitboard::new(0x40c0_0000_0000_0000),
];

/// All possible slider directions represented as compass points.
pub(crate) const N: usize = 0;
pub(crate) const S: usize = 1;
pub(crate) const E: usize = 2;
pub(crate) const W: usize = 3;
pub(crate) const NE: usize = 4;
pub(crate) const NW: usize = 5;
pub(crate) const SE: usize = 6;
pub(crate) const SW: usize = 7;

pub(crate) const RAYS: [[Bitboard; 8]; 64] = gen_rays();

/// Check if file and rank produce a valid square on the board.
///
/// # Parameters
/// - `file`: File index (0..7).
/// - `rank`: Rank index (0..7).
///
/// # Returns
/// `true` if `(file, rank)` is on the board.
const fn on_board(file: i8, rank: i8) -> bool {
    file >= 0 && file < 8 && rank >= 0 && rank < 8
}

/// Generate a ray towards a given direction.
///
/// # Parameters
/// - `sq`: Origin square.
/// - `df`: File delta per step.
/// - `dr`: Rank delta per step.
///
/// # Returns
/// A bitboard containing all squares on the ray.
const fn gen_ray_from(sq: u8, df: i8, dr: i8) -> Bitboard {
    let mut mask = 0u64;

    // Compute initial file and rank from the square integer
    let f0 = (sq % 8) as i8;
    let r0 = (sq / 8) as i8;

    // Compute dynamic file and rank
    let mut f = f0 + df;
    let mut r = r0 + dr;

    while on_board(f, r) {
        // Compute destination square
        let to = (r as u8) * 8 + (f as u8);

        // Convert the destination square to a bitboard and combine it with the mask
        mask |= 1u64 << to;

        // Increment to the next slider square
        f += df;
        r += dr;
    }

    Bitboard::new(mask)
}

/// Generate all rays, from all squares to all directions.
///
/// # Returns
/// A 64x8 table of rays indexed by square and direction.
const fn gen_rays() -> [[Bitboard; 8]; 64] {
    let mut rays = [[Bitboard::EMPTY; 8]; 64];
    let mut sq = 0u8;

    while sq < 64 {
        // Generate rays to all directions from the given square
        rays[sq as usize][N] = gen_ray_from(sq, 0, 1);
        rays[sq as usize][S] = gen_ray_from(sq, 0, -1);
        rays[sq as usize][E] = gen_ray_from(sq, 1, 0);
        rays[sq as usize][W] = gen_ray_from(sq, -1, 0);
        rays[sq as usize][NE] = gen_ray_from(sq, 1, 1);
        rays[sq as usize][NW] = gen_ray_from(sq, -1, 1);
        rays[sq as usize][SE] = gen_ray_from(sq, 1, -1);
        rays[sq as usize][SW] = gen_ray_from(sq, -1, -1);

        sq += 1;
    }

    rays
}
