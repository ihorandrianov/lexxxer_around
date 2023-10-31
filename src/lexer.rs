use std::iter::Peekable;

use crate::token::{Token, TokenType};


pub struct Lexer<'a> {
  input: Peekable<Box<dyn Iterator<Item = char> + 'a>>,
  ch: Option<u8>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let iterator = Box::new(input.chars()) as Box<dyn Iterator<Item = char>>;
    let mut lexer = Lexer {
      input: iterator.peekable(),
      ch: None,
    };
    lexer.read_char();
    lexer
  }
  
  fn read_char(&mut self) {
    self.ch = self.input.next().map(|c| c as u8);
  }
  
  fn skip_whitespace(&mut self) {
    while let Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') = self.ch {
      self.read_char();
    }
  }
  
  fn peek_char(&mut self) -> Option<u8> {
    self.input.peek().map(|c| *c as u8)
  }
  
  fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    
    if self.is_letter() {
      let token = self.read_identifier();
      self.read_char();
      return token;
    }
    
    if self.is_digit() {
      let token = self.read_int();
      return token;
    }
    
    let token = match self.ch {
      Some(b'=') => {
        if self.peek_char() == Some(b'=') {
          self.read_char();
          Token::new(TokenType::Eq)
        } else {
          Token::new(TokenType::Assign)
        }
      },
      Some(b'!') => {
        if self.peek_char() == Some(b'=') {
          self.read_char();
          Token::new(TokenType::NotEq)
        } else {
          Token::new(TokenType::Bang)
        }
      },
      Some(b';') => Token::new(TokenType::Semicolon),
      Some(b'(') => Token::new(TokenType::Lparen),
      Some(b')') => Token::new(TokenType::Rparen),
      Some(b'{') => Token::new(TokenType::Lbrace),
      Some(b'}') => Token::new(TokenType::Rbrace),
      Some(b',') => Token::new(TokenType::Comma),
      Some(b'+') => Token::new(TokenType::Plus),
      Some(_) => Token::new(TokenType::Illegal),
      None => Token::new(TokenType::Eof),
    };
    self.read_char();
    token
  }
  
  fn is_letter(&self) -> bool {
    match self.ch {
      Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'_') => true,
      _ => false,
    }
  }
  
  fn read_identifier(&mut self) -> Token {
    let mut identifier = String::new();
    while self.is_letter() {
      match self.ch {
        Some(letter) => identifier.push(letter as char),
        None => (),
      }
      self.read_char()
    }
    if &identifier == "let" {
      return Token::new(TokenType::Let)
    }
    
    Token::new(TokenType::Indent(identifier))
  }
  
  fn is_digit(&self) -> bool {
    match self.ch {
      Some(b'0'..=b'9') => true,
      _ => false,
    }
  }
  
  fn read_int(&mut self) -> Token {
    let mut integer = String::new();
    while self.is_digit() {
      match self.ch {
        Some(digit) => integer.push(digit as char),
        None => ()
      }
      self.read_char()
    }
    
    Token::new(TokenType::Int(integer.parse::<usize>().unwrap()))
  }
  
}

impl Iterator for Lexer<'_> {
  type Item = Token;
  
  fn next(&mut self) -> Option<Self::Item> {
    let token = self.next_token();
    match token.token_type() {
      TokenType::Eof => None,
      _ => Some(token),
    }
  }
}