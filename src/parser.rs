use std::ops::Range;

use crate::lexer::Token;

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

pub struct Parser {
    tokens: Vec<(Token, Range<usize>)>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<(Token, Range<usize>)>) -> Self {
        Self { tokens: tokens.to_vec(), pos: 0 }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn peek(&self, offset: usize) -> &Token {
        &self.tokens[self.pos + offset].0
        
    }

    pub fn check(&self, match_token: TokenKind) -> bool {
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