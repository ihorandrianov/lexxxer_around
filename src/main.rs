pub mod token;
pub mod lexer;
pub mod repl;



fn main() {
    let mut repl = repl::Repl::new();
    repl.run();
}
