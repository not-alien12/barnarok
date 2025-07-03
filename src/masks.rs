use crate::Bitboard;

// Arrays containing precomputed masks.
const RANK_MASKS: [Bitboard; 64] = make_rank_masks();
const FILE_MASKS: [Bitboard; 64] = make_file_masks();
const ANTIDIAGONAL_MASKS: [Bitboard; 64] = make_antidiagonal_masks();
const DIAGONAL_MASKS: [Bitboard; 64] = make_diagonal_masks();
const KNIGHT_MASKS: [Bitboard; 64] = make_knight_masks();
const KING_MASKS: [Bitboard; 64] = make_king_masks();
const WHITE_KING_PAWN_MASKS: [Bitboard; 64] = make_white_king_pawn_masks();
const BLACK_KING_PAWN_MASKS: [Bitboard; 64] = make_black_king_pawn_masks();

// Getters for precomputed masks:

#[inline(always)]
pub fn rank_mask(sq: usize) -> Bitboard
{
    return RANK_MASKS[sq];
}

#[inline(always)]
pub fn file_mask(sq: usize) -> Bitboard
{
    return FILE_MASKS[sq];
}

#[inline(always)]
pub fn antidiagonal_mask(sq: usize) -> Bitboard
{
    return ANTIDIAGONAL_MASKS[sq];
}

#[inline(always)]
pub fn diagonal_mask(sq: usize) -> Bitboard
{
    return DIAGONAL_MASKS[sq];
}

#[inline(always)]
pub fn knight_mask(sq: usize) -> Bitboard
{
    return KNIGHT_MASKS[sq];
}

#[inline(always)]
pub fn king_mask(sq: usize) -> Bitboard
{
    return KING_MASKS[sq];
}

#[inline(always)]
pub fn white_king_pawn_mask(sq: usize) -> Bitboard
{
    return WHITE_KING_PAWN_MASKS[sq];
}

#[inline(always)]
pub fn black_king_pawn_mask(sq: usize) -> Bitboard
{
    return BLACK_KING_PAWN_MASKS[sq];
}

const fn make_rank_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares on the same rank as 'sq'.
    const fn create_mask(sq: usize) -> Bitboard
    {
        let r = sq / 8;
        return 0xFFu64 << (r * 8);
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_file_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares on the same file as 'sq'.
    const fn create_mask(sq: usize) -> Bitboard
    {
        // Get the file index (0 to 7).
        let f = sq % 8;
        // For each rank, add the intersection between the rank and the file to the mask.
        return (1u64 << f) * 0x01_01_01_01_01_01_01_01;
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_antidiagonal_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares on the same antidiagonal as 'sq'.
    // An antidiagonal is a diagonal spanning from southeast to northwest (\).
    const fn create_mask(sq: usize) -> Bitboard
    {
        let file = (sq % 8) as isize; // File of sq.
        let rank = (sq / 8) as isize; // Rank of sq.

        let mut mask = 0;

        let mut f = file;
        let mut r = rank;

        // Walk NW (−1 file, +1 rank) until a board edge is hit.
        while f >= 0 && r < 8
        {
            mask |= 1u64 << (r as usize * 8 + f as usize);
            f -= 1;
            r += 1;
        }

        f = file;
        r = rank;

        // Walk NW (+1 file, -1 rank) until a board edge is hit.
        while f < 8 && r >= 0
        {
            mask |= 1u64 << (r as usize * 8 + f as usize);
            f += 1;
            r -= 1;
        }

        return mask;
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_diagonal_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares on the same diagonal as 'sq'.
    // A diagonal spans from soutwest to northeast (/).
    const fn create_mask(sq: usize) -> Bitboard
    {
        let file = (sq % 8) as isize; // File of sq.
        let rank = (sq / 8) as isize; // Rank of sq.

        let mut mask = 0;

        let mut f = file;
        let mut r = rank;

        // Walk NE (+1 file, +1 rank) until a board edge is hit.
        while f < 8 && r < 8
        {
            mask |= 1u64 << (r as usize * 8 + f as usize);
            f += 1;
            r += 1;
        }

        f = file;
        r = rank;

        // Walk SW (-1 file, -1 rank) until a board edge is hit.
        while f >= 0 && r >= 0
        {
            mask |= 1u64 << (r as usize * 8 + f as usize);
            f -= 1;
            r -= 1;
        }

        return mask;
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_knight_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares that a knight could go to, starting from 'sq'.
    const fn create_mask(sq: usize) -> Bitboard
    {
        let file = (sq % 8) as isize; // File of sq.
        let rank = (sq / 8) as isize; // Rank of sq.

        let mut m = 0u64;

        if file > 1
        {
            if rank > 1
            {
                m |= 1u64 << (sq - 10);
            }
            if rank < 7
            {
                m |= 1u64 << (sq + 6);
            }
        }
        if file > 0
        {
            if rank > 2
            {
                m |= 1u64 << (sq - 17);
            }
            if rank < 6
            {
                m |= 1u64 << (sq + 15);
            }
        }
        if file < 6
        {
            if rank < 7
            {
                m |= 1u64 << (sq + 10);
            }
            if rank > 1
            {
                m |= 1u64 << (sq - 6);
            }
        }
        if file < 7
        {
            if rank < 6
            {
                m |= 1u64 << (sq + 17);
            }
            if rank > 2
            {
                m |= 1u64 << (sq - 15);
            }
        }

        return m;
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_king_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares that a king could go to, starting from 'sq'.
    const fn create_mask(sq: usize) -> Bitboard
    {
        let file = (sq % 8) as isize; // File of sq.
        let rank = (sq / 8) as isize; // Rank of sq.

        let mut m = 0u64;

        if file > 1
        {
            m |= 1u64 << (sq - 1);
            if rank > 1
            {
                m |= 1u64 << (sq - 9);
            }
            if rank < 7
            {
                m |= 1u64 << (sq + 7);
            }
        }
        if file < 7
        {
            m |= 1u64 << (sq + 1);
            if rank > 1
            {
                m |= 1u64 << (sq - 7);
            }
            if rank < 7
            {
                m |= 1u64 << (sq + 9);
            }
        }
        if rank > 1
        {
            m |= 1u64 << (sq - 8);
        }
        if rank < 7
        {
            m |= 1u64 << (sq + 8);
        }

        return m;
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_white_king_pawn_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares that white king could be attacked from.
    const fn create_mask(sq: usize) -> Bitboard
    {
        let file = (sq % 8) as isize; // File of sq.
        let rank = (sq / 8) as isize; // Rank of sq.

        let mut m = 0u64;

        if rank < 7
        {
            if file > 0
            {
                m |= 1u64 << (sq + 7);
            }
            if file < 7
            {
                m |= 1u64 << (sq + 9);
            }
        }

        return m;
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_black_king_pawn_masks() -> [Bitboard; 64]
{
    // Create a mask representing all squares that black king could be attacked from.
    const fn create_mask(sq: usize) -> Bitboard
    {
        let file = (sq % 8) as isize; // File of sq.
        let rank = (sq / 8) as isize; // Rank of sq.

        let mut m = 0u64;

        if rank > 0
        {
            if file > 0
            {
                m |= 1u64 << (sq - 9);
            }
            if file < 7
            {
                m |= 1u64 << (sq - 7);
            }
        }

        return m;
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_mask(i);
        i += 1;
    }
    return masks;
}
