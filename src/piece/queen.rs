use crate::{
    Bitboard, Board, Move, MoveContext, get_piece_type_on_square, is_king_attacked,
    piece::{bishop::*, rook::*},
};

// Generate a bitboard representing squares attacked by the queens of the player that just played.
pub fn generate_queen_attacks(board: &Board) -> Bitboard
{
    // Empty bitboard.
    let mut m = 0u64;

    // Get relevant masks.
    let occ = board.pieces;
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };
    let queens = if board.white_to_play { board.white_queens } else { board.black_queens };

    // Loop over the bishops.
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

        // Fill the bitboard with the attacked squares.
        let mut t = targets;
        while t != 0
        {
            // Get the target square index.
            let to = t.trailing_zeros() as usize;
            // Add a bit.
            m |= 1u64 << to;
            t &= t - 1;
        }
    }

    return m;
}

// Create a vector containing moves that queens on the board can make.
pub fn generate_queen_moves_hq(board: &mut Board) -> Vec<Move>
{
    let mut moves = Vec::new();

    // Get relevant masks.
    let occ = board.pieces;
    let enemy = if board.white_to_play { board.black_pieces } else { board.white_pieces };
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
            let to_mask = 1u64 << to;

            // Create the move.
            let mv = Move {
                start: from,
                end: to,
                context: MoveContext::None,
                previous_ep_target: board.en_passant_target,
                previous_wqs: board.white_queen_side_castling_right,
                previous_wks: board.white_king_side_castling_right,
                previous_bqs: board.black_queen_side_castling_right,
                previous_bks: board.black_king_side_castling_right,
                capture: if enemy & to_mask != 0
                {
                    Some(get_piece_type_on_square(board, to))
                }
                else
                {
                    None
                },
            };
            
            board.make_move(mv);
            // Add the move only if the king is not in check.
            if !is_king_attacked(&board, true)
            {
                moves.push(mv);
            }
            board.unmake_move(mv);
            t &= t - 1;
        }
    }

    return moves;
}

// Create a bitboard representing the squares a queen can get to.
// It allows the queen to capture friendly pieces, so another mask is needed after that.
pub fn queen_attacks_hq(sq: usize, occ: u64) -> u64
{
    return rook_attacks_hq(sq, occ) | bishop_attacks_hq(sq, occ);
}
