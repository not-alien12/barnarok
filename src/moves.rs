use super::{board::*, defines::*, piece::*};
use crate::{Bitboard, black_king_pawn_mask, king_mask, knight_mask, white_king_pawn_mask};

// Enum to add context to a special move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveContext
{
    None,
    EnPassant,
    QueenSideCastle,
    KingSideCastle,
    DoubleStep,
    Promotion(Piece),
}

// A move consists of a start tile and an end tile.
// I might need to add more fields when I start using it.
#[derive(Debug, Clone, Copy)]
pub struct Move
{
    pub start: Index,
    pub end: Index,
    pub context: MoveContext,
    pub previous_ep_target: Option<Index>,
    pub previous_wqs: bool,
    pub previous_wks: bool,
    pub previous_bqs: bool,
    pub previous_bks: bool,
    pub capture: Option<Piece>,
}

impl Move
{
    // Return a string representing a move using the Standard Algebraic Notation.
    // Examples: e4, Nf3, Bef4, R1xb1, axb7+, etc.
    pub fn to_san(&self, board: &mut Board) -> String
    {
        let piece = board.piece_at(self.start);
        let piece_char = match piece
        {
            PAWN => "",
            KNIGHT => "N",
            BISHOP => "B",
            ROOK => "R",
            QUEEN => "Q",
            KING => "K",
            _ => "",
        };

        // Special cases for castling.
        if self.context == MoveContext::KingSideCastle
        {
            return "O-O".into();
        }
        if self.context == MoveContext::QueenSideCastle
        {
            return "O-O-O".into();
        }

        let to_coord = Self::idx_to_coord(self.end);
        let from_coord = Self::idx_to_coord(self.start);
        let is_capture = self.context == MoveContext::EnPassant || self.capture != None;

        let mut disambiguation = String::new();

        // Disambiguation logic for non-pawns.
        if piece != PAWN && piece != KING
        {
            let all_moves = board.get_legal_moves();
            let mut same_dest_count = 0;
            let mut need_file = false;
            let mut need_rank = false;

            for other_mv in all_moves.iter()
            {
                if other_mv.end == self.end && other_mv.start != self.start
                {
                    let other_piece = board.piece_at(other_mv.start);
                    if other_piece == piece
                    {
                        same_dest_count += 1;
                        if (other_mv.start % 8) != (self.start % 8)
                        {
                            need_file = true;
                        }
                        if (other_mv.start / 8) != (self.start / 8)
                        {
                            need_rank = true;
                        }
                    }
                }
            }

            if same_dest_count > 0
            {
                if need_file && need_rank
                {
                    disambiguation = from_coord.clone();
                }
                else if need_file
                {
                    disambiguation.push(from_coord.chars().next().unwrap());
                }
                else
                {
                    disambiguation.push(from_coord.chars().nth(1).unwrap());
                }
            }
        }

        let mut san = String::new();

        // Pawn moves.
        if piece == PAWN
        {
            if is_capture
            {
                san.push(from_coord.chars().next().unwrap());
                san.push('x');
            }
            san.push_str(&to_coord);
        }
        else
        {
            san.push_str(piece_char);
            san.push_str(&disambiguation);
            if is_capture
            {
                san.push('x');
            }
            san.push_str(&to_coord);
        }

        if let MoveContext::Promotion(promoted) = self.context
        {
            match promoted
            {
                BISHOP => san.push('b'),
                ROOK => san.push('r'),
                KNIGHT => san.push('n'),
                QUEEN => san.push('q'),
                _ => (),
            }
        }

        board.make_move(*self);
        if is_king_attacked(&board, false)
        {
            san.push('+');
        }
        board.unmake_move(*self);

        return san;
    }

    // Return a string representing a move using the Universal Chess Interface notation.
    // Examples: e2e4, g1f3, d2f4, f7f8r, etc.
    pub fn to_uci(&self) -> String
    {
        let mut uci = String::new();
        uci.push_str(&Self::idx_to_coord(self.start));
        uci.push_str(&Self::idx_to_coord(self.end));
        if let MoveContext::Promotion(promoted) = self.context
        {
            match promoted
            {
                BISHOP => uci.push('b'),
                ROOK => uci.push('r'),
                KNIGHT => uci.push('n'),
                QUEEN => uci.push('q'),
                _ => (),
            }
        }
        return uci;
    }

    fn idx_to_coord(idx: usize) -> String
    {
        let file = (b'a' + (idx % 8) as u8) as char;
        let rank = (1 + idx / 8).to_string();
        format!("{}{}", file, rank)
    }
}

// Get legal moves for the playing side.
pub fn get_legal_moves(board: &mut Board) -> Vec<Move>
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

// Return true if the square is attacked by the specified side.
pub fn is_square_attacked(sq: usize, board: &Board, by_playing_side: bool) -> bool
{
    let attacked_by_white =
        if by_playing_side { board.white_to_play } else { !board.white_to_play };

    let enemy_pawns = if attacked_by_white { board.white_pawns } else { board.black_pawns };
    if enemy_pawns
        & (if attacked_by_white { black_king_pawn_mask(sq) } else { white_king_pawn_mask(sq) })
        != 0
    {
        return true;
    }

    let enemy_knights = if attacked_by_white { board.white_knights } else { board.black_knights };
    if enemy_knights & knight_mask(sq) != 0
    {
        return true;
    }

    let enemy_straight_sliders = if attacked_by_white
    {
        board.white_rooks | board.white_queens
    }
    else
    {
        board.black_rooks | board.black_queens
    };
    if enemy_straight_sliders & rook_attacks_hq(sq, board.pieces) != 0
    {
        return true;
    }

    let enemy_diagonal_sliders = if attacked_by_white
    {
        board.white_bishops | board.white_queens
    }
    else
    {
        board.black_bishops | board.black_queens
    };
    if enemy_diagonal_sliders & bishop_attacks_hq(sq, board.pieces) != 0
    {
        return true;
    }

    let enemy_king =
        if attacked_by_white { 1u64 << board.white_king } else { 1u64 << board.black_king };
    if enemy_king & king_mask(sq) != 0
    {
        return true;
    }

    return false;
}

// Return true if the playing king is attacked by an enemy piece.
pub fn is_king_attacked(board: &Board, by_playing_side: bool) -> bool
{
    let attacked_by_white =
        if by_playing_side { board.white_to_play } else { !board.white_to_play };

    let sq = if attacked_by_white { board.black_king } else { board.white_king };

    return is_square_attacked(sq, board, by_playing_side);
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
