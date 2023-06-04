use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("*")]
    MULT,
    #[token("/")]
    DIV,
    #[regex(r"[0-9]+")]
    INTEGER,
    #[regex(r"(([0-9]+)[.])\d+")]
    FLOAT, // floating points must have a character after the '.'
    // ex: 34422. == INVALID     34423.0 == VALID
    #[token("%")]
    MOD,
    #[regex(r#""[^"]*""#)]
    STRING,
    #[token("!")]
    BANG,
    #[token("!=")]
    BANG_EQUAL,
    #[token("<=")]
    LESS_EQUAL,
    #[token(">=")]
    GREATER_EQUAL,
    #[token(">")]
    GREATER,
    #[token("<")]
    LESSER,
    #[token("=")]
    EQUAL,
    #[token("==")]
    EQUAL_COMPARISON,
    #[token(";")]
    DELMITTER,
    #[token("false")]
    FALSE,
    #[token("true")]
    TRUE,
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