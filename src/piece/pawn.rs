use crate::{Bitboard, Board, Move, MoveContext, get_piece_type_on_square, is_king_attacked};

// Generate legal moves for pawns.
pub fn generate_pawn_attacks(board: &Board) -> Bitboard
{
    // Create a vector representing legal moves for pawns.
    let mut m = 0u64;

    // Create masks to filter legal moves only.
    // The following file masks allow us to prevent wrapping captures (when a pawn flies to the
    // other side of the board).
    const FILE_A: u64 = 0x01_01_01_01_01_01_01_01;
    const FILE_H: u64 = 0x80_80_80_80_80_80_80_80;
    // The following rank masks allow us to verify that a pawn is in a situation where an en passant
    // capture may be possible.
    const RANK_5: u64 = 0x00_00_00_FF_00_00_00_00;
    const RANK_4: u64 = 0x00_00_00_00_FF_00_00_00;

    // Get moves for white pawns.
    if board.white_to_play
    {
        let wp = board.white_pawns;

        // Add two bitboards representing squares that pawns can go to by capturing a black
        // piece. For each bitboard, the corresponding diagonal square must contain a black
        // piece.
        m |= (wp & !FILE_A) << 7;
        m |= (wp & !FILE_H) << 9;

        // If a black pawn moved two squares forward last ply, an en passant capture is possible
        // this ply.
        if let Some(ep_sq) = board.en_passant_target
            && ep_sq > 39
        {
            // Get a bitboard representing the position of the target tile.
            let ep_bb = 1u64 << ep_sq;

            // Create a bitboard representing a potential pawn that could take the target black pawn
            // from the right.
            let ep_from_right = ((wp & RANK_5 & !FILE_A) << 7) & ep_bb;
            m |= ep_from_right >> 8;

            // Create a bitboard representing a potential pawn that could take the target black pawn
            // from the left.
            let ep_from_left = ((wp & RANK_5 & !FILE_H) << 9) & ep_bb;
            m |= ep_from_left >> 8;
        }
    }
    // Get moves for black pawns.
    else
    {
        let bp = board.black_pawns;

        // Add two bitboards representing squares that pawns can go to by capturing a white
        // piece. For each bitboard, the corresponding diagonal square must contain a white
        // piece.
        m |= (bp & !FILE_A) >> 9;
        m |= (bp & !FILE_H) >> 7;

        // If a white pawn moved two squares forward last ply, an en passant capture is possible
        // this ply.
        if let Some(ep_sq) = board.en_passant_target
            && ep_sq < 24
        {
            // Get a bitboard representing the position of the target tile.
            let ep_bb = 1u64 << ep_sq;

            // Create a bitboard representing a potential pawn that could take the target white pawn
            // from the right.
            let ep_from_right = ((bp & RANK_4 & !FILE_A) >> 7) & ep_bb;
            m |= ep_from_right << 8;

            // Create a bitboard representing a potential pawn that could take the target white pawn
            // from the left.
            let ep_from_left = ((bp & RANK_4 & !FILE_H) >> 9) & ep_bb;
            m |= ep_from_left << 8;
        }
    }

    return m;
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
        bitboard_to_moves(board, singles, 8, &mut moves, false);

        // Create a bitboard representing squares that pawns can go to by moving two squares
        // forward. The two squares ahead must be free, and the pawn must be on rank 2.
        let doubles = ((wp & RANK_2) << 16) & empty & (empty << 8);
        bitboard_to_moves(board, doubles, 16, &mut moves, false);

        // Create two bitboards representing squares that pawns can go to by capturing a black
        // piece. For each bitboard, the corresponding diagonal square must contain a black
        // piece.
        let cap_nw = ((wp & !FILE_A) << 7) & board.black_pieces;
        let cap_ne = ((wp & !FILE_H) << 9) & board.black_pieces;
        bitboard_to_moves(board, cap_nw, 7, &mut moves, false);
        bitboard_to_moves(board, cap_ne, 9, &mut moves, false);

        // If a black pawn moved two squares forward last ply, an en passant capture is possible
        // this ply.
        if let Some(ep_sq) = board.en_passant_target
            && ep_sq > 39
        {
            // Get a bitboard representing the position of the target tile.
            let ep_bb = 1u64 << ep_sq;

            // Create a bitboard representing a potential pawn that could take the target black pawn
            // from the right.
            let ep_from_right = ((wp & RANK_5 & !FILE_A) << 7) & ep_bb;
            bitboard_to_moves(board, ep_from_right, 7, &mut moves, true);

            // Create a bitboard representing a potential pawn that could take the target black pawn
            // from the left.
            let ep_from_left = ((wp & RANK_5 & !FILE_H) << 9) & ep_bb;
            bitboard_to_moves(board, ep_from_left, 9, &mut moves, true);
        }
    }
    // Get moves for black pawns.
    else
    {
        let bp = board.black_pawns;

        // Create a bitboard representing squares that pawns can go to by moving one square forward.
        // The square ahead must be free.
        let singles = (bp >> 8) & empty;
        bitboard_to_moves(board, singles, -8, &mut moves, false);

        // Create a bitboard representing squares that pawns can go to by moving two squares
        // forward. The two squares ahead must be free, and the pawn must be on rank 7.
        let doubles = ((bp & RANK_7) >> 16) & empty & (empty >> 8);
        bitboard_to_moves(board, doubles, -16, &mut moves, false);

        // Create two bitboards representing squares that pawns can go to by capturing a white
        // piece. For each bitboard, the corresponding diagonal square must contain a white
        // piece.
        let cap_sw = ((bp & !FILE_A) >> 9) & board.white_pieces;
        let cap_se = ((bp & !FILE_H) >> 7) & board.white_pieces;
        bitboard_to_moves(board, cap_sw, -9, &mut moves, false);
        bitboard_to_moves(board, cap_se, -7, &mut moves, false);

        // If a white pawn moved two squares forward last ply, an en passant capture is possible
        // this ply.
        if let Some(ep_sq) = board.en_passant_target
            && ep_sq < 24
        {
            // Get a bitboard representing the position of the target tile.
            let ep_bb = 1u64 << ep_sq;

            // Create a bitboard representing a potential pawn that could take the target white pawn
            // from the right.
            let ep_from_right = ((bp & RANK_4 & !FILE_A) >> 7) & ep_bb;
            bitboard_to_moves(board, ep_from_right, -7, &mut moves, true);

            // Create a bitboard representing a potential pawn that could take the target white pawn
            // from the left.
            let ep_from_left = ((bp & RANK_4 & !FILE_H) >> 9) & ep_bb;
            bitboard_to_moves(board, ep_from_left, -9, &mut moves, true);
        }
    }

    return moves;
}

