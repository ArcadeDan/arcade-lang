use logos::Logos;
use std::{
    io::{stdin, BufRead},
    ops::Range,
};

use crate::{
    lexer::{Token, TokenType},
    parser::Parser,
};

mod lexer;
mod parser;

fn main() {
    for line in stdin().lock().lines() {
        let expression = line.unwrap();
        let lexer = Token::lexer(&expression);

        // we lex the tokens from standard input and collect them into a vector which
        // includes the span of the token

        let terminals: Vec<(Token, Range<usize>)> =
            lexer.spanned().filter(|x| x.0 != Token::ERROR).collect();
        let mut tokens: Vec<TokenType> = Vec::new();
        for token in terminals.iter() {
            let t = TokenType::new(token.0, token.1.clone());
            tokens.push(t);
        }
        let mut parser = Parser::new(&tokens);

        for token in tokens.iter() {
            print!("{:?} : ", token.tokentype);
        }
        println!("\n");
    }
}
#[cfg(test)]
mod tests {
    use crate::{lexer, Token};
    use logos::Logos;

    #[test]
    fn test_lexer() {
        let expression = "2 + 2";
        let lexer = Token::lexer(expression);
        let tokens: Vec<_> = lexer.spanned().filter(|x| x.0 != Token::ERROR).collect();

        assert_eq!(Token::INTEGER, tokens.get(0).unwrap().0);
        assert_eq!(Token::PLUS, tokens.get(1).unwrap().0);
        assert_eq!(Token::INTEGER, tokens.get(2).unwrap().0);
    }
}
