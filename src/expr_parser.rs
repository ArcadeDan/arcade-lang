use crate::parser::TokenKind;


enum Precedence {
    None,
    Assign,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    List,
    Primary
}

impl From<TokenKind> for Precedence {
    fn from(token: TokenKind) -> Precedence {
        match token {
            TokenKind::TKBind => Precedence::Assign,
            TokenKind::TKOr => Precedence::Or,
            TokenKind::TKAnd => Precedence::And,
            TokenKind::TKBangEqual | TokenKind::TKEqual => Precedence::Equality,
            TokenKind::TKLess
                | TokenKind::TKLessEq
                | TokenKind::TKGreater
                | TokenKind::TKGreaterEq => Precedence::Comparison,
            TokenKind::TKPlus | TokenKind::TKMinus => Precedence::Term,
            TokenKind::TKMultiply | TokenKind::TKDivide => Precedence::Factor,
            TokenKind::TKBang => Precedence::Unary,
            TokenKind::TKLeftParan => Precedence::Call,
            TokenKind::TKDot => Precedence::Call,
            TokenKind::TKLeftBracket => Precedence::List
            _ => Precedence::None
        }
    }
}