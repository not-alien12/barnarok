use super::{
    chess::{Bitboard, Board, Move},
    masks::*,
};

pub fn get_legal_moves(board: &Board) -> Vec<Move>
{
    let mut all_moves = vec![];

    let mut pawn_moves = generate_pawn_moves(board);
    let mut rook_moves = generate_rook_moves_hq(board);
    let mut bishop_moves = generate_bishop_moves_hq(board);
    let mut queen_moves = generate_queen_moves_hq(board);

    all_moves.append(&mut pawn_moves);
    all_moves.append(&mut rook_moves);
    all_moves.append(&mut bishop_moves);
    all_moves.append(&mut queen_moves);

    return all_moves;
}

// Use Hyperbola-Quintessence to get the mask of squares that a sliding piece can get to.
// The mask includes the capture of a blocking piece by the sliding piece.
// But friendly pieces can also be captured according to the mask,
// so a friendly pieces mask should be subtracted before using it.
fn slider_attacks_hq(sq: usize, occ: u64, mask: u64) -> u64
{
    // Mask representing the position of the sliding piece.
    let bit = 1u64 << sq;

    // Keep only the bits on this rank/file/diag/antidiag.
    let occ_masked = occ & mask;

    // Double the sliding piece's bitboard to move the piece one square to the 'right' (along the
    // wanted axis).
    // Subtract this new mask to the occupancy mask to get a mask containing squares that the piece
    // can get to following this axis going to the 'right', up to the first blocking piece.
    let forward = occ_masked.wrapping_sub(2 * bit);

    // Repeat the operation with reversed masks, to get the squares accessible by going to the left.
    let rev_masked = occ_masked.reverse_bits();
    let rev_bit = bit.reverse_bits();
    let reverse = rev_masked.wrapping_sub(2 * rev_bit).reverse_bits();

    // Apply the XOR operation to the 2 masks (left & right), to get the attack positions in both
    // directions. We use XOR and not OR, to take out the sliding piece position.
    // Then we apply the original axis mask to prevent move overflowing in other ranks/files/diags.
    return (forward ^ reverse) & mask;
}

// Create a bitboard representing the squares a rook can get to.
// It allows the rook to capture friendly pieces, so another mask is needed after that.
pub fn rook_attacks_hq(sq: usize, occ: u64) -> u64
{
    let rm = rank_mask(sq);
    let fm = file_mask(sq);
    return slider_attacks_hq(sq, occ, rm) | slider_attacks_hq(sq, occ, fm);
}

// Create a bitboard representing the squares a bishop can get to.
// It allows the bishop to capture friendly pieces, so another mask is needed after that.
pub fn bishop_attacks_hq(sq: usize, occ: u64) -> u64
{
    let m1 = antidiagonal_mask(sq);
    let m2 = diagonal_mask(sq);
    return slider_attacks_hq(sq, occ, m1) | slider_attacks_hq(sq, occ, m2);
}

// Create a bitboard representing the squares a queen can get to.
// It allows the queen to capture friendly pieces, so another mask is needed after that.
pub fn queen_attacks_hq(sq: usize, occ: u64) -> u64
{
    return rook_attacks_hq(sq, occ) | bishop_attacks_hq(sq, occ);
}

// Create a vector containing moves that rooks on the board can make.
pub fn generate_rook_moves_hq(board: &Board) -> Vec<Move>
{
    let mut moves = Vec::new();

    // Get relevant masks.
    let occ = board.pieces;
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };
    let rooks = if board.white_to_play { board.white_rooks } else { board.black_rooks };

    // Loop over friendly rooks:
    let mut bits = rooks;
    while bits != 0
    {
        // Get the square index of the next rook.
        let from = bits.trailing_zeros() as usize;
        bits &= bits - 1;

        // Get all pseudo-legals moves the rook can make.
        let attacks = rook_attacks_hq(from, occ);

        // Forbid capturing friendly pieces.
        let targets = attacks & !friendly;

        // Add a move for each target square.
        let mut t = targets;
        while t != 0
        {
            // Get the target square index.
            let to = t.trailing_zeros() as usize;
            // Add the move.
            moves.push(Move { start: from, end: to });
            t &= t - 1;
        }
    }

    return moves;
}

