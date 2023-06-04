use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
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
#[derive(Debug, Clone)]
pub struct TokenType {
    token: Token,
    span: std::ops::Range<usize>,
}

impl TokenType {
    pub fn new(token: Token, span: std::ops::Range<usize>) -> Self {
        Self { token, span }
    }
}