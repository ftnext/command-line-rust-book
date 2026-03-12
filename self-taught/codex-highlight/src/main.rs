use two_face::re_exports::syntect;

const PYTHON_SOURCE: &str = r#"
def plus_one_ng(numbers: list[int]) -> list[int]:
    return [n + 1 for n in numbers]

def plus_one_ok(numbers: Iterable[int]) -> list[int]:
    return [n + 1 for n in numbers]
"#;

fn main() {
    let syn_set = two_face::syntax::extra_newlines();
    let theme_set = two_face::theme::extra();

    let syn_ref = syn_set.find_syntax_by_extension("py").unwrap();
    let theme = &theme_set[two_face::theme::EmbeddedThemeName::Nord];
    let mut highlighter = syntect::easy::HighlightLines::new(syn_ref, theme);
    let lines = PYTHON_SOURCE.lines();
    let mut output = String::new();

    for line in lines {
        let ranges = highlighter.highlight_line(line, &syn_set).unwrap();
        let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], false);
        output.push_str(&escaped);
        output.push('\n');
    }

    println!("{}", output);
}
