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
enum TokenKind {
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
    TKelse,
    TKWhile,
    TKFor,

    // misc
    TKERROR,
}

impl From<TokenKind> for Token {
    fn from(token: TokenKind) -> Self {
        match token {
            TokenKind::TKPlus => Self::TAdd,
            TokenKind::TKMinus => Self::TSubtract,
            TokenKind::TKMultiply => Self::TMultiply,
            TokenKind::TKDivide => Self::TDivide,
            TokenKind::TKModulus => Self::TModulus,
            TokenKind::TKInteger => Self::TInteger,
            TokenKind::TKFloat => Self::TFloat,
            TokenKind::TKIf => Self::TIf,
            TokenKind::TKWhile => Self::TWhile,
            TokenKind::TKFor => Self::TFor,
            TokenKind::TKSemicolon => Self::TExpressiondelimiter,
            TokenKind::TKBind => Self::Tassign,
            TokenKind::TKEqual => Self::TEqual,
            TokenKind::TKGreater => Self::TGreater,
            TokenKind::TKGreaterEq => Self::TGreatereq,
            TokenKind::TKLess => Self::TLess,
            TokenKind::TKLessEq => Self::TLesseq,
            _ => Self::ERROR,
        }
    }
}

impl From<&Token> for TokenKind {
    fn from(token: &Token) -> Self {
        match token {
            Token::TAdd => Self::TKPlus,
            Token::TSubtract => Self::TKMinus,
            Token::TMultiply => Self::TKMultiply,
            Token::TDivide => Self::TKDivide,
            Token::TModulus => Self::TKModulus,
            Token::TInteger => Self::TKInteger,
            Token::TFloat => Self::TKFloat,
            Token::TIf => Self::TKIf,
            Token::TWhile => Self::TKWhile,
            Token::TFor => Self::TKFor,
            Token::TExpressiondelimiter => Self::TKSemicolon,
            Token::Tassign => Self::TKBind,
            Token::TEqual => Self::TKEqual,
            Token::TGreater => Self::TKGreater,
            Token::TGreatereq => Self::TKGreaterEq,
            Token::TLess => Self::TKLess,
            Token::TLesseq => Self::TKLessEq,
            _ => Self::TKERROR,
        }
    }
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
        Self { tokens, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self, offset: usize) -> &Token {
        &self.tokens[self.pos + offset]
    }

    fn check(&self, match_token: TokenKind) -> bool {
        let token = self.peek(1);
        token.clone() == match_token.into()
        
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn next_token(&mut self) -> &Token {
        self.pos += 1;
        self.peek(1)
    }

    fn expression() {
        todo!()
    }

    fn is_match(&self, token_type: TokenKind) -> bool {
        todo!()
    }
}

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
