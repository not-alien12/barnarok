const fn make_rank_masks() -> [u64; 64]
{
    // Create a mask representing all squares on the same rank as 'sq'.
    const fn create_rank_mask(sq: usize) -> u64
    {
        let r = sq / 8;
        return 0xFFu64 << (r * 8);
    }

    // Use the helper to fill the array with masks.
    let mut masks = [0u64; 64];
    let mut i = 0;
    while i < 64
    {
        masks[i] = create_rank_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_file_masks() -> [u64; 64]
{
    // Create a mask representing all squares on the same file as 'sq'.
    const fn create_file_mask(sq: usize) -> u64
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
        masks[i] = create_file_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_antidiagonal_masks() -> [u64; 64]
{
    // Create a mask representing all squares on the same antidiagonal as 'sq'.
    // An antidiagonal is a diagonal spanning from southeast to northwest (\).
    const fn create_antidiagonal_mask(sq: usize) -> u64
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
        masks[i] = create_antidiagonal_mask(i);
        i += 1;
    }
    return masks;
}

const fn make_diagonal_masks() -> [u64; 64]
{
    // Create a mask representing all squares on the same diagonal as 'sq'.
    // A diagonal spans from soutwest to northeast (/).
    const fn create_diagonal_mask(sq: usize) -> u64
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
        masks[i] = create_diagonal_mask(i);
        i += 1;
    }
    return masks;
}

// Arrays containing precomputed masks.
const RANK_MASKS: [u64; 64] = make_rank_masks();
const FILE_MASKS: [u64; 64] = make_file_masks();
const ANTIDIAGONAL_MASKS: [u64; 64] = make_antidiagonal_masks();
const DIAGONAL_MASKS: [u64; 64] = make_diagonal_masks();

// Getters for precomputed masks:

#[inline(always)]
pub fn rank_mask(sq: usize) -> u64
{
    return RANK_MASKS[sq];
}

#[inline(always)]
pub fn file_mask(sq: usize) -> u64
{
    return FILE_MASKS[sq];
}

#[inline(always)]
pub fn antidiagonal_mask(sq: usize) -> u64
{
    return ANTIDIAGONAL_MASKS[sq];
}

#[inline(always)]
pub fn diagonal_mask(sq: usize) -> u64
{
    return DIAGONAL_MASKS[sq];
}