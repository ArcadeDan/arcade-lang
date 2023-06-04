use crate::lexer::TokenType;

pub struct Parser {
    pub tokens: Vec<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<TokenType>) -> Self {
        Self {
            tokens: tokens.clone(),
            current: 0,
        }
    }
    pub fn z

}

