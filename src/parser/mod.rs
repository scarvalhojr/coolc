//! Cool source code parsing functions.

use crate::ptree::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::{
    alphanumeric1, anychar, char, digit1, multispace0, multispace1,
};
use nom::combinator::{map, opt, recognize, value};
use nom::multi::{fold_many0, many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use Expression::*;
use Feature::*;

#[cfg(test)]
mod tests;

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    map(
        many1(preceded(
            multispace0,
            terminated(class, tuple((multispace0, tag(";"), multispace0))),
        )),
        Program::new,
    )(input)
}

fn class(input: &str) -> IResult<&str, Class> {
    map(
        tuple((
            preceded(
                tuple((multispace0, tag_no_case("class"), multispace1)),
                type_id,
            ),
            opt(preceded(
                tuple((multispace1, tag_no_case("inherits"), multispace1)),
                type_id,
            )),
            delimited(
                tuple((multispace0, tag("{"), multispace0)),
                many0(terminated(
                    feature,
                    tuple((multispace0, tag(";"), multispace0)),
                )),
                pair(multispace0, tag("}")),
            ),
        )),
        |(name, super_name, features)| Class::new(name, super_name, features),
    )(input)
}

fn feature(input: &str) -> IResult<&str, Feature> {
    // TODO: methods
    attribute(input)
}

fn attribute(input: &str) -> IResult<&str, Feature> {
    map(
        tuple((
            terminated(object_id, tuple((multispace0, tag(":"), multispace0))),
            type_id,
            opt(preceded(
                tuple((multispace0, tag("<-"), multispace0)),
                expression,
            )),
        )),
        |(obj_id, type_id, expr)| Attribute(obj_id, type_id, expr),
    )(input)
}

fn object_id(input: &str) -> IResult<&str, ObjectId> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_lowercase()),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        str::to_string,
    )(input)
}

fn type_id(input: &str) -> IResult<&str, TypeId> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_uppercase()),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        str::to_string,
    )(input)
}

fn expression(input: &str) -> IResult<&str, Expression> {
    let (input, operand1) = term(input)?;
    let (input, operations) = many0(tuple((
        delimited(multispace0, alt((char('+'), char('-'))), multispace0),
        term,
    )))(input)?;
    Ok((input, build_arith_expr(operand1, operations)))
}

fn build_arith_expr(
    first_operand: Expression,
    operations: Vec<(char, Expression)>,
) -> Expression {
    operations
        .into_iter()
        .fold(first_operand, |acc, (operator, next_expr)| {
            Expression::new_arith_expression(acc, operator, next_expr)
        })
}

fn term(input: &str) -> IResult<&str, Expression> {
    let (input, operand1) = factor(input)?;
    let (input, operations) = many0(tuple((
        delimited(multispace0, alt((char('*'), char('/'))), multispace0),
        factor,
    )))(input)?;
    Ok((input, build_arith_expr(operand1, operations)))
}

fn factor(input: &str) -> IResult<&str, Expression> {
    alt((
        // TODO: lots more here...
        parens_expression,
        integer_literal,
        string_literal,
        boolean_literal,
    ))(input)
}

fn parens_expression(input: &str) -> IResult<&str, Expression> {
    delimited(
        pair(tag("("), multispace0),
        expression,
        pair(multispace0, tag(")")),
    )(input)
}

fn integer_literal(input: &str) -> IResult<&str, Expression> {
    map(digit1, |parsed: &str| IntegerLiteral(parsed.to_string()))(input)
}

fn string_literal(input: &str) -> IResult<&str, Expression> {
    let build_string =
        fold_many0(string_fragment, String::new, |mut string, fragment| {
            match fragment {
                StringFragment::UnescapedFragment(s) => string.push_str(s),
                StringFragment::EscapedChar(c) => string.push(c),
            }
            string
        });

    map(delimited(char('"'), build_string, char('"')), StringLiteral)(input)
}

enum StringFragment<'a> {
    UnescapedFragment(&'a str),
    EscapedChar(char),
}

fn string_fragment(input: &str) -> IResult<&str, StringFragment> {
    alt((
        map(unescaped_fragment, StringFragment::UnescapedFragment),
        map(escaped_char, StringFragment::EscapedChar),
    ))(input)
}

fn unescaped_fragment(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c != '\\' && c != '"')(input)
}

fn escaped_char(input: &str) -> IResult<&str, char> {
    preceded(
        char('\\'),
        alt((
            value('\n', char('n')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            // TODO: reject null character \u{00}
            anychar,
        )),
    )(input)
}

fn boolean_literal(input: &str) -> IResult<&str, Expression> {
    map(alt((false_literal, true_literal)), |parsed: &str| {
        BooleanLiteral(parsed.to_string())
    })(input)
}

fn false_literal(input: &str) -> IResult<&str, &str> {
    recognize(pair(char('f'), tag_no_case("alse")))(input)
}

fn true_literal(input: &str) -> IResult<&str, &str> {
    recognize(pair(char('t'), tag_no_case("rue")))(input)
}
