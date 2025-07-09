use crate::{Board, defines::*};

// Get the piece type on a certain square.
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

pub fn launch_explore(board: &mut Board, max_depth: usize, verbose: bool) -> usize
{
    if verbose
    {
        return explore_verbose(board, max_depth, String::new());
    }
    else
    {
        return explore(board, max_depth, true);
    }
}

// Explore every possible position after a certain amount of plies.
// 1: 20; 2: 400; 3: 8902; etc.
fn explore(board: &mut Board, max_depth: usize, root: bool) -> usize
{
    if max_depth == 0
    {
        return 1;
    }

    let mut n = 0;
    let mut uci: String;
    let moves = board.get_legal_moves();
    for mv in moves.iter()
    {
        if root
        {
            uci = mv.to_uci();
        }
        else
        {
            uci = "".into();
        }
        board.make_move(*mv);
        let m = explore(board, max_depth - 1, false);
        if root
        {
            println!("{}: {}", uci, m)
        };
        n += m;
        board.unmake_move(*mv);
    }

    return n;
}

// Explore every possible position after a certain amount of plies, and print the tree of moves.
fn explore_verbose(board: &mut Board, max_depth: usize, prefix: String) -> usize
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
        let nb = explore(board, max_depth - 1, false);
        board.unmake_move(*mv);

        println!(
            "{}{}{} {}",
            prefix,
            branch,
            mv.to_uci(),
            if max_depth > 1 { format!("({})", nb) } else { "".into() }
        );

        board.make_move(*mv);
        explore_verbose(board, max_depth - 1, prefix.clone() + child_prefix);
        board.unmake_move(*mv);

        n += nb;
    }

    return n;
}
