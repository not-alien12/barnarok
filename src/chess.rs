use super::moves::get_legal_moves;
use crate::get_attacked_squares;

pub type Piece = u8;

// Each piece is represented by a 5 bit number (stored in a u8).
// The last 3 bits represent the type,
// and the the first 2 bits represent the color.
pub const EMPTY: Piece = 0; // 0b00000
pub const PAWN: Piece = 1; // 0b00001
pub const ROOK: Piece = 2; // 0b00010
pub const KNIGHT: Piece = 3; // 0b00011
pub const BISHOP: Piece = 4; // 0b00100
pub const QUEEN: Piece = 5; // 0b00101
pub const KING: Piece = 6; // 0b00110
pub const WHITE: Piece = 8; // 0b01000
pub const BLACK: Piece = 16; // 0b10000

// Use bitwise AND to apply a filter to the piece and get its type.
pub const fn get_piece_type(piece: Piece) -> Piece
{
    return piece & 0b111;
}

// Use bitwise AND to apply a filter to the piece and get its color.
pub const fn get_piece_color(piece: Piece) -> Piece
{
    return piece & 0b11000;
}

// An Index represents a tile on the board.
pub type Index = usize;

// Enum to add context to a special move.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveContext
{
    None,
    EnPassant,
    QueenSideCastle,
    KingSideCastle,
    DoubleStep,
    Capture(Piece),
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
}

// A bitboard is a 64 bit number, and each bit indicates the presence or absence
// of a given piece on a tile. For a "white pawn bitboard", a '0' at the n-th
// bit means that there is no white pawn at the n-th tile, and a '1' means that
// there is one.
pub type Bitboard = u64;

// This struct represents the current state of the board.
// Bitboards and indices are used to give information on the positions of the
// pieces.
// Some data is redundant, but it should help calculating possible moves without
// looking for each piece manually.
#[derive(Clone, Copy)]
pub struct Board
{
    // White pieces positions by type.
    pub white_pawns: Bitboard,
    pub white_rooks: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Index,

    // Black pieces positions by type.
    pub black_pawns: Bitboard,
    pub black_rooks: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Index,

    // General pieces positions.
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub pieces: Bitboard,

    // Squares attacked by the side that played last.
    // This is used to detect checking.
    pub attacked_squares: Bitboard,

    // When a pawn moves 2 tiles, it can be taken using the 'en passant' rule.
    // There can only be one at a time, so we don't need a bitboard.
    // There can also be zero, so the index can be None.
    pub en_passant_target: Option<Index>,

    // These rights are granted when the game begins, but are lost indefinitely when the king or
    // the corresponding rook moves.
    // The other rules (no piece in the way, no check...) will be checked manually, and not stored.
    pub white_queen_side_castling_right: bool,
    pub white_king_side_castling_right: bool,
    pub black_queen_side_castling_right: bool,
    pub black_king_side_castling_right: bool,

    pub white_to_play: bool,
}

impl Board
{
    pub fn get_legal_moves(&self) -> Vec<Move>
    {
        return get_legal_moves(self);
    }

