

#[derive(Debug, PartialEq)]
pub enum TokenType {
  Illegal,
  Eof,
  Indent(String),
  Int(usize),
  Assign,
  Plus,
  Comma,
  Semicolon,
  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Function,
  Let,
  Eq,
  NotEq,
  Bang,
}

#[derive(Debug)]
pub struct Token {
  token_type: TokenType,
  
}

impl Token {
  pub fn new(token_type: TokenType) -> Self {
    Token {
      token_type,
    }
  }
  pub fn token_type(&self) -> &TokenType {
    &self.token_type
  }
}