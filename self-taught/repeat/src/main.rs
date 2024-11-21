use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Position {
        string: String,
        #[arg(default_value = "3")]
        number: usize,
    },
    Option {
        #[arg(short, long)]
        string: String,
        #[arg(short, long, default_value = "3")]
        number: usize,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Position { string, number } => {
            println!("{}", string.repeat(number));
        }
        Commands::Option { string, number } => {
            println!("{}", string.repeat(number));
        }
    }
}
