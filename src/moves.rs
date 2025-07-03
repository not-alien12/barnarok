use super::{
    chess::{Board, Move},
    piece::*,
};
use crate::{Bitboard, black_king_pawn_mask, knight_mask, white_king_pawn_mask};

pub fn get_legal_moves(board: &Board) -> Vec<Move>
{
    let mut all_moves = vec![];

    let mut pawn_moves = generate_pawn_moves(board);
    let mut rook_moves = generate_rook_moves_hq(board);
    let mut bishop_moves = generate_bishop_moves_hq(board);
    let mut queen_moves = generate_queen_moves_hq(board);
    let mut knight_moves = generate_knight_moves(board);
    let mut king_moves = generate_king_moves(board);

    all_moves.append(&mut pawn_moves);
    all_moves.append(&mut rook_moves);
    all_moves.append(&mut bishop_moves);
    all_moves.append(&mut queen_moves);
    all_moves.append(&mut knight_moves);
    all_moves.append(&mut king_moves);

    return all_moves;
}

// Return true if the playing king is attacked by an enemy piece.
pub fn is_king_attacked(board: &Board, playing_side: bool) -> bool
{
    let white_to_play = if playing_side { board.white_to_play } else { !board.white_to_play };

    let sq = if white_to_play { board.white_king } else { board.black_king };

    let enemy_pawns = if white_to_play { board.black_pawns } else { board.white_pawns };
    if enemy_pawns
        & (if white_to_play { white_king_pawn_mask(sq) } else { black_king_pawn_mask(sq) })
        != 0
    {
        return true;
    }

    let enemy_knights = if white_to_play { board.black_knights } else { board.white_knights };
    if enemy_knights & knight_mask(sq) != 0
    {
        return true;
    }

    let enemy_straight_sliders = if white_to_play
    {
        board.black_rooks | board.black_queens
    }
    else
    {
        board.white_rooks | board.white_queens
    };
    if enemy_straight_sliders & rook_attacks_hq(sq, board.pieces) != 0
    {
        return true;
    }

    let enemy_diagonal_sliders = if white_to_play
    {
        board.black_bishops | board.black_queens
    }
    else
    {
        board.white_bishops | board.white_queens
    };
    if enemy_diagonal_sliders & bishop_attacks_hq(sq, board.pieces) != 0
    {
        return true;
    }

    return false;
}

pub fn get_attacked_squares(board: &Board) -> Bitboard
{
    let mut m = 0u64;

    m |= generate_pawn_attacks(board);
    m |= generate_rook_attacks(board);
    m |= generate_knight_attacks(board);
    m |= generate_bishops_attacks(board);
    m |= generate_queen_attacks(board);
    m |= generate_king_attacks(board);

    return m;
}
