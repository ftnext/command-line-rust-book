use two_face::re_exports::syntect;

const TOML_TEXT: &str = "\
[section]
key = 123
";

fn main() {
    let syn_set = two_face::syntax::extra_newlines();
    let theme_set = two_face::theme::extra();

    let syn_ref = syn_set.find_syntax_by_extension("toml").unwrap();
    let theme = &theme_set[two_face::theme::EmbeddedThemeName::Nord];
    let mut highlighter = syntect::easy::HighlightLines::new(syn_ref, theme);
    let lines = TOML_TEXT.lines();
    let mut output = String::new();

    for line in lines {
        let ranges = highlighter.highlight_line(line, &syn_set).unwrap();
        let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], false);
        output.push_str(&escaped);
        output.push('\n');
    }

    println!("{}", output);
}
