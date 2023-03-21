use logos::Logos;
use std::io::{stdin, BufRead};

use crate::{lexer::Token, parser::Parser};

mod lexer;
mod parser;
mod expr_parser;
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
    use crate::{parser, Token};
    use logos::Logos;

    #[test]
    fn test_parse_expr() {
        let tokens: Vec<Token> = vec![
            Token::TLparen,
            Token::TInteger,
            Token::TAdd,
            Token::TInteger,
            Token::TRparen,
        ];

        //let parser = parser::Parser::new(&tokens);
        todo!()
    }
}
