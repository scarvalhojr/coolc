use coolc::lexer::lex_tokens;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

#[test]
fn test_files() {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/resources");

    for entry in read_dir(dir).unwrap() {
        let filename = entry.unwrap().path();
        if filename.extension().unwrap() == "tokens" {
            let source_filename = filename.with_extension("cool");
            let source_code = read_to_string(&source_filename).unwrap();
            let expected = read_to_string(&filename).unwrap();
            let (_, tokens) =
                lex_tokens(&source_code, source_filename.to_str().unwrap())
                    .unwrap();
            tokens.iter().zip(expected.lines().skip(1)).for_each(
                |(token, expected_line)| {
                    assert_eq!(
                        expected_line,
                        token.to_string(),
                        "Source: {}",
                        source_filename.display()
                    )
                },
            );
        }
    }
}
