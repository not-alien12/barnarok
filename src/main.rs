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
            let piece: Piece = ROOK | WHITE;
            println!("type: {}", get_piece_type(piece));
            println!("color: {}", get_piece_color(piece));
        },
    }
}
