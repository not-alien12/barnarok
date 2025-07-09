use rand::{rng, seq::SliceRandom};

use crate::{Board, Move, is_king_attacked};

const INF: i32 = 1_000_000;

pub fn negamax(board: &mut Board, depth: u8) -> (i32, Option<Move>)
{
    if depth == 0
    {
        return (board.evaluate(), None);
    }
    let mut max = -INF;
    let mut best = None;

    let moves = board.get_legal_moves();
    if moves.is_empty()
    {
        return (-INF, None);
    }

    for mv in moves.iter()
    {
        board.make_move(*mv);
        let (mut score, _) = negamax(board, depth - 1);
        score = -score;
        board.unmake_move(*mv);
        if score > max
        {
            max = score;
            best = Some(*mv);
        }
    }
    return (max, best);
}

pub fn launch_alpha_beta_quiesce(board: &mut Board, depth: u8) -> (i32, Option<Move>)
{
    return alpha_beta_quiesce(board, -INF, INF, depth);
}

fn alpha_beta_quiesce(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    depth: u8,
) -> (i32, Option<Move>)
{
    if depth == 0
    {
        return (quiesce(board, alpha, beta), None);
    }
    let mut max = -INF;
    let mut best = None;

    let mut moves = board.get_legal_moves();
    let mut rng = rng();
    moves.shuffle(&mut rng);
    if moves.is_empty()
    {
        if is_king_attacked(board, false)
        {
            return (-INF, None);
        }
        return (0, None);
    }

    for mv in moves.iter()
    {
        board.make_move(*mv);
        let (mut score, _) = alpha_beta(board, -beta, -alpha, depth - 1);
        score = -score;
        board.unmake_move(*mv);
        if score > max
        {
            max = score;
            best = Some(*mv);
            if score > alpha
            {
                alpha = score;
            }
        }
        if score >= beta
        {
            return (max, best);
        }
    }
    return (max, best);
}

pub fn launch_alpha_beta(board: &mut Board, depth: u8) -> (i32, Option<Move>)
{
    return alpha_beta(board, -INF, INF, depth);
}

fn alpha_beta(board: &mut Board, mut alpha: i32, beta: i32, depth: u8) -> (i32, Option<Move>)
{
    if depth == 0
    {
        return (board.evaluate(), None);
    }
    let mut max = -INF;
    let mut best = None;

    let mut moves = board.get_legal_moves();
    let mut rng = rng();
    moves.shuffle(&mut rng);
    if moves.is_empty()
    {
        if is_king_attacked(board, false)
        {
            return (-INF, None);
        }
        return (0, None);
    }

    for mv in moves.iter()
    {
        board.make_move(*mv);
        let (mut score, _) = alpha_beta(board, -beta, -alpha, depth - 1);
        score = -score;
        board.unmake_move(*mv);
        if score > max
        {
            max = score;
            best = Some(*mv);
            if score > alpha
            {
                alpha = score;
            }
        }
        if score >= beta
        {
            return (max, best);
        }
    }
    return (max, best);
}

fn quiesce(board: &mut Board, mut alpha: i32, beta: i32) -> i32
{
    let mut best_value = board.evaluate();
    if best_value >= beta
    {
        return best_value;
    }
    if best_value > alpha
    {
        alpha = best_value;
    }

    for mv in board.get_legal_moves().iter()
    {
        if mv.capture != None
        {
            continue;
        }
        board.make_move(*mv);
        let score = -quiesce(board, -beta, -alpha);
        board.unmake_move(*mv);
        if score >= beta
        {
            return score;
        }
        if score > best_value
        {
            best_value = score;
        }
        if score > alpha
        {
            alpha = score;
        }
    }
    return best_value;
}
