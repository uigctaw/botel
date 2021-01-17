use botel_tokenizer::Token;
//use std::collections::VecDeque;
use std::boxed::Box;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum AST {
    BinOp(BinOp),
    Name(String),
    Integer(i64),  // This will need a designeted type
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum BinOpName {
    Equation,
    Sum,
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct BinOp{
    left: Box<AST>,
    right: Box<AST>,
    what: BinOpName,
}


pub fn parse(tokens: Vec<Token>) -> Vec<AST> {
    let postfix_tokens = convert_to_postfix_notation(tokens);
    let mut ast = Vec::new();
    for token in postfix_tokens.into_iter() {
        match token {
            Token::Name(name) => {
                ast.push(AST::Name(name))
            },
            Token::Integer(num) => { 
                ast.push(AST::Integer(num.parse::<i64>().unwrap()));
            },
            Token::Equals(_) => {
                let maybe_right = ast.pop();
                let maybe_left = ast.pop();
                if let (Some(left), Some(right)) = (maybe_left, maybe_right) {
                    let eq = BinOp {
                        left: Box::new(left),
                        right: Box::new(right),
                        what: BinOpName::Equation,
                    };
                    ast.push(AST::BinOp(eq));
                } else { panic!("some syntax error") }
            },
            Token::Plus(_) => {
                let maybe_right = ast.pop();
                let maybe_left = ast.pop();
                if let (Some(left), Some(right)) = (maybe_left, maybe_right) {
                    let sum = BinOp {
                        left: Box::new(left),
                        right: Box::new(right),
                        what: BinOpName::Sum,
                    };
                    ast.push(AST::BinOp(sum));
                } else { panic!("some syntax error") }
            },
            _ => panic!("unexpected token"),
        }
    }

    ast
}


fn convert_to_postfix_notation(tokens: Vec<Token>) -> Vec<Token> {
    let mut postfix_tokens = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens.into_iter() {
        match token {
            Token::Equals(_) => { operator_stack.push(token) },
            Token::Plus(_) => { operator_stack.push(token) },
            _ => { postfix_tokens.push(token) },
        }
    }
    for operator in operator_stack.into_iter().rev() {
        postfix_tokens.push(operator);
    }

    postfix_tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_name_or_integer() {
        let result = parse(vec![Token::Name("Hi".to_string())]);
        assert_eq!(result, vec![AST::Name("Hi".to_string())]);

        let result = parse(vec![Token::Integer("1234".to_string())]);
        assert_eq!(result, vec![AST::Integer(1234)]);
    }

    #[test]
    fn equation_name_equals_value_to_postfix_notation() {
        let tokens = vec![
            Token::Name("Hi".to_string()),
            Token::Equals("does not matter".to_string()),
            Token::Integer("1234".to_string()),
        ];
        let postfixed = convert_to_postfix_notation(tokens);
        assert_eq!(
            postfixed,
            vec![
                Token::Name("Hi".to_string()),
                Token::Integer("1234".to_string()),
                Token::Equals("does not matter".to_string()),
            ],
        );
    }

    #[test]
    fn equation_name_equals_value() {
        let tokens = vec![
            Token::Name("Hi".to_string()),
            Token::Equals("does not matter".to_string()),
            Token::Integer("1234".to_string()),
        ];
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![
                AST::BinOp(BinOp {
                    left: Box::new(AST::Name("Hi".to_string())),
                    right: Box::new(AST::Integer(1234)),
                    what: BinOpName::Equation,
                }),
            ],
        );
    }

    #[test]
    fn equation_name_equals_sum_to_postfix_notation() {
        let tokens = vec![
            Token::Name("Hi".to_string()),
            Token::Equals("does not matter".to_string()),
            Token::Integer("2".to_string()),
            Token::Plus("also does not matter".to_string()),
            Token::Name("Hello".to_string()),
        ];
        let postfixed = convert_to_postfix_notation(tokens);
        assert_eq!(
            postfixed,
            vec![
            Token::Name("Hi".to_string()),
            Token::Integer("2".to_string()),
            Token::Name("Hello".to_string()),
            Token::Plus("also does not matter".to_string()),
            Token::Equals("does not matter".to_string()),
            ],
        );
    }

    #[test]
    fn equation_name_equals_sum() {
        let tokens = vec![
            Token::Name("Hi".to_string()),
            Token::Equals("does not matter".to_string()),
            Token::Integer("2".to_string()),
            Token::Plus("also does not matter".to_string()),
            Token::Name("Hello".to_string()),
        ];
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![
                AST::BinOp(BinOp {
                    left: Box::new(AST::Name("Hi".to_string())),
                    right: Box::new(AST::BinOp(BinOp {
                        left: Box::new(AST::Integer(2)),
                        right: Box::new(AST::Name("Hello".to_string())),
                        what: BinOpName::Sum,
                    })),
                    what: BinOpName::Equation,
                }),
            ],
        );
    }
}
