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
    let htmlified =
        syntect::html::highlighted_html_for_string(TOML_TEXT, &syn_set, syn_ref, theme).unwrap();
    println!("{}", htmlified);
}