    // Apply a move a update the board data.
    pub fn make_move(&mut self, mv: Move)
    {
        let from = mv.start;
        let to = mv.end;
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        if self.white_to_play
        {
            // Update white pieces position by removing the 'from' bit and adding the 'to' bit.
            self.white_pieces = (self.white_pieces & !from_mask) | to_mask;

            // Update the bitboard corresponding to the piece that was moved.
            if self.white_pawns & from_mask != 0
            {
                self.white_pawns = (self.white_pawns & !from_mask) | to_mask;
            }
            else if self.white_rooks & from_mask != 0
            {
                // Rook moved from A1, so white loses its queen side castling right.
                if from == 0
                {
                    self.white_queen_side_castling_right = false;
                }
                // Rook moved from H1, so white loses its king side castling right.
                else if from == 7
                {
                    self.white_king_side_castling_right = false;
                }
                self.white_rooks = (self.white_rooks & !from_mask) | to_mask;
            }
            else if self.white_knights & from_mask != 0
            {
                self.white_knights = (self.white_knights & !from_mask) | to_mask;
            }
            else if self.white_bishops & from_mask != 0
            {
                self.white_bishops = (self.white_bishops & !from_mask) | to_mask;
            }
            else if self.white_queens & from_mask != 0
            {
                self.white_queens = (self.white_queens & !from_mask) | to_mask;
            }
            else if self.white_king == from
            {
                // The king moved, so white loses all its castling rights.
                self.white_king_side_castling_right = false;
                self.white_queen_side_castling_right = false;
                self.white_king = to;
            }
            else
            {
                panic!("make_move: no white piece on {}.", from);
            }

            // Update black bitboards to apply the potential capture.
            if let MoveContext::Capture(piece_type) = mv.context
            {
                self.black_pieces &= !to_mask;
                match piece_type
                {
                    PAWN => self.black_pawns &= !to_mask,
                    ROOK => self.black_rooks &= !to_mask,
                    KNIGHT => self.black_knights &= !to_mask,
                    BISHOP => self.black_bishops &= !to_mask,
                    QUEEN => self.black_queens &= !to_mask,
                    _ =>
                    {},
                }
            }
            // If the move is an en passant capture, the black pawn must be deleted.
            else if mv.context == MoveContext::EnPassant
            {
                let cap_sq = to - 8;
                let cap_mask = 1u64 << cap_sq;
                self.black_pieces &= !cap_mask;
                self.black_pawns &= !cap_mask;
            }

            // Set the en passant target.
            self.en_passant_target =
                if mv.context == MoveContext::DoubleStep { Some(to - 8) } else { None };
        }
        else
        {
            // Update black pieces position by removing the 'from' bit and adding the 'to' bit.
            self.black_pieces = (self.black_pieces & !from_mask) | to_mask;

            // Update the bitboard corresponding to the piece that was moved.
            if self.black_pawns & from_mask != 0
            {
                self.black_pawns = (self.black_pawns & !from_mask) | to_mask;
            }
            else if self.black_rooks & from_mask != 0
            {
                // Rook moved from A8, so black loses its queen side castling right.
                if from == 56
                {
                    self.black_queen_side_castling_right = false;
                }
                // Rook moved from H8, so black loses its king side castling right.
                else if from == 63
                {
                    self.black_king_side_castling_right = false;
                }
                self.black_rooks = (self.black_rooks & !from_mask) | to_mask;
            }
            else if self.black_knights & from_mask != 0
            {
                self.black_knights = (self.black_knights & !from_mask) | to_mask;
            }
            else if self.black_bishops & from_mask != 0
            {
                self.black_bishops = (self.black_bishops & !from_mask) | to_mask;
            }
            else if self.black_queens & from_mask != 0
            {
                self.black_queens = (self.black_queens & !from_mask) | to_mask;
            }
            else if self.black_king == from
            {
                // The king moved, so white loses all its castling rights.
                self.black_king_side_castling_right = false;
                self.black_queen_side_castling_right = false;
                self.black_king = to;
            }
            else
            {
                panic!("make_move: no black piece on {}.", from);
            }
            // Update white bitboards to apply the potential capture.
            if let MoveContext::Capture(piece_type) = mv.context
            {
                self.white_pieces &= !to_mask;
                match piece_type
                {
                    PAWN => self.white_pawns &= !to_mask,
                    ROOK => self.white_rooks &= !to_mask,
                    KNIGHT => self.white_knights &= !to_mask,
                    BISHOP => self.white_bishops &= !to_mask,
                    QUEEN => self.white_queens &= !to_mask,
                    _ =>
                    {},
                }
            }
            // If the move is an en passant capture, the white pawn must be deleted.
            if mv.context == MoveContext::EnPassant
            {
                let cap_sq = to + 8;
                let cap_mask = 1u64 << cap_sq;
                self.white_pieces &= !cap_mask;
                self.white_pawns &= !cap_mask;
            }

            // Set the en passant target.
            self.en_passant_target =
                if mv.context == MoveContext::DoubleStep { Some(to + 8) } else { None };
        }

        // Update the global piece bitboard using the sided bitboards.
        self.pieces = self.white_pieces | self.black_pieces;

        // Update the attacked squares bitboard to detect checks and forbidden moves.
        self.attacked_squares = get_attacked_squares(self);

        self.white_to_play = !self.white_to_play;
    }

