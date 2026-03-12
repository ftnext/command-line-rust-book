use clap::Parser;
use two_face::re_exports::syntect;

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser)]
struct Cli {
    /// The path to the Python file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path.display(), err)))?;

    let syn_set = two_face::syntax::extra_newlines();
    let theme_set = two_face::theme::extra();

    let syn_ref = syn_set.find_syntax_by_extension("py").unwrap();
    let theme = &theme_set[two_face::theme::EmbeddedThemeName::Nord];
    let mut highlighter = syntect::easy::HighlightLines::new(syn_ref, theme);
    let lines = content.lines();
    let mut output = String::new();

    for line in lines {
        let ranges = highlighter.highlight_line(line, &syn_set).unwrap();
        let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], false);
        output.push_str(&escaped);
        output.push('\n');
    }

    println!("{}", output);
    Ok(())
}
