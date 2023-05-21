mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn render_error(code: String, name: String, file: String, line: usize, position: usize) {
    println!("\x1b[34m{file}\x1b[0m @ line \x1b[35m{line}\x1b[0m");
    let out = format!("\x1b[33m  \\________.-->\x1b[0m {code}");
    print!("{out}\n");
    for _ in 0..position + out.len() - 14 {
        print!(" ");
    }
    println!("^");
}

fn main() {
    let code = std::env::args().nth(1).unwrap();
    let mut lex = Lexer::new();

    lex.feed_file((code, "file.mfl".to_string()));
    if let Err(err) = lex.lex() {
        render_error(lex.input.get(err.3).unwrap().0.clone()[0 .. err.1+1].to_string(), err.0, lex.input.get(err.3).unwrap().1.clone(), err.2, err.1);
        return;
    }

    let mut parser = Parser::new(lex);
    parser.start_parsing().unwrap();
}