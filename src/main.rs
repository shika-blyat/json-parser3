mod ast;
mod error;
mod lexer;
mod parser;
mod tokens;

use lexer::Lexer;
use parser::Parser;
fn main() {
    let lexer = Lexer::new(
        r#"
    {
        "abc": 15,
        "foo": {
            "def": [
                12,
                true,
                null
            ]
        }
    }    
"#,
    );
    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);
    let mut parser = Parser::new(tokens.unwrap());
    println!("{:#?}", parser.json());
}
