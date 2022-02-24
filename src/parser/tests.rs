use super::*;
use crate::lexer::lex_tokens;
use crate::tokens::Tokens;
use nom::InputLength;
use ExpressionData::*;

fn tokens(input: &str) -> Vec<Token> {
    let (_, tokens) = lex_tokens(input, "").unwrap();
    tokens
}

#[test]
fn test_parse_program() {
    [
        ("class A {};", vec!["A"]),
        ("Class B{\nc:D;\n};\ncLASS E{};", vec!["B", "E"]),
        ("claSS\t E {\n  fG\t : \nHij ;\n };\nclass K {};", vec!["E", "K"]),
        ("class L{};class M{};class N{};", vec!["L", "M", "N"]),
        ("class\nO\n{\n\n\n}\n;", vec!["O"]),
    ]
    .iter()
    .for_each(|(input, classes)| {
        assert!(matches!(
            parse_program(&tokens(input)),
            Ok((rest, p)) if rest.input_len() == 0
                && p.classes.iter().map(|c| c.name.as_str()).collect::<Vec<_>>() == *classes,
        ))
    })
}

#[test]
fn test_bad_parse_program() {
    ["class A;", "class A {}", "class A {};;"]
        .iter()
        .for_each(|input| assert!(parse_program(&tokens(input)).is_err()))
}

#[test]
fn test_class_no_inherit() {
    [
        ("class A{}", "A"),
        ("Class B{c:D;}; ", "B"),
        ("claSS\t E {\n  fG\t : \nHij ;\n }; ", "E"),
        ("CLASS Klm { n : O ; p : Q ; } ", "Klm"),
        ("class\nR\n{\n\n\n}\n;", "R"),
    ]
    .iter()
    .for_each(|(input, name)| {
        assert!(matches!(
            class(Tokens::new(&tokens(input))),
            Ok((_, c)) if c.name == *name && c.super_class_name == "Object",
        ))
    })
}

#[test]
fn test_class_inherits() {
    [
        ("class A inherits A2{}", "A", "A2"),
        ("Class B Inherits B2{c:D;}; ", "B", "B2"),
        (
            "claSS\t E\tinherITS\nE2 {\n  fG\t : \nHij ;\n }; ",
            "E",
            "E2",
        ),
        ("CLASS Klm INHERITS K2 { n : O ; p : Q ; } ", "Klm", "K2"),
        ("cLASS\nR\niNHERITS\nR2\n{\n\n\n}\n;", "R", "R2"),
    ]
    .iter()
    .for_each(|(input, name, super_class)| {
        assert!(matches!(
            class(Tokens::new(&tokens(input))),
            Ok((_, c)) if c.name == *name && c.super_class_name == *super_class,
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
    .for_each(|input| assert!(class(Tokens::new(&tokens(input))).is_err()))
}

#[test]
fn test_unitialized_attribute() {
    [
        ("a:A", "a", "A"),
        ("b : B;", "b", "B"),
        ("c \t :\r\n C {", "c", "C"),
    ]
    .iter()
    .for_each(|(input, obj_id, type_id)| {
        assert!(matches!(
            attribute(Tokens::new(&tokens(input))),
            Ok((_, attr)) if attr.data == Attribute(
                obj_id.to_string(), type_id.to_string(), None),
        ))
    })
}

#[test]
fn test_initialized_attribute() {
    [
        ("d:D<-true", "d", "D", true),
        ("e \t: \r  E \n<- \t false\t;", "e", "E", false),
    ]
    .iter()
    .for_each(|(input, obj_id, type_id, value)| {
        assert!(matches!(
            attribute(Tokens::new(&tokens(input))),
            Ok((_, attr)) if matches!(
                &attr.data,
                Attribute(o, t, Some(e)) if o == *obj_id && t == *type_id
                    && e.data == BoolLiteral(*value),
            )
        ))
    })
}

#[test]
fn test_bad_attribute() {
    ["a::A", "b B", "c<-C", "d.D"].iter().for_each(|input| {
        assert!(attribute(Tokens::new(&tokens(input))).is_err())
    })
}

#[test]
fn test_parens_expression() {
    [
        ("(0)", 0),
        ("( 1);", 1),
        ("(\t 123 \n)\n", 123),
        ("((0))", 0),
        ("(\t(\n( 0\n )\t) );\n", 0),
    ]
    .iter()
    .for_each(|(input, literal)| {
        assert!(matches!(
            parens_expression(Tokens::new(&tokens(input))),
            Ok((_, e)) if e.data == IntLiteral(*literal),
        ))
    })
}

#[test]
fn test_bad_parens_expression() {
    ["()", ")(", "(()"].iter().for_each(|input| {
        assert!(parens_expression(Tokens::new(&tokens(input))).is_err())
    })
}
