use coolc::lexer::lex_tokens;
use coolc::parser::parse_program;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

#[test]
fn test_files() {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/resources");

    for entry in read_dir(dir).unwrap() {
        let filename = entry.unwrap().path();
        if filename.extension().unwrap() == "ptree" {
            let source_filename = filename.with_extension("cool");
            let source_code = read_to_string(&source_filename).unwrap();
            let expected = read_to_string(&filename).unwrap();
            let (_, tokens) = lex_tokens(
                &source_code,
                source_filename.file_name().unwrap().to_str().unwrap(),
            )
            .unwrap();
            let (_, parse_tree) = parse_program(&tokens).unwrap();
            parse_tree
                .format()
                .to_string()
                .lines()
                .zip(expected.lines().zip(1..))
                .for_each(|(produced_line, (expected_line, line_num))| {
                    assert_eq!(
                        expected_line,
                        produced_line,
                        "Mismatch at line {line_num}, source: {}",
                        source_filename.display()
                    )
                });
        }
    }
}
