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
        side: String,
    },
}

fn main()
{
    let cli = Cli::parse();

    match &cli.command
    {
        Commands::Run =>
        {
            match Board::from_fen("rnb2bnr/pppppppp/8/3kq2R/8/5K2/PPPPPPPP/RNBQ1BN1 w - -")
            {
                Ok(board) =>
                {
                    board.display();
                },
                Err(err) => eprint!("{}", err),
            }
            print_bb(knight_mask(11));
        },
        Commands::Explore { depth, verbose } =>
        {
            match Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -")
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
        Commands::Play { side } =>
        {
            let is_white = if side == "w"
            {
                true
            }
            else if side == "b"
            {
                false
            }
            else
            {
                panic!("This is not a valid side.")
            };
            play(is_white);
        },
    }
}
