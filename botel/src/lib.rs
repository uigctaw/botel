use std::collections::HashMap;
//use std::any::Any;

use botel_tokenizer::tokenize;
use botel_parser::parse;
use botel_parser::AST;
use botel_parser::BinOpName;
use botel_parser::BinOp;


pub fn run(
    program_text: String,
    find: String,
    ) -> Option<i64> {

    let tokens = tokenize(program_text);
    let ast = parse(tokens);
    let names = get_names(&ast);
    let result = names.get(&find);
    if let Some(node) = result {
        if let AST::BinOp(BinOp {
                right,
                ..
        }) = node {
            if let AST::Integer(value) = **right{
                return Some(value)
            }
        }
    }
    None
}


fn get_names<'a>(ast: &'a Vec<AST>) -> HashMap<&String, &AST> {
    let mut names = HashMap::new();
    for node in ast.iter() {
        if let AST::BinOp(BinOp {
                left,
                what: BinOpName::Equation,
                ..
        }) = node {
            if let AST::Name(name) = &**left {
                names.insert(name, node);
            }
        }
    }
    names
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_equals_int() {
        let maybe_x = run("x = 2".to_string(), "x".to_string());
        if let Some(x) = maybe_x {
            assert_eq!(x, 2);
        } else {
            panic!("Could not calculate!");
        }
    }
}
