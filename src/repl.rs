use std::io::{BufReader, BufWriter, Stdin, Stdout, stdin, stdout, Write, BufRead};
use crate::token::TokenType;
use crate::lexer::Lexer;

pub struct Repl {
    prompt: String,
    reader: BufReader<Stdin>,
    writer: BufWriter<Stdout>,
}

impl Repl {
    pub fn new() -> Self {
        let prompt = ">> ".to_string();
        let reader = BufReader::new(stdin());
        let writer = BufWriter::new(stdout());
        Self {
            prompt,
            reader,
            writer,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.writer.write_all(self.prompt.as_bytes()).unwrap();
            self.writer.flush().unwrap();
            let mut line = String::new();
            self.reader.read_line(&mut line).unwrap();
            let mut lexer = Lexer::new(&line);
            loop {
                let token = lexer.next_token();
                if token.token_type() == &TokenType::Eof {
                    break;
                }
                println!("{:?}", token);
            }
        }
    }
}