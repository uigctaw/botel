use regex::Regex;
use lazy_static::lazy_static;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Whitespace(String),    
    Equals(String),    
    Integer(String),
    Name(String),
    Plus(String),
}


lazy_static! {
    static ref REGEX_AND_KIND: Vec<(Regex, fn(String) -> Token)> = vec![
        (Regex::new(r"^\s+").unwrap(), Token::Whitespace),
        (Regex::new(r"^=").unwrap(), Token::Equals),
        (Regex::new(r"^(\d+)(?:\s|$)").unwrap(), Token::Integer),
        (Regex::new(r"^[_A-Za-z][_A-Za-z0-9]*").unwrap(), Token::Name),
    ];
}


pub fn tokenize(program_text: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut slice = &program_text[..];
    while slice.len() > 0 {
        let mut matched = false;
        for (regex, token_kind) in REGEX_AND_KIND.iter() {
            if let Some(maybe_captures) = regex.captures(slice) {
                let match_ = match maybe_captures.len() {
                    1 => maybe_captures.get(0).unwrap(),
                    2 => maybe_captures.get(1).unwrap(),
                    _ => panic!("Too many captures!"),
                };
                if match_.start() != 0 { panic!("Wat?!") };
                let matched_slice = slice[.. match_.end()].to_string();
                tokens.push(token_kind(matched_slice));
                slice = &slice[match_.end() ..];
                matched = true;
                break;
            }
        }
        if !matched { panic!("Could not parse {}", slice) };
    }
    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_whitespace() {
        let tokens = tokenize("  ".to_string());
        assert_eq!(tokens, vec![Token::Whitespace("  ".to_string())]);
    }

    #[test]
    fn find_equals() {
        let tokens = tokenize("==".to_string());
        assert_eq!(
            tokens, 
            vec![
                Token::Equals("=".to_string()),
                Token::Equals("=".to_string()),
            ],
        );
    }

    #[test]
    fn find_integer() {
        let tokens = tokenize("12 34".to_string());
        assert_eq!(
            tokens, 
            vec![
                Token::Integer("12".to_string()),
                Token::Whitespace(" ".to_string()),
                Token::Integer("34".to_string()),
            ],
        );
    }

    #[test]
    fn find_name() {
        let tokens = tokenize("a _ Bc _d".to_string());
        assert_eq!(
            tokens, 
            vec![
                Token::Name("a".to_string()),
                Token::Whitespace(" ".to_string()),
                Token::Name("_".to_string()),
                Token::Whitespace(" ".to_string()),
                Token::Name("Bc".to_string()),
                Token::Whitespace(" ".to_string()),
                Token::Name("_d".to_string()),
            ],
        );
    }
}
