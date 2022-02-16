use clap::{app_from_crate, arg, crate_description, crate_version};
use coolc::parser::parse_program;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args = app_from_crate!()
        .args(&[
            arg!(<SOURCE> "Cool source file"),
            arg!(-p --print "Print parse tree").long("print-parse-tree"),
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

    let parse_tree = match parse_program(&source) {
        Ok((_unparsed, tree)) => tree,
        Err(_unparsed) => {
            eprintln!("Program failed to compile.");
            exit(1);
        }
    };

    if args.is_present("print") {
        println!("{}", parse_tree.print(filename));
    } else {
        eprintln!("Program compiled successfully.");
    }

    exit(0);
}
