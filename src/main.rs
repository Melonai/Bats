mod lexer;

use anyhow::Result;
use lexer::Lexer;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    let mut file = File::open("examples/test.bs")?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lexer = Lexer::new(&content);
    for token in lexer {
        println!("{:?}", token);
    }

    Ok(())
}