// Create a vector containing moves that bishops on the board can make.
pub fn generate_bishop_moves_hq(board: &Board) -> Vec<Move>
{
    let mut moves = Vec::new();

    // Get relevant masks.
    let occ = board.pieces;
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };
    let bishops = if board.white_to_play { board.white_bishops } else { board.black_bishops };

    // Loop over friendly bishops:
    let mut bits = bishops;
    while bits != 0
    {
        // Get the square index of the next bishop.
        let from = bits.trailing_zeros() as usize;
        bits &= bits - 1;

        // Get all pseudo-legals moves the bishop can make.
        let attacks = bishop_attacks_hq(from, occ);

        // Forbid capturing friendly pieces.
        let targets = attacks & !friendly;

        // Add a move for each target square.
        let mut t = targets;
        while t != 0
        {
            // Get the target square index.
            let to = t.trailing_zeros() as usize;
            // Add the move.
            moves.push(Move { start: from, end: to });
            t &= t - 1;
        }
    }

    return moves;
}

// Create a vector containing moves that queens on the board can make.
pub fn generate_queen_moves_hq(board: &Board) -> Vec<Move>
{
    let mut moves = Vec::new();

    // Get relevant masks.
    let occ = board.pieces;
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };
    let queens = if board.white_to_play { board.white_queens } else { board.black_queens };

    // Loop over friendly queens:
    let mut bits = queens;
    while bits != 0
    {
        // Get the square index of the next queen.
        let from = bits.trailing_zeros() as usize;
        bits &= bits - 1;

        // Get all pseudo-legals moves the queen can make.
        let attacks = queen_attacks_hq(from, occ);

        // Forbid capturing friendly pieces.
        let targets = attacks & !friendly;

        // Add a move for each target square.
        let mut t = targets;
        while t != 0
        {
            // Get the target square index.
            let to = t.trailing_zeros() as usize;
            // Add the move.
            moves.push(Move { start: from, end: to });
            t &= t - 1;
        }
    }

    return moves;
}

