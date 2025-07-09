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
    Explore
    {
        #[arg(short, long)]
        depth: usize,
        #[arg(short, long)]
        verbose: bool,
    },
    Play
    {
        #[arg(short, long)]
        wstrat: String,
        #[arg(short, long)]
        bstrat: String,
    },
}

fn main()
{
    let cli = Cli::parse();

    match &cli.command
    {
        Commands::Run =>
        {
            match Board::from_fen("rnb2bnr/ppqppppp/8/3kq2R/8/5K2/PPPPPPPP/RNBQ1BN1 w - -")
            {
                Ok(board) =>
                {
                    board.display();
                    println!("evaluation: {}", board.evaluate());
                },
                Err(err) => eprint!("{}", err),
            }
        },
        Commands::Explore { depth, verbose } =>
        {
            match Board::from_fen("8/8/8/3q4/8/4Q3/8/4K2k w - -")
            {
                Ok(mut board) =>
                {
                    board.display();
                    println!(
                        "number of positions at a depth of {}: {}",
                        depth,
                        launch_explore(&mut board, *depth, *verbose)
                    );
                },
                Err(err) => eprint!("{}", err),
            }
        },
        Commands::Play { wstrat, bstrat } => match play(*&wstrat.as_str(), *&bstrat.as_str())
        {
            Ok(result) => match result
            {
                GameResult::White => println!("White wins by checkmate."),
                GameResult::Black => println!("Black wins by checkmate."),
                GameResult::Stalemate => println!("The game ends in a draw by checkmate."),
            },
            Err(err) => eprintln!("{}", err),
        },
    }
}
