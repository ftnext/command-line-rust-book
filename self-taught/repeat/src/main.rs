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
        #[clap(default_value = "3")]
        number: usize,
    },
    Option {
        #[clap(short)]
        string: String,
        #[clap(short, default_value = "3")]
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
