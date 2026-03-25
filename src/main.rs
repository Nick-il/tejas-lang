use lexer::*;
use sourcer::*;
use std::path::Path;

fn main() {
    let mut sm = SourceManager::new();
    let (_, text) = load_from_file(&mut sm, Path::new("tejas_tests/test1.tej"))
        .expect("Error while loading 'tests/test1.tej' file.'");

    let mut lexer = Lexer::new(text);
    while !lexer.is_finished() {
        let res = lexer
            .scan_once()
            .unwrap_or_else(|e| panic!("{}", e.to_string()));
        if let Some(tok) = res {
            println!(
                "{} lexeme : '{}'",
                tok,
                text.slice_span(tok.span()).unwrap()
            );
        }
    }
}
