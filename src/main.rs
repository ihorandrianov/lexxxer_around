pub mod token;
pub mod lexer;



fn main() {
    let lexer = lexer::Lexer::new("let x = 69; let y = 420;");
    for token in lexer {
        println!("{:?}", token);
    }
   
}
