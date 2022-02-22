use super::*;

fn span(input: &str) -> Span {
    Span::new_extra(input, "")
}

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
            lex_tokens(span(input)),
            Ok((r, t)) if r.to_string().is_empty() && t.len() == *count,
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
            line_comment(span(input)),
            Ok((r, ())) if r.to_string() == *rest,
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
            block_comment(span(input)),
            Ok((r, ())) if r.to_string() == *rest,
        ))
    })
}

#[test]
fn test_reserved_word() {
    [
        ("CLASS A", TokenKind::Class, " A"),
        ("inherits B", TokenKind::Inherits, " B"),
        ("iF t", TokenKind::If, " t"),
        ("Then {", TokenKind::Then, " {"),
        ("eLsE 1", TokenKind::Else, " 1"),
        ("FI;", TokenKind::Fi, ";"),
        ("let a", TokenKind::Let, " a"),
        ("In {", TokenKind::In, " {"),
        ("WHILE t", TokenKind::While, " t"),
        ("loop {", TokenKind::Loop, " {"),
        ("Pool;", TokenKind::Pool, ";"),
        ("cAse a:", TokenKind::Case, " a:"),
        ("OF x", TokenKind::Of, " x"),
        ("eSAc;", TokenKind::Esac, ";"),
        ("NEW o", TokenKind::New, " o"),
        ("IsVoid x", TokenKind::IsVoid, " x"),
        ("not f", TokenKind::Not, " f"),
    ]
    .iter()
    .for_each(|(input, token_kind, rest)| {
        assert!(matches!(
            reserved_word(span(input)),
            Ok((r, t)) if r.to_string() == *rest && t.kind == *token_kind,
        ))
    })
}

#[test]
fn test_bad_reserved_word() {
    [
        "classe", "inh", "if1", "the", "else_", "fii", "is_void", "not_",
    ]
    .iter()
    .for_each(|input| assert!(reserved_word(span(input)).is_err()))
}

#[test]
fn test_symbol() {
    [
        ("@Blah", TokenKind::At, "Blah"),
        ("<- 0", TokenKind::Assign, " 0"),
        ("=> {", TokenKind::DoubleArrow, " {"),
        ("{ let", TokenKind::OpenBraces, " let"),
        ("};", TokenKind::CloseBraces, ";"),
        ("(abc", TokenKind::OpenParens, "abc"),
        (");", TokenKind::CloseParens, ";"),
        (".abc()", TokenKind::Dot, "abc()"),
        (", 1", TokenKind::Comma, " 1"),
        (": A", TokenKind::Colon, " A"),
        (";\n", TokenKind::SemiColon, "\n"),
        ("= 2", TokenKind::Equals, " 2"),
        ("+ 9", TokenKind::Plus, " 9"),
        ("- 5", TokenKind::Minus, " 5"),
        ("* 4", TokenKind::Multiply, " 4"),
        ("/ 3", TokenKind::Divide, " 3"),
        ("~ 8", TokenKind::Negative, " 8"),
        ("<= 10", TokenKind::LessThanOrEquals, " 10"),
        ("< 11", TokenKind::LessThan, " 11"),
    ]
    .iter()
    .for_each(|(input, token_kind, rest)| {
        assert!(matches!(
            symbol(span(input)),
            Ok((r, t)) if r.to_string() == *rest && t.kind == *token_kind,
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
    .for_each(|input| assert!(symbol(span(input)).is_err()))
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
    .for_each(|(input, integer, rest)| {
        assert!(matches!(
            int_literal(span(input)),
            Ok((r, t)) if r.to_string() == *rest
                && t.kind == TokenKind::IntLiteral(*integer),
        ))
    })
}

#[test]
fn test_bad_int_literal() {
    [" 0", "+1", "-1", "a1", "2147483648"]
        .iter()
        .for_each(|input| assert!(int_literal(span(input)).is_err()));
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
            str_literal(span(input)),
            Ok((r, t)) if r.to_string() == *rest
                && t.kind == TokenKind::StrLiteral(string.to_string()),
        ))
    })
}

#[test]
fn test_bad_str_literal() {
    ["", " ", "\"", "\"abc", "abc"]
        .iter()
        .for_each(|input| assert!(str_literal(span(input)).is_err()))
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
        assert!(matches!(
            unescaped_fragment(span(input)),
            Ok((r, f)) if r.to_string() == *rest && f.to_string() == *fragment,
        ))
    })
}

#[test]
fn test_bad_unescaped_fragment() {
    // TODO: check null character is rejected
    ["", r"\n", r"\t", r"\\"]
        .iter()
        .for_each(|input| assert!(unescaped_fragment(span(input)).is_err()))
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
        assert!(matches!(
            escaped_char(span(input)),
            Ok((r, c)) if r.to_string() == *rest && c == *ch,
        ))
    })
}

#[test]
fn test_bad_escaped_char() {
    // TODO: test null character is rejected
    ["", r" ", r"n"]
        .iter()
        .for_each(|input| assert!(escaped_char(span(input)).is_err()))
}

#[test]
fn test_false_literal() {
    [("false", ""), ("fAlSe;", ";"), ("fALSE ", " ")]
        .iter()
        .for_each(|(input, rest)| {
            assert!(matches!(
                false_literal(span(input)),
                Ok((r, t)) if r.to_string() == *rest
                    && t.kind == TokenKind::BoolLiteral(false),
            ))
        })
}

#[test]
fn test_bad_false_literal() {
    ["False", "FaLsE", "FALSE", "false_", "false0"]
        .iter()
        .for_each(|input| assert!(false_literal(span(input)).is_err()))
}

#[test]
fn test_true_literal() {
    [("true", ""), ("tRuE or", " or"), ("tRUE,", ",")]
        .iter()
        .for_each(|(input, rest)| {
            assert!(matches!(
                true_literal(span(input)),
                Ok((r, t)) if r.to_string() == *rest
                    && t.kind == TokenKind::BoolLiteral(true),
            ))
        })
}

#[test]
fn test_bad_true_literal() {
    ["True", "TrUe", "TRUE", "true_", "true1"]
        .iter()
        .for_each(|input| assert!(true_literal(span(input)).is_err()))
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
        assert!(matches!(
            type_id(span(input)),
            Ok((r, t)) if r.to_string() == *rest
                && t.kind == TokenKind::TypeId(id.to_string()),
        ))
    })
}

#[test]
fn test_bad_type_id() {
    [
        "", "a", "_B", "0C", "_1", ":D", "$E", "@F", "#G", "£H", "%I", "&J",
        "*H", "~IJK", "?L", "mNOP",
    ]
    .iter()
    .for_each(|input| assert!(type_id(span(input)).is_err()))
}

#[test]
fn test_ident() {
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
        assert!(matches!(
            ident(span(input)),
            Ok((r, t)) if r.to_string() == *rest
                && t.kind == TokenKind::Ident(id.to_string()),
        ))
    })
}

#[test]
fn test_bad_ident() {
    [
        "", "A", "_b", "0c", "_1", ":d", "$e", "@f", "#g", "£h", "%i", "&j",
        "*h", "~ijk", "?l", "Mnop",
    ]
    .iter()
    .for_each(|input| assert!(ident(span(input)).is_err()))
}
