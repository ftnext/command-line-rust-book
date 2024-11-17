use clap::Parser;

#[derive(Parser)]
struct Cli {
    string: String,
    number: usize,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.string.repeat(args.number));
}
