use std::io::{stdin, BufRead};
use std::str::FromStr;
use std::{fs, io::BufReader};
use anyhow::Ok;
use anyhow::Result;
use logos::Logos;


#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("+")]
    TAdd,
    #[token("-")]
    TSubtract,
    #[token("*")]
    TMultiply,
    #[token("/")]
    TDivide,
    #[regex(r"[0-9]+")]
    TInteger,
    #[regex(r"(([0-9]+)[.])\d+")]
    TFloat,
    #[token(r"%")]
    TModulus,
    #[token(r"(")]
    TOpar,
    #[token(r")")]
    TCpar,
    #[error]
    #[regex(r"[\t\n\f]+", logos::skip)]
    ERROR,
}




fn main() {
    
    for line in stdin().lock().lines() {
        let expression = line.unwrap();
        let mut lexer = Token::lexer(&expression);
        let tokens: Vec<_> = lexer.spanned().filter(|x| x.0 != Token::ERROR).collect();
        for token in tokens.iter() {
            print!("{:?} ", token.0);
        }
        println!("\n");
    }
}
#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::Token;

    #[test]
    fn tokens_test() {
        todo!()
    }

}
