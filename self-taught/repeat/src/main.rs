use clap::Parser;

#[derive(Parser)]
struct Cli {
    string: String,
    #[clap(default_value = "3")]
    number: usize,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.string.repeat(args.number));
}
