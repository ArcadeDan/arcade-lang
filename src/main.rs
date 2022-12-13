use anyhow::Ok;
use anyhow::Result;
use logos::Logos;
use std::io::{stdin, BufRead};
use std::str::FromStr;
use std::{fs, io::BufReader};

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
    TFloat, // floating points must have a character after the '.'
    #[token("%")]
    TModulus,
    #[token("(")]
    TLparen,
    #[token(")")]
    TRparen,
    #[token("[")]
    TLbrack,
    #[token("]")]
    TRbrack,
    #[token("{")]
    TLbrace,
    #[token("}")]
    TRbrace,
    #[regex(r#""[^"]*""#)]
    TString,
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
