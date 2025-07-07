use std::{collections::HashMap, io};

use rand::seq::IndexedRandom;

use super::*;

pub fn play(player_is_white: bool)
{
    match Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -")
    {
        Ok(mut board) =>
        {
            let mut player_playing = player_is_white;
            board.display();
            let mut n = 0;
            while n < 50
            {
                println!("=========");
                if player_playing
                {
                    match player_strategy(&mut board)
                    {
                        1 => break,
                        _ => (),
                    }
                }
                else
                {
                    match random_strategy(&mut board)
                    {
                        1 => break,
                        _ => (),
                    }
                }
                player_playing = !player_playing;
                n += 1;
            }
        },
        Err(err) => eprintln!("{}", err),
    }
}

fn player_strategy(board: &mut Board) -> u8
{
    let moves = board.get_legal_moves();
    if moves.len() == 0
    {
        return 1;
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
    board.make_move(**dict.get(&choice).unwrap());
    return 0;
}

fn random_strategy(board: &mut Board) -> u8
{
    match board.get_legal_moves().choose(&mut rand::rng())
    {
        Some(mv) =>
        {
            let move_name = mv.to_uci();
            board.make_move(*mv);
            board.display();
            println!("Move made: {}", move_name);
            return 0;
        },
        None =>
        {
            return 1;
        },
    }
}
