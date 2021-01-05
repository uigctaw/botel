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

    let mut results = HashMap::new();
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_equals() {
        let ret = run("x = 2", vec!["x"]);
        //assert_eq!(2 + 2, 4);
    }
}
