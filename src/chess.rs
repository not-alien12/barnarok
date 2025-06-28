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

// A move consists of a start tile and an end tile.
// I might need to add more fields when I start using it.
#[derive(Debug)]
pub struct Move
{
    pub start: Index,
    pub end: Index,
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
        // Only generates pawn moves for now.
        return generate_pawn_moves(self);
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
}

// Generate legal moves for pawns.
fn generate_pawn_moves(board: &Board) -> Vec<Move>
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
