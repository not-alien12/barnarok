use crate::{
    Bitboard, Board, Move, MoveContext, get_piece_type_on_square, is_king_attacked,
    is_square_attacked, masks::*,
};

// Generate a bitboard representing squares attacked by the king of the player that just played.
pub fn generate_king_attacks(board: &Board) -> Bitboard
{
    // Empty bitboard.
    let mut m = 0u64;

    // Get the starting position of the king.
    let from = if board.white_to_play { board.white_king } else { board.black_king };

    // Get legal moves.
    let moves_bb = king_mask(from);

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
pub fn generate_king_moves(board: &mut Board) -> Vec<Move>
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

    // Masks representing the squares that must be free for a castle to be legal.
    const WHITE_QUEENSIDE_FREE_PATH_MASK: u64 = 0x00_00_00_00_00_00_00_0e;
    const WHITE_KINGSIDE_FREE_PATH_MASK: u64 = 0x00_00_00_00_00_00_00_60;
    const BLACK_QUEENSIDE_FREE_PATH_MASK: u64 = 0x0e_00_00_00_00_00_00_00;
    const BLACK_KINGSIDE_FREE_PATH_MASK: u64 = 0x60_00_00_00_00_00_00_00;

    // Add the castle moves.
    if board.white_to_play
    {
        // White queen side.
        if board.white_queen_side_castling_right
            && board.pieces & WHITE_QUEENSIDE_FREE_PATH_MASK == 0
            && !is_square_attacked(2, board, false)
            && !is_square_attacked(3, board, false)
            && !is_square_attacked(4, board, false)
        {
            let mv = Move {
                start: from,
                end: from - 2,
                context: MoveContext::QueenSideCastle,
                previous_ep_target: board.en_passant_target,
                previous_wqs: board.white_queen_side_castling_right,
                previous_wks: board.white_king_side_castling_right,
                previous_bqs: board.black_queen_side_castling_right,
                previous_bks: board.black_king_side_castling_right,
                capture: None,
            };

            moves.push(mv);
        }
        // White king side.
        if board.white_king_side_castling_right
            && board.pieces & WHITE_KINGSIDE_FREE_PATH_MASK == 0
            && !is_square_attacked(4, board, false)
            && !is_square_attacked(5, board, false)
            && !is_square_attacked(6, board, false)
        {
            let mv = Move {
                start: from,
                end: from + 2,
                context: MoveContext::KingSideCastle,
                previous_ep_target: board.en_passant_target,
                previous_wqs: board.white_queen_side_castling_right,
                previous_wks: board.white_king_side_castling_right,
                previous_bqs: board.black_queen_side_castling_right,
                previous_bks: board.black_king_side_castling_right,
                capture: None,
            };

            moves.push(mv);
        }
    }
    else
    {
        // Black queen side.
        if board.black_queen_side_castling_right
            && board.pieces & BLACK_QUEENSIDE_FREE_PATH_MASK == 0
            && !is_square_attacked(58, board, false)
            && !is_square_attacked(59, board, false)
            && !is_square_attacked(60, board, false)
        {
            let mv = Move {
                start: from,
                end: from - 2,
                context: MoveContext::QueenSideCastle,
                previous_ep_target: board.en_passant_target,
                previous_wqs: board.white_queen_side_castling_right,
                previous_wks: board.white_king_side_castling_right,
                previous_bqs: board.black_queen_side_castling_right,
                previous_bks: board.black_king_side_castling_right,
                capture: None,
            };

            moves.push(mv);
        }
        // Black king side.
        if board.black_king_side_castling_right
            && board.pieces & BLACK_KINGSIDE_FREE_PATH_MASK == 0
            && !is_square_attacked(60, board, false)
            && !is_square_attacked(61, board, false)
            && !is_square_attacked(62, board, false)
        {
            let mv = Move {
                start: from,
                end: from + 2,
                context: MoveContext::KingSideCastle,
                previous_ep_target: board.en_passant_target,
                previous_wqs: board.white_queen_side_castling_right,
                previous_wks: board.white_king_side_castling_right,
                previous_bqs: board.black_queen_side_castling_right,
                previous_bks: board.black_king_side_castling_right,
                capture: None,
            };

            moves.push(mv);
        }
    }

    return moves;
}