    // Go back to the previous state of the board, before the move was applied.
    pub fn unmake_move(&mut self, mv: Move)
    {
        let from = mv.start;
        let to = mv.end;
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;

        // Flip the playing side.
        self.white_to_play = !self.white_to_play;

        // Store the piece type on the destination square.
        let moved_piece_type;

        // Undo the move.
        if self.white_to_play
        {
            // Remove the white piece in 'to' square, add it back to 'from' square.
            if self.white_pawns & to_mask != 0
            {
                moved_piece_type = PAWN;
                self.white_pawns &= !to_mask;
            }
            else if self.white_rooks & to_mask != 0
            {
                moved_piece_type = ROOK;
                self.white_rooks &= !to_mask;
            }
            else if self.white_knights & to_mask != 0
            {
                moved_piece_type = KNIGHT;
                self.white_knights &= !to_mask;
            }
            else if self.white_bishops & to_mask != 0
            {
                moved_piece_type = BISHOP;
                self.white_bishops &= !to_mask;
            }
            else if self.white_queens & to_mask != 0
            {
                moved_piece_type = QUEEN;
                self.white_queens &= !to_mask;
            }
            else if self.white_king == to
            {
                moved_piece_type = KING;
                self.white_king = from;
            }
            else
            {
                panic!("unmake_move: no white piece on to={}", to);
            }
            self.white_pieces &= !to_mask;

            // Restore any captured black piece.
            match mv.context
            {
                MoveContext::Capture(piece) =>
                {
                    // Normal capture: put it back on 'to'.
                    let bm = to_mask;
                    self.black_pieces |= bm;
                    match piece
                    {
                        PAWN => self.black_pawns |= bm,
                        ROOK => self.black_rooks |= bm,
                        KNIGHT => self.black_knights |= bm,
                        BISHOP => self.black_bishops |= bm,
                        QUEEN => self.black_queens |= bm,
                        KING => self.black_king = to,
                        _ => unreachable!(),
                    }
                },
                MoveContext::EnPassant =>
                {
                    // En passant: captured pawn was behind 'to'.
                    let cap_sq = to - 8;
                    let cap_mask = 1u64 << cap_sq;
                    self.black_pawns |= cap_mask;
                    self.black_pieces |= cap_mask;
                },
                _ =>
                {},
            }

            // Restore the white piece back to 'from'.
            match moved_piece_type
            {
                PAWN => self.white_pawns |= from_mask,
                ROOK => self.white_rooks |= from_mask,
                KNIGHT => self.white_knights |= from_mask,
                BISHOP => self.white_bishops |= from_mask,
                QUEEN => self.white_queens |= from_mask,
                KING => self.white_king = from,
                _ => unreachable!(),
            }
            self.white_pieces |= from_mask;
        }
        // Black made the move:
        else
        {
            // Remove the black piece in 'to' square, add it back to 'from' square.
            if self.black_pawns & to_mask != 0
            {
                moved_piece_type = PAWN;
                self.black_pawns &= !to_mask;
            }
            else if self.black_rooks & to_mask != 0
            {
                moved_piece_type = ROOK;
                self.black_rooks &= !to_mask;
            }
            else if self.black_knights & to_mask != 0
            {
                moved_piece_type = KNIGHT;
                self.black_knights &= !to_mask;
            }
            else if self.black_bishops & to_mask != 0
            {
                moved_piece_type = BISHOP;
                self.black_bishops &= !to_mask;
            }
            else if self.black_queens & to_mask != 0
            {
                moved_piece_type = QUEEN;
                self.black_queens &= !to_mask;
            }
            else if self.black_king == to
            {
                moved_piece_type = KING;
                self.black_king = from;
            }
            else
            {
                panic!("unmake_move: no black piece on to={}", to);
            }
            self.black_pieces &= !to_mask;

            // Restore any captured white piece.
            match mv.context
            {
                MoveContext::Capture(piece) =>
                {
                    // Normal capture: put it back on 'to'.
                    let wm = to_mask;
                    self.white_pieces |= wm;
                    match piece
                    {
                        PAWN => self.white_pawns |= wm,
                        ROOK => self.white_rooks |= wm,
                        KNIGHT => self.white_knights |= wm,
                        BISHOP => self.white_bishops |= wm,
                        QUEEN => self.white_queens |= wm,
                        KING => self.white_king = to,
                        _ => unreachable!(),
                    }
                },
                MoveContext::EnPassant =>
                {
                    // En passant: captured pawn was behind 'to'.
                    let cap_sq = to + 8;
                    let cap_mask = 1u64 << cap_sq;
                    self.white_pawns |= cap_mask;
                    self.white_pieces |= cap_mask;
                },
                _ =>
                {},
            }

            // Restore the black piece back to 'from'.
            match moved_piece_type
            {
                PAWN => self.black_pawns |= from_mask,
                ROOK => self.black_rooks |= from_mask,
                KNIGHT => self.black_knights |= from_mask,
                BISHOP => self.black_bishops |= from_mask,
                QUEEN => self.black_queens |= from_mask,
                KING => self.black_king = from,
                _ => unreachable!(),
            }
            self.black_pieces |= from_mask;
        }

        // Restore previous en_passant_target.
        self.en_passant_target = mv.previous_ep_target;

        // Rebuild global occupancy and attacked squares.
        self.pieces = self.white_pieces | self.black_pieces;
        self.attacked_squares = get_attacked_squares(self);
    }

