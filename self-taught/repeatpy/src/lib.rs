use pyo3::prelude::*;
// use pyo3::{prelude::{pyfunction, pymodule}, types::{PyModule, PyModuleMethods}, wrap_pyfunction, Bound, PyResult};
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
        #[arg(default_value_t = 3)]
        number: usize,
    },
    Option {
        #[arg(short, long)]
        string: String,
        #[arg(short, long, default_value_t = 3)]
        number: usize,
    },
}

#[pyfunction]
fn repeat_cli() {
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

#[pymodule]
fn repeatpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(repeat_cli, m)?)?;
    Ok(())
}
