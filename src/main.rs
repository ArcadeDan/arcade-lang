use logos::Logos;
use std::{io::{stdin, BufRead}, ops::BitOr};

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



enum Grammar {
    Literal,
    Grouping,
    Unary,
    Binary,
    ERROR,
}

struct GrmExpr {
    grammar: Grammar,
}

impl GrmExpr {
    fn new(grm: Grammar) -> Self {
        match grm {
            Grammar::Literal => Self {
                grammar: Grammar::Literal,
            },
            Grammar::Unary => Self {
                grammar: Grammar::Unary,
            },
            Grammar::Binary => Self {
                grammar: Grammar::Binary,
            },
            Grammar::Grouping => Self {
                grammar: Grammar::Grouping,
            },
            _ => Self {
                grammar: Grammar::ERROR,
            },
        }
    }
}

struct GrmLiteral {
    literal: Token,
}

impl GrmLiteral {
    fn new(ltrl: Token) -> Self {
        match ltrl {
            Token::TFloat => Self {
                literal: Token::TFloat,
            },
            Token::TInteger => Self {
                literal: Token::TInteger,
            },
            Token::TString => Self {
                literal: Token::TString,
            },
            Token::Ttrue => Self {
                literal: Token::Ttrue,
            },
            Token::Tfalse => Self {
                literal: Token::Tfalse,
            },
            _ => Self {
                literal: Token::ERROR,
            },
        }
    }
}
struct GrmGrouping;
struct GrmUnary;
struct GrmOperator {
    operator: Token,
}
struct GrmBinary {
    left: GrmExpr,
    operator: GrmOperator,
    right: GrmExpr,
}
impl GrmBinary {
    fn new(l: GrmExpr, op: GrmOperator, r: GrmExpr) -> Self {
        Self {
            left: l,
            operator: op,
            right: r,
        }
    }
}

impl GrmOperator {
    fn new(op: Token) -> Self {
        match op {
            Token::TAdd => Self {
                operator: Token::TAdd,
            },
            Token::TSubtract => Self {
                operator: Token::TSubtract,
            },
            Token::TDivide => Self {
                operator: Token::TDivide,
            },
            Token::TMultiply => Self {
                operator: Token::TMultiply,
            },
            _ => Self {
                operator: Token::ERROR,
            },
        }
    }
}

fn parse(tokens: Vec<Token>) {
    let operators: Vec<Token> = tokens.clone().into_iter().filter(|t| match t {
        Token::TAdd => true,
        Token::TSubtract => true,
        Token::TDivide => true,
        Token::TMultiply => true,
        _ => false
    })
    .collect();

    let literals: Vec<Token> = tokens.clone().into_iter().filter(|t| match t {
        Token::TInteger => true,
        Token::TString => true,
        Token::Ttrue => true,
        Token::Tfalse => true,
        _ => false
    })
    .collect();



}


fn main() {
    for line in stdin().lock().lines() {
        let expression = line.unwrap();
        let lexer = Token::lexer(&expression);
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
