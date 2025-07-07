use crate::{
    Bitboard, Board, Move, MoveContext, get_piece_type_on_square, is_king_attacked, masks::*,
    piece::slider::*,
};

// Generate a bitboard representing squares attacked by the rooks of the player that just played.
pub fn generate_rook_attacks(board: &Board) -> Bitboard
{
    // Empty bitboard.
    let mut m = 0u64;

    // Get relevant masks.
    let occ = board.pieces;
    let rooks = if board.white_to_play { board.white_rooks } else { board.black_rooks };

    // Loop over the rooks.
    let mut bits = rooks;
    while bits != 0
    {
        // Get the square index of the next rook.
        let from = bits.trailing_zeros() as usize;
        bits &= bits - 1;

        // Get all legals moves the rook can make.
        let attacks = rook_attacks_hq(from, occ);

        // Fill the bitboard with the attacked squares.
        let mut t = attacks;
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

// Create a vector containing moves that rooks on the board can make.
pub fn generate_rook_moves_hq(board: &mut Board) -> Vec<Move>
{
    let mut moves = Vec::new();

    // Get relevant masks.
    let occ = board.pieces;
    let enemy = if board.white_to_play { board.black_pieces } else { board.white_pieces };
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

// Create a bitboard representing the squares a rook can get to.
// It allows the rook to capture friendly pieces, so another mask is needed after that.
pub fn rook_attacks_hq(sq: usize, occ: u64) -> u64
{
    let rm = rank_mask(sq);
    let fm = file_mask(sq);
    return slider_attacks_hq(sq, occ, rm) | slider_attacks_hq(sq, occ, fm);
}