// For a given move type (represented by a shift value), and a given destination bitboard,
// this helper creates a move and adds it to a vector.
fn bitboard_to_moves(board: &Board, to_bb: Bitboard, shift: isize, out: &mut Vec<Move>, ep: bool)
{
    // Copy the bitboard to a mutable value.
    let mut bits = to_bb;
    // Get the bitboard of enemy pieces.
    let enemy = if board.white_to_play { board.black_pieces } else { board.white_pieces };
    // While the remaining bitboard is not null, there are moves to add.
    while bits != 0
    {
        // Get the index of the first '1' in the remaining bitboard, starting from the end.
        // This index is that of a square that a piece can go to.
        let to = bits.trailing_zeros() as usize;
        let to_mask = 1u64 << to;
        let ctx = if (enemy & to_mask) != 0
        {
            MoveContext::Capture(get_piece_type_on_square(board, to))
        }
        else
        {
            MoveContext::None
        };
        // Get the index of the square the moving piece is currently on, using the shift
        // value passed for each move type.
        let from = ((to as isize) - shift) as usize;
        // Create a temporary copy of the board to test the validity of the move.
        let mut temp = board.clone();
        let mv = Move {
            start: from,
            end: to,
            context: if ep
            {
                MoveContext::EnPassant
            }
            else if shift == -16 || shift == 16
            {
                MoveContext::DoubleStep
            }
            else
            {
                ctx
            },
            previous_ep_target: board.en_passant_target,
        };
        temp.make_move(mv);
        // Add the move only if the king is not in check.
        if !is_king_attacked(&temp, false)
        {
            out.push(mv);
        }
        // Remove the last bit of the bitboard.
        bits &= bits - 1;
    }
}
