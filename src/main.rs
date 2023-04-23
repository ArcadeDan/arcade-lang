use logos::Logos;
use std::io::{stdin, BufRead};

use crate::{lexer::Token, parser::Parser};

mod expr_parser;
mod lexer;
mod parser;
/*

enum Grammar {
    Literal,
    Grouping,
    Unary,
    Binary,
    ERROR,
}
*/

fn main() {
    for line in stdin().lock().lines() {
        let expression = line.unwrap();
        let lexer = Token::lexer(&expression);

        // we lex the tokens from standard input and collect them into a vector which
        // includes the span of the token

        let tokens: Vec<_> = lexer.spanned().filter(|x| x.0 != Token::ERROR).collect();
        let mut parser = Parser::new(&tokens);
        
        for token in tokens.clone().iter() {
            print!("{:?} : ", token.0);
        }
        println!("\n");
    }
}
#[cfg(test)]
mod tests {
    use crate::{parser, Token, lexer};
    use logos::Logos;

    #[test]
    fn test_lexer() {
        let expression = "2 + 2";
        let lexer = Token::lexer(expression);
        let tokens: Vec<_> = lexer.spanned().filter(|x| x.0 != Token::ERROR).collect();

        assert_eq!(Token::TInteger, tokens.get(0).unwrap().0);
        assert_eq!(Token::TAdd, tokens.get(1).unwrap().0);
        assert_eq!(Token::TInteger, tokens.get(2).unwrap().0);

    }
}
