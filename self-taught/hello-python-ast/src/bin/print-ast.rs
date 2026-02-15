use rustpython_parser::{Parse, ast};

fn main() {
    let python_source = r#"
def plus_one_ng(numbers: list[int]) -> list[int]:
    return [n + 1 for n in numbers]
"#;
    let ast = ast::Suite::parse(python_source, "<embedded>");
    assert!(ast.is_ok());
    println!("{:#?}", ast.unwrap());
}
