use logos::Logos;
use std::io::{stdin, BufRead};

use crate::{lexer::Token, parser::Parser};

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

        // we then pass the tokens into the parser, but this time we only pass the token and not the ranges
        let mut parser = Parser::new(tokens.iter().map(|x| x.0.clone()).collect());

        for token in tokens.iter() {
            print!("{:?} : ", token.0);
        }
        println!("\n");
    }
}
#[cfg(test)]
mod tests {
    use crate::Token;
    use logos::Logos;

    #[test]
    fn negative_b() {
        todo!()
    }
}
