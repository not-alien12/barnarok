use barnarok::*;
use clap::{Parser, Subcommand};

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
}

fn main()
{
    let cli = Cli::parse();

    match &cli.command
    {
        Commands::Run =>
        {
            match Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq -")
            {
                Ok(board) =>
                {
                    board.display();
                    println!("possible moves: {}", board.get_legal_moves().len());
                    for (i, m) in board.get_legal_moves().iter().enumerate()
                    {
                        println!("{}: {:?}", i + 1, m);
                    }
                },
                Err(err) => eprint!("{}", err),
            }
        },
    }
}
