use super::*;

#[test]
fn test_lex_tokens() {
    [
        ("class A {};", 5),
        (
            " \t(* block comment (* nested block\n*)\n*)\
            -- \t long line comment \r\n\
            class A -- comment\n\
            inherits (* comment *) B\n\
            {};\n--line comment\t\n(* block\n\
            comment *)\n\n\n",
            7,
        ),
    ]
    .iter()
    .for_each(|(input, count)| {
        assert!(matches!(
            lex_tokens(input),
            Ok((rest, tokens)) if rest.is_empty() && tokens.len() == *count,
        ))
    })
}

#[test]
fn test_line_comment() {
    [
        ("--", ""),
        ("-- ", ""),
        ("-- comment\n", "\n"),
        ("-- \t long -- comment \r\nclass", "\r\nclass"),
    ]
    .iter()
    .for_each(|(input, rest)| {
        assert!(matches!(
            line_comment(input),
            Ok((r, ())) if r == *rest,
        ))
    })
}

#[test]
fn test_block_comment() {
    [
        ("(**)", ""),
        ("(* *) ", " "),
        ("(*(**)*)\n", "\n"),
        ("(* \t *\n) \r*)\na", "\na"),
        ("(**(*)*)*)", ""),
        (
            "(**\n )\n))* **\n**(**\n))\n))*)\n\n(* )* )*\n**)*)*)",
            "*)",
        ),
    ]
    .iter()
    .for_each(|(input, rest)| {
        assert!(matches!(
            block_comment(input),
            Ok((r, ())) if r == *rest,
        ))
    })
}

#[test]
fn test_reserved_word() {
    [
        ("CLASS A", Token::Class, " A"),
        ("inherits B", Token::Inherits, " B"),
        ("iF t", Token::If, " t"),
        ("Then {", Token::Then, " {"),
        ("eLsE 1", Token::Else, " 1"),
        ("FI;", Token::Fi, ";"),
        ("let a", Token::Let, " a"),
        ("In {", Token::In, " {"),
        ("WHILE t", Token::While, " t"),
        ("loop {", Token::Loop, " {"),
        ("Pool;", Token::Pool, ";"),
        ("cAse a:", Token::Case, " a:"),
        ("OF x", Token::Of, " x"),
        ("eSAc;", Token::Esac, ";"),
        ("NEW o", Token::New, " o"),
        ("IsVoid x", Token::IsVoid, " x"),
        ("not f", Token::Not, " f"),
    ]
    .iter()
    .for_each(|(input, token, rest)| {
        assert!(matches!(
            reserved_word(input),
            Ok((r, t)) if r == *rest && t == *token,
        ))
    })
}

#[test]
fn test_bad_reserved_word() {
    [
        "classe", "inh", "if1", "the", "else_", "fii", "is_void", "not_",
    ]
    .iter()
    .for_each(|input| assert!(reserved_word(input).is_err()))
}

#[test]
fn test_symbol() {
    [
        ("@Blah", Token::At, "Blah"),
        ("<- 0", Token::Assign, " 0"),
        ("=> {", Token::DoubleArrow, " {"),
        ("{ let", Token::OpenBraces, " let"),
        ("};", Token::CloseBraces, ";"),
        ("(abc", Token::OpenParens, "abc"),
        (");", Token::CloseParens, ";"),
        (".abc()", Token::Dot, "abc()"),
        (", 1", Token::Comma, " 1"),
        (": A", Token::Colon, " A"),
        (";\n", Token::SemiColon, "\n"),
        ("= 2", Token::Equals, " 2"),
        ("+ 9", Token::Plus, " 9"),
        ("- 5", Token::Minus, " 5"),
        ("* 4", Token::Multiply, " 4"),
        ("/ 3", Token::Divide, " 3"),
        ("~ 8", Token::Negative, " 8"),
        ("<= 10", Token::LessThanOrEquals, " 10"),
        ("< 11", Token::LessThan, " 11"),
    ]
    .iter()
    .for_each(|(input, token, rest)| {
        assert!(matches!(
            symbol(input),
            Ok((r, t)) if r == *rest && t == *token,
        ))
    })
}

#[test]
fn test_bad_symbol() {
    [
        " ", "a", "1", "!", "#", "$", "%", "^", "&", "_", "[", "]", "'", "\\",
        "|", "`", ">", "?",
    ]
    .iter()
    .for_each(|input| assert!(symbol(input).is_err()))
}

#[test]
fn test_int_literal() {
    [
        ("0", 0, ""),
        ("01;", 1, ";"),
        ("2_", 2, "_"),
        ("2147483647 ", i32::MAX, " "),
    ]
    .iter()
    .for_each(|(input, value, rest)| {
        assert!(matches!(
            int_literal(input),
            Ok((r, Token::IntLiteral(v))) if r == *rest && v == *value,
        ))
    })
}

#[test]
fn test_bad_int_literal() {
    [" 0", "+1", "-1", "a1", "2147483648"]
        .iter()
        .for_each(|input| assert!(int_literal(input).is_err()));
}

