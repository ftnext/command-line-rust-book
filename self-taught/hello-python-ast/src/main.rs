use rustpython_ast::{Arg, Arguments, Expr, ExprAttribute, Ranged, Visitor};
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

struct KotohaVisitor<'a> {
    source: &'a str,
}

fn byte_offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let prefix = &source[..offset];
    let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
    let col = match prefix.rfind('\n') {
        Some(last_newline) => offset - last_newline,
        None => offset + 1,
    };
    (line, col)
}

impl Visitor for KotohaVisitor<'_> {
    fn visit_arguments(&mut self, node: Arguments) {
        for arg in node.posonlyargs {
            self.visit_arg(arg.def);
            if let Some(default) = arg.default {
                self.visit_expr(*default);
            }
        }
        for arg in node.args {
            self.visit_arg(arg.def);
            if let Some(default) = arg.default {
                self.visit_expr(*default);
            }
        }
        if let Some(vararg) = node.vararg {
            self.visit_arg(*vararg);
        }
        for arg in node.kwonlyargs {
            self.visit_arg(arg.def);
            if let Some(default) = arg.default {
                self.visit_expr(*default);
            }
        }
        if let Some(kwarg) = node.kwarg {
            self.visit_arg(*kwarg);
        }
    }

    fn visit_arg(&mut self, node: Arg) {
        if let Some(annotation) = &node.annotation {
            if let Expr::Subscript(subscript) = annotation.as_ref() {
                if let Expr::Name(name) = subscript.value.as_ref() {
                    if name.id.as_str() == "list" {
                        let range = annotation.range();
                        let annotation_text =
                            &self.source[usize::from(range.start())..usize::from(range.end())];
                        let (line, col) =
                            byte_offset_to_line_col(self.source, usize::from(node.range().start()));
                        println!(
                            "Fix type hint `{}: {}` at {}:{}",
                            node.arg, annotation_text, line, col
                        );
                    }
                }
            }
        }
        self.generic_visit_arg(node);
    }
}

fn main() {
    let python_source = r#"
def plus_one_ng(numbers: list[int]) -> list[int]:
    return [n + 1 for n in numbers]

def plus_one_ok(numbers: Iterable[int]) -> list[int]:
    return [n + 1 for n in numbers]
"#;
    let ast = ast::Suite::parse(python_source, "<embedded>").unwrap();

    let mut visitor = KotohaVisitor {
        source: python_source,
    };
    for stmt in ast {
        visitor.visit_stmt(stmt);
    }
}
