use std::io;

use barnarok::*;
use clap::{Parser, Subcommand};
use rand::seq::IndexedRandom;

#[derive(Parser)]
#[command(name = "barnarok")]
#[command(about = "A Rust chess engine", long_about = None)]
struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands
{
    Run,
    Explore
    {
        #[arg(short, long)]
        depth: usize,
        #[arg(short, long)]
        verbose: bool,
    },
    Play,
}

fn main()
{
    let cli = Cli::parse();

    match &cli.command
    {
        Commands::Run =>
        {
            match Board::from_fen("rnbqk3/1p1pPp1p/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq -")
            {
                Ok(board) =>
                {
                    board.display();
                    println!("possible moves: {}", board.get_legal_moves().len());
                    for (i, m) in board.get_legal_moves().iter().enumerate()
                    {
                        println!("{}: {:?}", i + 1, m);
                    }
                    // print_bb(get_attacked_squares(&board));

                    println!("attacked: {}", is_king_attacked(&board, true));
                },
                Err(err) => eprint!("{}", err),
            }
        },
        Commands::Explore { depth, verbose } =>
        {
            match Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq -")
            {
                Ok(mut board) =>
                {
                    if *verbose
                    {
                        println!(
                            "number of positions at a depth of {}: {}",
                            depth,
                            explore_verbose(&mut board, *depth, String::new())
                        )
                    }
                    else
                    {
                        println!(
                            "number of positions at a depth of {}: {}",
                            depth,
                            explore(&mut board, *depth)
                        )
                    }

                    // board.display();
                },
                Err(err) => eprint!("{}", err),
            }
        },
        Commands::Play =>
        {
            match Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -")
            {
                Ok(mut board) =>
                {
                    board.display();
                    loop
                    {
                        println!("=========");
                        if board.white_to_play
                        {
                            let moves = board.get_legal_moves();
                            for (i, m) in moves.iter().enumerate()
                            {
                                println!("{}: {:?}", i + 1, m);
                            }
                            let mut choice = 0;
                            while choice < 1 || choice > moves.len()
                            {
                                println!("Select a move:");
                                let mut input = String::new();
                                io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read line");
                                choice =
                                    input.trim().parse().expect("Input was not a valid integer");
                            }
                            board.make_move(moves[choice - 1]);
                        }
                        else
                        {
                            match board.get_legal_moves().choose(&mut rand::rng())
                            {
                                Some(mv) =>
                                {
                                    board.make_move(*mv);
                                    board.display();
                                    println!("Move made: {} -> {}", mv.start, mv.end);
                                },
                                None =>
                                {
                                    break;
                                },
                            }
                        }
                    }
                },
                Err(err) => eprintln!("{}", err),
            }
        },
    }
}
