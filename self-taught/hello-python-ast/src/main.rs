use rustpython_parser::{ast, Parse};

fn main() {
    let python_source = "if True: pass # comment";
    let ast = ast::Suite::parse(python_source, "<embedded>");
    assert!(ast.is_ok());
    println!("{:#?}", ast.unwrap());
}
