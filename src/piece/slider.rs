// Use Hyperbola-Quintessence to get the mask of squares that a sliding piece can get to.
// The mask includes the capture of a blocking piece by the sliding piece.
// But friendly pieces can also be captured according to the mask,
// so a friendly pieces mask should be subtracted before using it.
pub fn slider_attacks_hq(sq: usize, occ: u64, mask: u64) -> u64
{
    // Mask representing the position of the sliding piece.
    let bit = 1u64 << sq;

    // Keep only the bits on this rank/file/diag/antidiag.
    let occ_masked = occ & mask;

    // Double the sliding piece's bitboard to move the piece one square to the 'right' (along the
    // wanted axis).
    // Subtract this new mask to the occupancy mask to get a mask containing squares that the piece
    // can get to following this axis going to the 'right', up to the first blocking piece.
    let forward = occ_masked.wrapping_sub(bit << 1);

    // Repeat the operation with reversed masks, to get the squares accessible by going to the left.
    let rev_masked = occ_masked.reverse_bits();
    let rev_bit = bit.reverse_bits();
    let reverse = rev_masked.wrapping_sub(rev_bit << 1).reverse_bits();

    // Apply the XOR operation to the 2 masks (left & right), to get the attack positions in both
    // directions. We use XOR and not OR, to take out the sliding piece position.
    // Then we apply the original axis mask to prevent move overflowing in other ranks/files/diags.
    return (forward ^ reverse) & mask;
}