    // Create a new Board from a FEN string.
    // It reads the first 4 fields:
    // - Piece placement
    // - Side to move
    // - Castling rights
    // - En passant target square
    // Later, I will have to add the move counters to manage draw rules.
    pub fn from_fen(fen: &str) -> Result<Self, String>
    {
        // Read the 4 fields.
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 4
        {
            // Return an error if too few or too many fields were provided.
            return Err("FEN strings must have exactly 4 fields.".into());
        }

        // Store the fields in explicit variables.
        let placement = parts[0];
        let active_color = parts[1];
        let castling = parts[2];
        let en_passant = parts[3];

        // Start with empty bitboards.
        let mut wp = 0u64;
        let mut wr = 0u64;
        let mut wn = 0u64;
        let mut wb = 0u64;
        let mut wq = 0u64;
        let mut wk_sq: Option<usize> = None;

        let mut bp = 0u64;
        let mut br = 0u64;
        let mut bn = 0u64;
        let mut bb = 0u64;
        let mut bq = 0u64;
        let mut bk_sq: Option<usize> = None;

        // Parse ranks from 8 to 1.
        let ranks: Vec<&str> = placement.split('/').collect();
        if ranks.len() != 8
        {
            // Return an error if there are not exactly 8 ranks specified.
            return Err("Expected 8 ranks in placement.".into());
        }

        // Loop over the ranks.
        for (r_idx, &rank_str) in ranks.iter().enumerate()
        {
            // Get the actual rank number.
            let rank = 7 - r_idx;

            // Loop over the files.
            let mut file = 0;
            for c in rank_str.chars()
            {
                // If the character is a digit, the given amount of files is skipped.
                if c.is_digit(10)
                {
                    file += c.to_digit(10).unwrap() as usize;
                }
                else
                {
                    // Return an error if more than 8 squares were specified on this rank.
                    if file >= 8
                    {
                        return Err(format!("Rank {} has too many squares.", 8 - r_idx));
                    }
                    // Convert the rank and file to a square index.
                    let sq = rank * 8 + file;
                    // Increment the file for next iteration.
                    file += 1;
                    // Get the bitboard corresponding to the piece to place.
                    let bb_target = match c
                    {
                        'P' => &mut wp,
                        'p' => &mut bp,
                        'R' => &mut wr,
                        'r' => &mut br,
                        'N' => &mut wn,
                        'n' => &mut bn,
                        'B' => &mut wb,
                        'b' => &mut bb,
                        'Q' => &mut wq,
                        'q' => &mut bq,
                        // If the piece is a king, we just need to set the king square index,
                        // instead of writing into a bitboard.
                        'K' =>
                        {
                            wk_sq = Some(sq);
                            continue;
                        },
                        'k' =>
                        {
                            bk_sq = Some(sq);
                            continue;
                        },
                        // Return an error if the character is not recognized.
                        _ => return Err(format!("Invalid piece char '{}'.", c)),
                    };
                    // Set the right bit of the right board to 1.
                    *bb_target |= 1u64 << sq;
                }
            }
            // Return an error if there are too few or too many squares on this rank.
            if file != 8
            {
                return Err(format!(
                    "Rank {} has {} squares, but 8 were expected.",
                    8 - r_idx,
                    file
                ));
            }
        }

        // Return an error if a king is missing.
        let white_king = wk_sq.ok_or("Missing white king.")?;
        let black_king = bk_sq.ok_or("Missing black king.")?;

        // Set castling rights.
        let mut wks = false;
        let mut wqs = false;
        let mut bks = false;
        let mut bqs = false;
        if castling != "-"
        {
            for ch in castling.chars()
            {
                match ch
                {
                    'K' => wks = true,
                    'Q' => wqs = true,
                    'k' => bks = true,
                    'q' => bqs = true,
                    _ => return Err(format!("Invalid castling char '{}'.", ch)),
                }
            }
        }

        // Set en passant target.
        let en_passant_target = if en_passant == "-"
        {
            None
        }
        else
        {
            // Convert each character of the square name into file and rank indices.
            let file = (en_passant.as_bytes()[0] - b'a') as usize;
            let rank = (en_passant.as_bytes()[1] - b'1') as usize;
            // Return an error if an index is invalid.
            if file > 7 || rank > 7
            {
                return Err(format!("Invalid en passant square '{}'.", en_passant));
            }
            // Set the square index using the file and rank indices.
            Some(rank * 8 + file)
        };

        // Aggregate piece bitboards to create broader bitboards.
        let white_pieces = wp | wr | wn | wb | wq | (1u64 << white_king);
        let black_pieces = bp | br | bn | bb | bq | (1u64 << black_king);
        let all_pieces = white_pieces | black_pieces;

        // Create the Board object using the data we gathered from the FEN string.
        return Ok(Board {
            white_pawns: wp,
            white_rooks: wr,
            white_knights: wn,
            white_bishops: wb,
            white_queens: wq,
            white_king,

            black_pawns: bp,
            black_rooks: br,
            black_knights: bn,
            black_bishops: bb,
            black_queens: bq,
            black_king,

            white_pieces,
            black_pieces,
            pieces: all_pieces,

            attacked_squares: 0u64,

            en_passant_target,

            white_queen_side_castling_right: wqs,
            white_king_side_castling_right: wks,
            black_queen_side_castling_right: bqs,
            black_king_side_castling_right: bks,

            // Use the last field of the FEN string to determine the side to move.
            white_to_play: match active_color
            {
                "w" => true,
                "b" => false,
                // Return an error if the character is invalid.
                _ => return Err(format!("Invalid active color `{}`.", active_color)),
            },
        });
    }

