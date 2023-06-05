use crate::lexer::TokenType;

pub struct Parser {
    pub tokens: Vec<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<TokenType>) -> Self {
        Self {
            tokens: tokens.clone(),
            current: 0,
        }
    }

    pub fn cursor(&self) -> usize {
        self.current
    }
    pub fn advance(&mut self) -> TokenType {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    pub fn previous(&self) -> TokenType {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
    pub fn peek(&self) -> TokenType {
        self.tokens.get(self.current).unwrap().clone()
    }
    pub fn peek_next(&self) -> TokenType {
        self.tokens.get(self.current + 1).unwrap().clone()
    }
    //checks if the next token is the same as the one passed in
    pub fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == token
    }
    pub fn match_token(&mut self, token: TokenType) -> bool {
        if self.check(token.clone()) {
            self.advance();
            return true;
        }
        false
    }
    /*
    pub fn consume(&mut self, token: TokenType, message: &str) {
        if self.check(token.clone()) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }
    */

    
}

struct ASTNode {
    token: TokenType,
    children: Vec<ASTNode>,
}

impl ASTNode {
    fn new(token: TokenType) -> Self {
        Self {
            token,
            children: Vec::new(),
        }
    }

}

struct AST {
    root: ASTNode,
}

impl AST {
    fn new(root: ASTNode) -> Self {
        Self { root }
    }
}


