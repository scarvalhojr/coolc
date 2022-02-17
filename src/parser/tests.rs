use super::*;

// TODO: test_parse_program

#[test]
fn test_class_no_inherit() {
    [
        ("class A{}", "A", ""),
        ("Class B{c:D;}; ", "B", "; "),
        ("claSS\t E {\n  fG\t : \nHij ;\n }; ", "E", "; "),
        ("CLASS Klm { n : O ; p : Q ; } ", "Klm", " "),
        ("class\nR\n{\n\n\n}\n;", "R", "\n;"),
    ]
    .iter()
    .for_each(|(input, name, rest)| {
        assert!(matches!(
            class(input),
            Ok((r, c)) if r == *rest && c.name == *name,
        ))
    })
}

#[test]
fn test_class_inherits() {
    [
        ("class A inherits A2{}", "A", "A2", ""),
        ("Class B Inherits B2{c:D;}; ", "B", "B2", "; "),
        (
            "claSS\t E\tinherITS\nE2 {\n  fG\t : \nHij ;\n }; ",
            "E",
            "E2",
            "; ",
        ),
        (
            "CLASS Klm INHERITS K2 { n : O ; p : Q ; } ",
            "Klm",
            "K2",
            " ",
        ),
        ("cLASS\nR\niNHERITS\nR2\n{\n\n\n}\n;", "R", "R2", "\n;"),
    ]
    .iter()
    .for_each(|(input, name, super_class, rest)| {
        assert!(matches!(
            class(input),
            Ok((r, c)) if r == *rest && c.name == *name
                && c.super_class_name == *super_class,
        ))
    })
}

#[test]
fn test_bad_class() {
    [
        "class A",
        "class A;",
        "class A {",
        "class A }",
        "class a {}",
        "class A inherits {}",
        "class A inherits {}",
        "class A inherits a {}",
        "class A inherits a {",
        "class A inherits a }",
        "class A inherits a;",
    ]
    .iter()
    .for_each(|input| assert!(class(input).is_err()))
}

#[test]
fn test_unitialized_attribute() {
    [
        ("a:A", "a", "A", ""),
        ("b : B;", "b", "B", ";"),
        ("c \t :\r\n C {", "c", "C", " {"),
    ]
    .iter()
    .for_each(|(input, obj_id, type_id, rest)| {
        assert_eq!(
            attribute(input),
            Ok((
                *rest,
                Attribute(obj_id.to_string(), type_id.to_string(), None)
            ))
        )
    })
}

#[test]
fn test_initialized_attribute() {
    [
        ("d:D<-true", "d", "D", ""),
        ("e \t: \r  E \n<- \t false\t;", "e", "E", "\t;"),
    ]
    .iter()
    .for_each(|(input, obj_id, type_id, rest)| {
        assert!(matches!(
            attribute(input),
            Ok((r, Attribute(o, t, Some(_))))
                if r == *rest && o == *obj_id && t == *type_id
        ))
    })
}

#[test]
fn test_bad_attribute() {
    ["a::A", "b B", "c<-C", "d.D"]
        .iter()
        .for_each(|input| assert!(attribute(input).is_err()))
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
        assert_eq!(object_id(input), Ok((*rest, id.to_string())))
    })
}

