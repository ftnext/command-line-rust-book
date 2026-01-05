use rustpython_ast::{ExprAttribute, Visitor};
use rustpython_parser::{Parse, ast};

struct AttributeCounter {
    attributes_count: usize,
}

impl Visitor for AttributeCounter {
    fn visit_expr_attribute(&mut self, node: ExprAttribute) {
        self.attributes_count += 1;
        self.generic_visit_expr_attribute(node);
    }
}

fn main() {
    let python_source = r#"
def greet(person):
    return person.name + " " + person.age
"#;
    let ast = ast::Suite::parse(python_source, "<embedded>").unwrap();

    let mut counter = AttributeCounter {
        attributes_count: 0,
    };
    println!("Attributes count: {}", counter.attributes_count);
    for stmt in ast {
        counter.visit_stmt(stmt);
    }

    println!("Attributes count: {}", counter.attributes_count);
}
