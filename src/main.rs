use clap::{arg, command, crate_description, crate_version};
use coolc::lexer::lex_tokens;
use coolc::parser::parse_program;
use coolc::tokens::Span;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args = command!()
        .arg_required_else_help(true)
        .args(&[
            arg!(<SOURCE> "Cool source file"),
            arg!(-l --lex "Run lexer only, print tokens and stop")
                .conflicts_with("parse"),
            arg!(-p --parse "Run lexer and parser, print parse tree and stop"),
        ])
        .get_matches();

    eprintln!("{} - {}", crate_description!(), crate_version!());

    let filename = args.value_of("SOURCE").unwrap();
    let source = match read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read source file: {err}.");
            exit(1);
        }
    };

    let tokens = match lex_tokens(Span::new_extra(&source, filename)) {
        Ok((_, tok)) => tok,
        Err(err) => {
            eprintln!("Scanner error: {err}.");
            exit(2);
        }
    };

    if args.is_present("lex") {
        // Print tokens...
        println!("#name \"{}\"", filename);
        for token in tokens.iter() {
            println!("{token}");
        }
        // ... and stop
        exit(0);
    }

    let parse_tree = match parse_program(&source) {
        Ok((_unparsed, tree)) => tree,
        Err(err) => {
            eprintln!("Parser error: {err}.");
            exit(3);
        }
    };

    if args.is_present("parse") {
        // Print parse tree
        println!("{}", parse_tree.print(filename));
    } else {
        eprintln!("Program compiled successfully.");
    }

    exit(0);
}