#[test]
fn test_bad_object_id() {
    [
        "", "A", "_b", "0c", "_1", ":d", "$e", "@f", "#g", "£h", "%i", "&j",
        "*h", "~ijk", "?l", "Mnop",
    ]
    .iter()
    .for_each(|input| assert!(object_id(input).is_err()))
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
        assert_eq!(type_id(input), Ok((*rest, id.to_string())))
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
fn test_arithmetic_expression() {
    [
        (
            "1+2",
            Expression::new_arith_expression(
                IntegerLiteral("1".to_string()),
                '+',
                IntegerLiteral("2".to_string()),
            ),
            "",
        ),
        (
            "2-1;",
            Expression::new_arith_expression(
                IntegerLiteral("2".to_string()),
                '-',
                IntegerLiteral("1".to_string()),
            ),
            ";",
        ),
        (
            "3\t * \n4\n;",
            Expression::new_arith_expression(
                IntegerLiteral("3".to_string()),
                '*',
                IntegerLiteral("4".to_string()),
            ),
            "\n;",
        ),
        (
            "10\n\n/ \t5 ",
            Expression::new_arith_expression(
                IntegerLiteral("10".to_string()),
                '/',
                IntegerLiteral("5".to_string()),
            ),
            " ",
        ),
        (
            "2 + 3 * 5",
            Expression::new_arith_expression(
                IntegerLiteral("2".to_string()),
                '+',
                Expression::new_arith_expression(
                    IntegerLiteral("3".to_string()),
                    '*',
                    IntegerLiteral("5".to_string()),
                ),
            ),
            "",
        ),
        (
            "2 + 3 * 5 + 4 +",
            Expression::new_arith_expression(
                Expression::new_arith_expression(
                    IntegerLiteral("2".to_string()),
                    '+',
                    Expression::new_arith_expression(
                        IntegerLiteral("3".to_string()),
                        '*',
                        IntegerLiteral("5".to_string()),
                    ),
                ),
                '+',
                IntegerLiteral("4".to_string()),
            ),
            " +",
        ),
        (
            "2 + 3 * 5 - 8 / 4 * - 1",
            Expression::new_arith_expression(
                Expression::new_arith_expression(
                    IntegerLiteral("2".to_string()),
                    '+',
                    Expression::new_arith_expression(
                        IntegerLiteral("3".to_string()),
                        '*',
                        IntegerLiteral("5".to_string()),
                    ),
                ),
                '-',
                Expression::new_arith_expression(
                    IntegerLiteral("8".to_string()),
                    '/',
                    IntegerLiteral("4".to_string()),
                ),
            ),
            " * - 1",
        ),
    ]
    .iter()
    .for_each(|(input, expr, rest)| {
        assert!(matches!(
            expression(input),
            Ok((r, e)) if r == *rest && &e == expr,
        ))
    });
}

#[test]
fn test_parens_expression() {
    [
        ("(0)", "0", ""),
        ("( 1);", "1", ";"),
        ("(\t 123 \n)\n", "123", "\n"),
        ("((0))", "0", ""),
        ("(\t(\n( 0\n )\t) );\n", "0", ";\n"),
    ]
    .iter()
    .for_each(|(input, literal, rest)| {
        assert!(matches!(
            parens_expression(input),
            Ok((r, IntegerLiteral(i))) if r == *rest && i == *literal,
        ))
    });

    ["()", ")(", "(()"]
        .iter()
        .for_each(|input| assert!(integer_literal(input).is_err()));
}

#[test]
fn test_integer_literal() {
    [
        ("0", "0", ""),
        ("01;", "01", ";"),
        ("9999999999 ", "9999999999", " "),
    ]
    .iter()
    .for_each(|(input, literal, rest)| {
        assert!(matches!(
            integer_literal(input),
            Ok((r, IntegerLiteral(i))) if r == *rest && i == *literal,
        ))
    });

    [" 0", "+1", "-1", "a1"]
        .iter()
        .for_each(|input| assert!(integer_literal(input).is_err()));
}

#[test]
fn test_string_literal() {
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
            string_literal(input),
            Ok((r, StringLiteral(s))) if r == *rest && s == *string,
        ))
    });

    ["", " ", "\"", "\"abc"]
        .iter()
        .for_each(|input| assert!(string_literal(input).is_err()));
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
    });

    // TODO: check null character is rejected
    ["", r"\n", r"\t", r"\\"]
        .iter()
        .for_each(|input| assert!(unescaped_fragment(input).is_err()));
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
    });

    // TODO: test null character is rejected
    ["", r" ", r"n"]
        .iter()
        .for_each(|input| assert!(escaped_char(input).is_err()));
}

#[test]
fn test_false_literal() {
    [
        ("false", "false", ""),
        ("fAlSe;", "fAlSe", ";"),
        ("fALSE ", "fALSE", " "),
    ]
    .iter()
    .for_each(|(input, literal, rest)| {
        assert_eq!(false_literal(input), Ok((*rest, *literal)))
    });

    ["False", "FaLsE", "FALSE"]
        .iter()
        .for_each(|input| assert!(false_literal(input).is_err()));
}

#[test]
fn test_true_literal() {
    [
        ("true", "true", ""),
        ("tRuE or", "tRuE", " or"),
        ("tRUE,", "tRUE", ","),
    ]
    .iter()
    .for_each(|(input, literal, rest)| {
        assert_eq!(true_literal(input), Ok((*rest, *literal)))
    });

    ["True", "TrUe", "TRUE"]
        .iter()
        .for_each(|input| assert!(true_literal(input).is_err()));
}