// Generate legal moves for pawns.
pub fn generate_pawn_moves(board: &Board) -> Vec<Move>
{
    // Create a vector representing legal moves for pawns.
    let mut moves = Vec::new();

    // Create a bitboard representing empty squares.
    let empty = !board.pieces;

    // Create masks to filter legal moves only.
    // The following file masks allow us to prevent wrapping captures (when a pawn flies to the
    // other side of the board).
    const FILE_A: u64 = 0x01_01_01_01_01_01_01_01;
    const FILE_H: u64 = 0x80_80_80_80_80_80_80_80;
    // The following rank masks allow us to verify that a pawn is allowed to move 2 squares forward.
    const RANK_2: u64 = 0x00_00_00_00_00_00_FF_00;
    const RANK_7: u64 = 0x00_FF_00_00_00_00_00_00;
    // The following rank masks allow us to verify that a pawn is in a situation where an en passant
    // capture may be possible.
    const RANK_5: u64 = 0x00_00_00_FF_00_00_00_00;
    const RANK_4: u64 = 0x00_00_00_00_FF_00_00_00;

    // Get moves for white pawns.
    if board.white_to_play
    {
        let wp = board.white_pawns;

        // Create a bitboard representing squares that pawns can go to by moving one square forward.
        // The square ahead must be free.
        let singles = (wp << 8) & empty;
        bitboard_to_moves(singles, 8, &mut moves);

        // Create a bitboard representing squares that pawns can go to by moving two squares
        // forward. The two squares ahead must be free, and the pawn must be on rank 2.
        let doubles = ((wp & RANK_2) << 16) & empty & (empty << 8);
        bitboard_to_moves(doubles, 16, &mut moves);

        // Create two bitboards representing squares that pawns can go to by capturing a black
        // piece. For each bitboard, the corresponding diagonal square must contain a black
        // piece.
        let cap_nw = ((wp & !FILE_A) << 7) & board.black_pieces;
        let cap_ne = ((wp & !FILE_H) << 9) & board.black_pieces;
        bitboard_to_moves(cap_nw, 7, &mut moves);
        bitboard_to_moves(cap_ne, 9, &mut moves);

        // If a black pawn moved two squares forward last ply, an en passant capture is possible
        // this ply.
        if let Some(ep_sq) = board.en_passant_target
            && ep_sq > 39
        {
            // Get a bitboard representing the position of the target tile.
            let ep_bb = 1u64 << ep_sq;

            // Create a bitboard representing a potential pawn that could take the target black pawn
            // from the left.
            let ep_from_left = ((wp & RANK_5 & !FILE_A) << 7) & ep_bb;
            bitboard_to_moves(ep_from_left, 7, &mut moves);

            // Create a bitboard representing a potential pawn that could take the target black pawn
            // from the right.
            let ep_from_right = ((wp & RANK_5 & !FILE_H) << 9) & ep_bb;
            bitboard_to_moves(ep_from_right, 9, &mut moves);
        }
    }
    // Get moves for black pawns.
    else
    {
        let bp = board.black_pawns;

        // Create a bitboard representing squares that pawns can go to by moving one square forward.
        // The square ahead must be free.
        let singles = (bp >> 8) & empty;
        bitboard_to_moves(singles, -8, &mut moves);

        // Create a bitboard representing squares that pawns can go to by moving two squares
        // forward. The two squares ahead must be free, and the pawn must be on rank 7.
        let doubles = ((bp & RANK_7) >> 16) & empty & (empty >> 8);
        bitboard_to_moves(doubles, -16, &mut moves);

        // Create two bitboards representing squares that pawns can go to by capturing a white
        // piece. For each bitboard, the corresponding diagonal square must contain a white
        // piece.
        let cap_sw = ((bp & !FILE_A) >> 9) & board.white_pieces;
        let cap_se = ((bp & !FILE_H) >> 7) & board.white_pieces;
        bitboard_to_moves(cap_sw, -9, &mut moves);
        bitboard_to_moves(cap_se, -7, &mut moves);

        // If a white pawn moved two squares forward last ply, an en passant capture is possible
        // this ply.
        if let Some(ep_sq) = board.en_passant_target
            && ep_sq < 24
        {
            // Get a bitboard representing the position of the target tile.
            let ep_bb = 1u64 << ep_sq;

            // Create a bitboard representing a potential pawn that could take the target white pawn
            // from the right.
            let ep_from_right = ((bp & RANK_4 & !FILE_H) >> 7) & ep_bb;
            bitboard_to_moves(ep_from_right, -7, &mut moves);

            // Create a bitboard representing a potential pawn that could take the target white pawn
            // from the left.
            let ep_from_left = ((bp & RANK_4 & !FILE_A) >> 9) & ep_bb;
            bitboard_to_moves(ep_from_left, -9, &mut moves);
        }
    }

    return moves;
}

// For a given move type (represented by a shift value), and a given destination bitboard,
// this helper creates a move and adds it to a vector.
fn bitboard_to_moves(to_bb: Bitboard, shift: isize, out: &mut Vec<Move>)
{
    // Copy the bitboard to a mutable value.
    let mut bits = to_bb;
    // While the remaining bitboard is not null, there are moves to add.
    while bits != 0
    {
        // Get the index of the first '1' in the remaining bitboard, starting from the end.
        // This index is that of a square that a piece can go to.
        let to = bits.trailing_zeros() as usize;
        // Get the index of the square the moving piece is currently on, using the shift
        // value passed for each move type.
        let from = ((to as isize) - shift) as usize;
        // Add the calculated move to the vector.
        out.push(Move { start: from, end: to });
        // Remove the last bit of the bitboard.
        bits &= bits - 1;
    }
}
