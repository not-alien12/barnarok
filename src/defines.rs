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

// A bitboard is a 64 bit number, and each bit indicates the presence or absence
// of a given piece on a tile. For a "white pawn bitboard", a '0' at the n-th
// bit means that there is no white pawn at the n-th tile, and a '1' means that
// there is one.
pub type Bitboard = u64;