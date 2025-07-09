use std::{collections::HashMap, io};

use rand::seq::IndexedRandom;

use super::*;

pub enum GameResult
{
    White,
    Black,
    Stalemate,
}

pub fn play(white_strategy_choice: &str, black_strategy_choice: &str)
-> Result<GameResult, String>
{
    let white_strategy = match white_strategy_choice
    {
        "player" => player_strategy,
        "random" => random_strategy,
        "negamax" => negamax_strategy,
        "alphabeta" => alpha_beta_strategy,
        "alphabetaq" => alpha_beta_quiesce_strategy,
        _ => return Err("The chosen white strategy is not valid.".into()),
    };
    let black_strategy = match black_strategy_choice
    {
        "player" => player_strategy,
        "random" => random_strategy,
        "negamax" => negamax_strategy,
        "alphabeta" => alpha_beta_strategy,
        "alphabetaq" => alpha_beta_quiesce_strategy,
        _ => return Err("The chosen black strategy is not valid.".into()),
    };
    match Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -")
    {
        Ok(mut board) =>
        {
            let mut game_result = GameResult::Stalemate;
            board.display();
            let mut n = 0;
            while n < 500
            {
                if board.white_to_play
                {
                    println!("============================");
                    match white_strategy(&mut board)
                    {
                        Some(mv) =>
                        {
                            let mv_name = mv.to_uci();
                            board.make_move(mv);
                            board.display();
                            println!("White played: {}", mv_name);
                        },
                        None =>
                        {
                            if is_king_attacked(&board, false)
                            {
                                game_result = GameResult::Black;
                            }
                            break;
                        },
                    }
                }
                else
                {
                    println!("============================");
                    match black_strategy(&mut board)
                    {
                        Some(mv) =>
                        {
                            let mv_name = mv.to_uci();
                            board.make_move(mv);
                            board.display();
                            println!("Black played: {}", mv_name);
                            n += 1;
                        },
                        None =>
                        {
                            if is_king_attacked(&board, false)
                            {
                                game_result = GameResult::White;
                            }
                            break;
                        },
                    }
                }
            }
            println!("The game ends after {} full moves.", n);
            return Ok(game_result);
        },
        Err(err) => return Err(err),
    }
}

fn player_strategy(board: &mut Board) -> Option<Move>
{
    let moves = board.get_legal_moves();
    if moves.len() == 0
    {
        return None;
    }
    let mut dict = HashMap::new();
    for m in moves.iter()
    {
        dict.insert(m.to_uci(), m);
    }
    let mut choice = String::new();
    while !dict.contains_key(choice.as_str())
    {
        println!("Write a valid move name:");
        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        choice = choice.trim().to_string();
    }
    return Some(**dict.get(&choice).unwrap());
}

fn random_strategy(board: &mut Board) -> Option<Move>
{
    return board.get_legal_moves().choose(&mut rand::rng()).cloned();
}

fn negamax_strategy(board: &mut Board) -> Option<Move>
{
    let (_, result) = negamax(board, 4);
    return result;
}

fn alpha_beta_strategy(board: &mut Board) -> Option<Move>
{
    let (_, result) = launch_alpha_beta(board, 4);
    return result;
}

fn alpha_beta_quiesce_strategy(board: &mut Board) -> Option<Move>
{
    let (_, result) = launch_alpha_beta_quiesce(board, 4);
    return result;
}
