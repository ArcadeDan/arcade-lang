
use logos::Logos;
use std::io::{stdin, BufRead};


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
    Expression,
    Literal,
    Grouping,
    Unary,
    Binary,
    Operator,
    ERROR
}

struct GrmExpr {
    grammar: Grammar
}

impl GrmExpr {
    fn new(grm: Grammar) -> Self {
        match grm {
            Grammar::Literal => Self { grammar: Grammar::Literal },
            Grammar::Unary => Self { grammar: Grammar::Unary },
            Grammar::Binary => Self { grammar: Grammar::Binary },
            Grammar::Grouping => Self { grammar: Grammar::Grouping },
            _ => Self { grammar: Grammar::ERROR }
        }
    }
}

struct GrmLiteral;
struct GrmGrouping;
struct GrmUnary;
struct GrmOperator {
    operator: Token
}
struct GrmBinary {
    left: GrmExpr,
    operator: GrmOperator,
    right: GrmExpr
}
impl GrmBinary {
    fn new(l: GrmExpr, op: GrmOperator, r: GrmExpr) -> Self {
        Self { left: l, operator: op, right: r }
    }
}

impl GrmOperator {
    fn new(op: Token) -> Self {
        match op {
            Token::TAdd => Self { operator: Token::TAdd },
            Token::TSubtract => Self { operator: Token::TSubtract },
            Token::TDivide => Self { operator: Token::TDivide },
            Token::TMultiply => Self { operator: Token::TMultiply},
            _ => Self { operator: Token::ERROR }
        }
        
    }
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
