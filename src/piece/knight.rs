use crate::{
    Bitboard, Board, Move, MoveContext, get_piece_type_on_square, is_king_attacked, masks::*,
};

// Generate a bitboard representing squares attacked by the knights of the player that just played.
pub fn generate_knight_attacks(board: &Board) -> Bitboard
{
    let mut m = 0u64;

    // Get relevant bitboards.
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };
    let knights = if board.white_to_play { board.white_knights } else { board.black_knights };

    // Loop over friendly knights.
    let mut bits = knights;
    while bits != 0
    {
        // Get the square index of the next knight.
        let from = bits.trailing_zeros() as usize;
        bits &= bits - 1;

        // Get pseudo-legal moves.
        let pl_moves_bb = knight_mask(from);

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
    }

    return m;
}

// Create a vector containing moves that knights can make.
pub fn generate_knight_moves(board: &mut Board) -> Vec<Move>
{
    let mut moves = vec![];

    // Get relevant bitboards.
    let enemy = if board.white_to_play { board.black_pieces } else { board.white_pieces };
    let friendly = if board.white_to_play { board.white_pieces } else { board.black_pieces };
    let knights = if board.white_to_play { board.white_knights } else { board.black_knights };

    // Loop over friendly knights.
    let mut bits = knights;
    while bits != 0
    {
        // Get the square index of the next knight.
        let from = bits.trailing_zeros() as usize;
        bits &= bits - 1;

        // Get pseudo-legal moves.
        let pl_moves_bb = knight_mask(from);

        // Forbid capture of friendly pieces.
        let moves_bb = pl_moves_bb & !friendly;

        // Add a move for each target square.
        let mut t = moves_bb;
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
