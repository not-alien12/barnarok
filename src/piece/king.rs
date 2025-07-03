use crate::{
    Bitboard, Board, Move, MoveContext, get_piece_type_on_square, is_king_attacked, masks::*,
};

// Generate a bitboard representing squares attacked by the king of the player that just played.
pub fn generate_king_attacks(board: &Board) -> Bitboard
{
    // Empty bitboard.
    let mut m = 0u64;

    // Get the relevant bitboard.
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };

    // Get the starting position of the king.
    let from = if board.white_to_play { board.white_king } else { board.black_king };

    // Get pseudo-legal moves.
    let pl_moves_bb = king_mask(from);

    // Forbid capture of friendly pieces.
    let moves_bb = pl_moves_bb & !friendly;

    // Add a bit for each target square.
    let mut t = moves_bb;
    while t != 0
    {
        // Get the target square index.
        let to = t.trailing_zeros() as usize;
        // Add the bit.
        m |= 1u64 << to;
        t &= t - 1;
    }

    return m;
}

// Create a vector containing moves that the king can make.
pub fn generate_king_moves(board: &Board) -> Vec<Move>
{
    let mut moves = vec![];

    // Get the relevant bitboards.
    let enemy = if board.white_to_play { board.black_pieces } else { board.white_pieces };
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };

    // Get the starting position of the king.
    let from = if board.white_to_play { board.white_king } else { board.black_king };

    // Get pseudo-legal moves.
    let pl_moves_bb = king_mask(from);

    // Forbid capture of friendly pieces.
    let mut moves_bb = pl_moves_bb & !friendly;

    // Forbid moving to an attacked square.
    moves_bb &= !board.attacked_squares;

    // Add a move for each target square.
    let mut t = moves_bb;
    while t != 0
    {
        // Get the target square index.
        let to = t.trailing_zeros() as usize;
        let to_mask = 1u64 << to;
        let ctx = if (enemy & to_mask) != 0
        {
            MoveContext::Capture(get_piece_type_on_square(board, to))
        }
        else
        {
            MoveContext::None
        };
        // Create a temporary copy of the board to test the validity of the move.
        let mut temp = board.clone();
        let mv = Move {
            start: from,
            end: to,
            context: ctx,
            previous_ep_target: board.en_passant_target,
        };
        temp.make_move(mv);
        // Add the move only if the king is not in check.
        if !is_king_attacked(&temp, false)
        {
            moves.push(mv);
        }
        t &= t - 1;
    }

    return moves;
}
