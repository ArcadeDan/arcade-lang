use logos::Logos;
use std::{
    io::{stdin, BufRead},
    ops::BitOr,
};

#[derive(Logos, Debug, PartialEq, Clone)]
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
    // ex: 34422. == INVALID     34423.0 == VALID
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
    #[token("!")]
    TBang,
    #[token("!=")]
    TBangeq,
    #[token("<=")]
    TLesseq,
    #[token(">=")]
    TGreatereq,
    #[token(">")]
    TGreater,
    #[token("<")]
    TLess,
    #[token("=")]
    Tassign,
    #[token("==")]
    TEqual,
    #[token("if")]
    TIf,
    #[token("while")]
    TWhile,
    #[token("for")]
    TFor,
    #[token(";")]
    TExpressiondelimiter,
    #[token("false")]
    Tfalse,
    #[token("true")]
    Ttrue,
    #[error]
    #[regex(r"[\t\n\f]+", logos::skip)]
    ERROR,
}



#[derive(PartialEq, Eq)]
enum TokenType {

    // operators
    TKPlus,
    TKMinus,
    TKMultiply,
    TKDivide,
    TKModulus,

    // integrals
    TKInteger,
    TKFloat,

    // keywords
    TKIf,
    TKWhile,
    TKFor,
    TKSemicolon,
    TKBind,
    TKReturn,

    // boolean logic
    TKAnd,
    TKOr,
    TKNot,

    // comparison
    TKEqual,
    TKGreater,
    TKGreaterEq,
    TKLess,
    TKLessEq,
    
    // conditionals
    TKIf,
    TKelse

}


enum Grammar {
    Literal,
    Grouping,
    Unary,
    Binary,
    ERROR,
}




struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self, offset: usize) -> &Token {
        &self.tokens[self.pos + offset]
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn next_token(&self) -> &Token {
        self.pos += 1;
        self.peek(1)
        
    }
    
    fn expression() {
        todo!()
    }

    fn check

    fn is_match(&self, token_type: TokenType) -> bool {
        !self.is_eof() && self.peek()
    
}


fn main() {
    
    for line in stdin().lock().lines() {
        let expression = line.unwrap();
        let lexer = Token::lexer(&expression);
        let tokens: Vec<_> = lexer.spanned().filter(|x| x.0 != Token::ERROR).collect();
        let mut parser = Parser::new(tokens);
        
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
