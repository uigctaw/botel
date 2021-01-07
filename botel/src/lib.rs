use std::collections::HashMap;
use std::any::Any;

use botel_tokenizer::tokenize;
use botel_parser::parse;


pub fn run<'a>(
    program_text: &str,
    find: Vec<&'a str>,
    ) -> HashMap<&'a str, Box<dyn Any>> {

    let tokens = tokenize(program_text);
    let program = parse(&tokens);

    let mut results = HashMap::<&str, Box<dyn Any>>::new();
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_equals_int() {
        let ret = run("x = 2", vec!["x"]);
        let maybe_x = ret.get("x");
        let any_x = match maybe_x {
            Some(maybe_x) => maybe_x,
            None => panic!("x is missing!"),
        };
        let x = any_x.downcast_ref::<i32>().unwrap();
        assert_eq!(*x, 2);
    }
}
