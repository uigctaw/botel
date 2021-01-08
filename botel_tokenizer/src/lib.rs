use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Whitespace(String),    
    Equals(String),    
}


lazy_static! {
    static ref REGEX_AND_KIND: Vec<(Regex, fn(String) -> Token)> = vec![
        (Regex::new(r"^\s+").unwrap(), Token::Whitespace),
        (Regex::new(r"^=").unwrap(), Token::Equals),
        //(Regex::new(r"^\d+[\s$]").unwrap(), TokenKind::Int),
        //(Regex::new(r"^[_A-Za-z][_A-Za-z0-9]*").unwrap(), TokenKind::Name),
    ];
}



pub fn tokenize(program_text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut slice = program_text;
    while slice.len() > 0 {
        for (regex, token_kind) in REGEX_AND_KIND.iter() {
            if let Some(match_) = regex.find(slice) {
                let matched_slice = slice[.. match_.end()].to_string();
                tokens.push(token_kind(matched_slice));
                slice = &slice[match_.end() ..];
                break;
            }
        }
    }
    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_whitespace() {
        let tokens = tokenize("  ");
        assert_eq!(tokens, vec![Token::Whitespace("  ".to_string())]);
    }

    #[test]
    fn find_equals() {
        let tokens = tokenize("==");
        assert_eq!(
            tokens, 
            vec![
                Token::Equals("=".to_string()),
                Token::Equals("=".to_string()),
            ],
        );
    }
}