    pub fn display(&self)
    {
        for rank in (0 .. 8).rev()
        {
            for file in 0 .. 8
            {
                let sq = rank * 8 + file;
                let ch = if (self.white_pawns >> sq) & 1 != 0
                {
                    'P'
                }
                else if (self.white_rooks >> sq) & 1 != 0
                {
                    'R'
                }
                else if (self.white_knights >> sq) & 1 != 0
                {
                    'N'
                }
                else if (self.white_bishops >> sq) & 1 != 0
                {
                    'B'
                }
                else if (self.white_queens >> sq) & 1 != 0
                {
                    'Q'
                }
                else if self.white_king == sq
                {
                    'K'
                }
                else if (self.black_pawns >> sq) & 1 != 0
                {
                    'p'
                }
                else if (self.black_rooks >> sq) & 1 != 0
                {
                    'r'
                }
                else if (self.black_knights >> sq) & 1 != 0
                {
                    'n'
                }
                else if (self.black_bishops >> sq) & 1 != 0
                {
                    'b'
                }
                else if (self.black_queens >> sq) & 1 != 0
                {
                    'q'
                }
                else if self.black_king == sq
                {
                    'k'
                }
                else
                {
                    '·'
                };
                print!("{} ", ch);
            }
            println!();
        }
    }