#[test]
fn test_str_literal() {
    [
        ("\"\"", "", ""),
        ("\" \"", " ", ""),
        ("\"a\"", "a", ""),
        ("\"a b c\";", "a b c", ";"),
        ("\"a\\tb c\\nd\\\\e\".concat", "a\tb c\nd\\e", ".concat"),
    ]
    .iter()
    .for_each(|(input, string, rest)| {
        assert!(matches!(
            str_literal(input),
            Ok((r, Token::StrLiteral(s))) if r == *rest && s == *string,
        ))
    })
}

#[test]
fn test_bad_str_literal() {
    ["", " ", "\"", "\"abc", "abc"]
        .iter()
        .for_each(|input| assert!(str_literal(input).is_err()))
}

#[test]
fn test_unescaped_fragment() {
    [
        (" ", " ", ""),
        ("abc", "abc", ""),
        ("a\tb", "a\tb", ""),
        (r"a\tb", "a", r"\tb"),
        (r"a\\b", "a", r"\\b"),
        (r"a\\ ", "a", r"\\ "),
    ]
    .iter()
    .for_each(|(input, fragment, rest)| {
        assert_eq!(unescaped_fragment(input), Ok((*rest, *fragment)),)
    })
}

#[test]
fn test_bad_unescaped_fragment() {
    // TODO: check null character is rejected
    ["", r"\n", r"\t", r"\\"]
        .iter()
        .for_each(|input| assert!(unescaped_fragment(input).is_err()))
}

#[test]
fn test_escaped_char() {
    [
        (r"\n", '\n', ""),
        (r"\t ", '\t', " "),
        (r"\b\", '\u{08}', r"\"),
        (r"\f;", '\u{0C}', ";"),
        (r"\ \ ", ' ', r"\ "),
        (r"\\a", '\\', "a"),
        (r"\xyz", 'x', "yz"),
    ]
    .iter()
    .for_each(|(input, ch, rest)| {
        assert_eq!(escaped_char(input), Ok((*rest, *ch)),)
    })
}

#[test]
fn test_bad_escaped_char() {
    // TODO: test null character is rejected
    ["", r" ", r"n"]
        .iter()
        .for_each(|input| assert!(escaped_char(input).is_err()))
}

#[test]
fn test_false_literal() {
    [("false", ""), ("fAlSe;", ";"), ("fALSE ", " ")]
        .iter()
        .for_each(|(input, rest)| {
            assert_eq!(
                false_literal(input),
                Ok((*rest, Token::BoolLiteral(false)))
            )
        })
}

#[test]
fn test_bad_false_literal() {
    ["False", "FaLsE", "FALSE", "false_", "false0"]
        .iter()
        .for_each(|input| assert!(false_literal(input).is_err()))
}

#[test]
fn test_true_literal() {
    [("true", ""), ("tRuE or", " or"), ("tRUE,", ",")]
        .iter()
        .for_each(|(input, rest)| {
            assert_eq!(
                true_literal(input),
                Ok((*rest, Token::BoolLiteral(true)))
            )
        })
}

#[test]
fn test_bad_true_literal() {
    ["True", "TrUe", "TRUE", "true_", "true1"]
        .iter()
        .for_each(|input| assert!(true_literal(input).is_err()))
}

#[test]
fn test_type_id() {
    [
        ("A", "A", ""),
        ("B_;", "B_", ";"),
        ("C0{", "C0", "{"),
        ("DE inherits", "DE", " inherits"),
        ("Fghij.abc", "Fghij", ".abc"),
        ("K123<- 0", "K123", "<- 0"),
        ("L4m5N6_=> a", "L4m5N6_", "=> a"),
    ]
    .iter()
    .for_each(|(input, id, rest)| {
        assert_eq!(type_id(input), Ok((*rest, Token::TypeId(id.to_string()))))
    })
}

#[test]
fn test_bad_type_id() {
    [
        "", "a", "_B", "0C", "_1", ":D", "$E", "@F", "#G", "£H", "%I", "&J",
        "*H", "~IJK", "?L", "mNOP",
    ]
    .iter()
    .for_each(|input| assert!(type_id(input).is_err()))
}

#[test]
fn test_object_id() {
    [
        ("a", "a", ""),
        ("b_ ", "b_", " "),
        ("c0;", "c0", ";"),
        ("de <- ", "de", " <- "),
        ("fGHIJ}", "fGHIJ", "}"),
        ("k123 --", "k123", " --"),
        ("l4M5n6_> ", "l4M5n6_", "> "),
    ]
    .iter()
    .for_each(|(input, id, rest)| {
        assert_eq!(ident(input), Ok((*rest, Token::Ident(id.to_string()))))
    })
}

#[test]
fn test_bad_object_id() {
    [
        "", "A", "_b", "0c", "_1", ":d", "$e", "@f", "#g", "£h", "%i", "&j",
        "*h", "~ijk", "?l", "Mnop",
    ]
    .iter()
    .for_each(|input| assert!(ident(input).is_err()))
}
