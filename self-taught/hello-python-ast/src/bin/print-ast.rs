use rustpython_parser::{ast, Parse};

fn main() {
    let python_source = r#"
def greet(person):
    return person.name + " " + person.age
"#;
    let ast = ast::Suite::parse(python_source, "<embedded>");
    assert!(ast.is_ok());
    println!("{:#?}", ast.unwrap());
}