    // Convert a move into Standard Algebraic Notation (SAN).
    pub fn to_san(&self, mv: Move) -> String
    {
        // Helper: index -> square name.
        fn idx_to_coord(idx: usize) -> String
        {
            let file = (b'a' + (idx % 8) as u8) as char;
            let rank = (1 + idx / 8).to_string();
            format!("{}{}", file, rank)
        }

        // Detect piece type moved.
        let piece = self.piece_at(mv.start);
        // Map to letter.
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

        // Destination coord.
        let to_coord = idx_to_coord(mv.end);
        // Origin coord (for pawn captures).
        let from_coord = idx_to_coord(mv.start);

        let mut san = String::new();
        match mv.context
        {
            MoveContext::KingSideCastle => return "O-O".into(),
            MoveContext::QueenSideCastle => return "O-O-O".into(),
            _ =>
            {},
        }

        // Is it a capture?
        let is_capture = matches!(mv.context, MoveContext::Capture(_) | MoveContext::EnPassant);

        // Build the SAN string.
        if piece == PAWN && is_capture
        {
            // When a pawn captures a piece, its file is always indicated.
            san.push(from_coord.chars().next().unwrap_or_default());
            // Add the capture indicator.
            san.push('x');
        }
        else
        {
            // TODO: disambiguating moves if pieces of the same type can move to the same square.

            // Add the piece type indicator.
            san.push_str(piece_char);
            // Add the capture indicator.
            if is_capture
            {
                san.push('x');
            }
        }

        // Add the destination square.
        san.push_str(&to_coord);

        // TODO: check (+), checkmate (#).
        return san;
    }

    fn piece_at(&self, sq: usize) -> Piece
    {
        return get_piece_type_on_square(self, sq);
    }
}

pub fn get_piece_type_on_square(board: &Board, sq: usize) -> Piece
{
    let sq_bb = 1u64 << sq;
    if board.white_pieces & sq_bb != 0
    {
        return if board.white_pawns & sq_bb != 0
        {
            PAWN
        }
        else if board.white_rooks & sq_bb != 0
        {
            ROOK
        }
        else if board.white_knights & sq_bb != 0
        {
            KNIGHT
        }
        else if board.white_bishops & sq_bb != 0
        {
            BISHOP
        }
        else if board.white_queens & sq_bb != 0
        {
            QUEEN
        }
        else
        {
            KING
        };
    }
    else if board.black_pieces & sq_bb != 0
    {
        return if board.black_pawns & sq_bb != 0
        {
            PAWN
        }
        else if board.black_rooks & sq_bb != 0
        {
            ROOK
        }
        else if board.black_knights & sq_bb != 0
        {
            KNIGHT
        }
        else if board.black_bishops & sq_bb != 0
        {
            BISHOP
        }
        else if board.black_queens & sq_bb != 0
        {
            QUEEN
        }
        else
        {
            KING
        };
    }
    else
    {
        return EMPTY;
    }
}

// Print a bitboard as an 8x8 board (white perspective).
pub fn print_bb(bb: u64)
{
    for rank in (0 .. 8).rev()
    {
        for file in 0 .. 8
        {
            let sq = rank * 8 + file;
            print!("{} ", if (bb >> sq) & 1 == 1 { '1' } else { '·' });
        }
        println!();
    }
    println!();
}

// Explore every possible position after a certain amount of plies.
// 1: 20; 2: 400; 3: 8902; etc.
pub fn explore(board: &mut Board, max_depth: usize) -> usize
{
    if max_depth == 0
    {
        return 1;
    }

    let mut n = 0;
    let moves = board.get_legal_moves();
    for mv in moves.iter()
    {
        board.make_move(*mv);
        n += explore(board, max_depth - 1);
        board.unmake_move(*mv);
    }

    return n;
}

// Explore every possible position after a certain amount of plies, and print the tree of moves.
pub fn explore_verbose(board: &mut Board, max_depth: usize, prefix: String) -> usize
{
    if max_depth == 0
    {
        return 1;
    }

    let mut n = 0;
    let moves = board.get_legal_moves();
    let count = moves.len();

    for (i, mv) in moves.iter().enumerate()
    {
        let is_last = i == count - 1;

        let branch = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        board.make_move(*mv);
        let nb = explore(board, max_depth - 1);
        board.unmake_move(*mv);

        println!(
            "{}{}{} {}",
            prefix,
            branch,
            board.to_san(*mv),
            if max_depth > 1 { format!("({})", nb) } else { "".into() }
        );

        board.make_move(*mv);
        explore_verbose(board, max_depth - 1, prefix.clone() + child_prefix);
        board.unmake_move(*mv);

        n += nb;
    }

    return n;
}
