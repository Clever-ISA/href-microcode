use std::io::Read;

mod lex;
mod parse;
fn main() {
    let fname = std::env::args().nth(1).expect("Expected a file path");

    let file = std::fs::File::open(&fname).unwrap_or_else(|e| {
        eprintln!("Could not open {fname}: {e}");
        std::process::exit(1)
    });

    let chars = utf::decode_utf8(file.bytes().map(|r| {
        r.unwrap_or_else(|e| {
            eprintln!("Could not read from {fname}: {e}");
            std::process::exit(1)
        })
    }))
    .map(|r| {
        r.unwrap_or_else(|_| {
            eprintln!("Could not read from {fname}: Not valid UTF-8");
            std::process::exit(1)
        })
    });

    let toks = lex::Lexer::new(chars).collect::<Vec<_>>();
    println!("{:?}", toks);
}
