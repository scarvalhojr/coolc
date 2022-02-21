//! The lexer functions read Cool source code as a string and produce a series
//! of tokens.

use crate::tokens::Token;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while1};
use nom::character::complete::{
    alphanumeric1, anychar, char, digit1, multispace1, none_of, not_line_ending,
};
use nom::combinator::{eof, map, map_res, peek, recognize, value, verify};
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use std::str::FromStr;

#[cfg(test)]
mod tests;

// TODO: handle errors

pub fn lex_tokens(input: &str) -> IResult<&str, Vec<Token>> {
    terminated(
        many0(preceded(many0(discarded), token)),
        preceded(many0(discarded), eof),
    )(input)
}

fn discarded(input: &str) -> IResult<&str, ()> {
    alt((value((), multispace1), line_comment, block_comment))(input)
}

fn line_comment(input: &str) -> IResult<&str, ()> {
    value((), preceded(tag("--"), not_line_ending))(input)
}

fn block_comment(input: &str) -> IResult<&str, ()> {
    delimited(
        tag("(*"),
        value((), many0(alt((block_comment, block_comment_contents)))),
        tag("*)"),
    )(input)
}

fn block_comment_contents(input: &str) -> IResult<&str, ()> {
    alt((
        value((), preceded(char('('), peek(none_of("*")))),
        value((), preceded(char('*'), peek(none_of(")")))),
        value((), is_not("*(")),
    ))(input)
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((
        reserved_word,
        symbol,
        int_literal,
        str_literal,
        bool_literal,
        type_id,
        ident,
    ))(input)
}

fn reserved_word(input: &str) -> IResult<&str, Token> {
    map_res(
        // Capture the longest sequence of alphanumeric characters and
        // '_' to make sure we don't confuse the prefix of a valid identifier
        // or type with a reserved word. For instance, "class1" and "If2" should
        // not be confused with reserved words.
        recognize(many0(alt((alphanumeric1, tag("_"))))),
        |s: &str| match s.to_lowercase().as_str() {
            "class" => Ok(Token::Class),
            "inherits" => Ok(Token::Inherits),
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "fi" => Ok(Token::Fi),
            "let" => Ok(Token::Let),
            "in" => Ok(Token::In),
            "while" => Ok(Token::While),
            "loop" => Ok(Token::Loop),
            "pool" => Ok(Token::Pool),
            "case" => Ok(Token::Case),
            "of" => Ok(Token::Of),
            "esac" => Ok(Token::Esac),
            "new" => Ok(Token::New),
            "isvoid" => Ok(Token::IsVoid),
            "not" => Ok(Token::Not),
            _ => Err(()),
        },
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Token> {
    alt((
        value(Token::At, tag("@")),
        value(Token::Assign, tag("<-")),
        value(Token::DoubleArrow, tag("=>")),
        value(Token::OpenBraces, tag("{")),
        value(Token::CloseBraces, tag("}")),
        value(Token::OpenParens, tag("(")),
        value(Token::CloseParens, tag(")")),
        value(Token::Dot, tag(".")),
        value(Token::Comma, tag(",")),
        value(Token::Colon, tag(":")),
        value(Token::SemiColon, tag(";")),
        value(Token::Equals, tag("=")),
        value(Token::Plus, tag("+")),
        value(Token::Minus, tag("-")),
        value(Token::Multiply, tag("*")),
        value(Token::Divide, tag("/")),
        value(Token::Negative, tag("~")),
        value(Token::LessThanOrEquals, tag("<=")),
        value(Token::LessThan, tag("<")),
    ))(input)
}

fn int_literal(input: &str) -> IResult<&str, Token> {
    map(map_res(digit1, FromStr::from_str), Token::IntLiteral)(input)
}

fn str_literal(input: &str) -> IResult<&str, Token> {
    let build_string =
        fold_many0(str_fragment, String::new, |mut string, fragment| {
            match fragment {
                StrFragment::UnescapedFragment(s) => string.push_str(s),
                StrFragment::EscapedChar(c) => string.push(c),
            }
            string
        });

    map(
        delimited(char('"'), build_string, char('"')),
        Token::StrLiteral,
    )(input)
}

enum StrFragment<'a> {
    UnescapedFragment(&'a str),
    EscapedChar(char),
}

fn str_fragment(input: &str) -> IResult<&str, StrFragment> {
    alt((
        map(unescaped_fragment, StrFragment::UnescapedFragment),
        map(escaped_char, StrFragment::EscapedChar),
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

fn bool_literal(input: &str) -> IResult<&str, Token> {
    alt((false_literal, true_literal))(input)
}

fn false_literal(input: &str) -> IResult<&str, Token> {
    value(
        Token::BoolLiteral(false),
        pair(
            char('f'),
            verify(
                recognize(many0(alt((alphanumeric1, tag("_"))))),
                |s: &str| s.to_lowercase() == "alse",
            ),
        ),
    )(input)
}

fn true_literal(input: &str) -> IResult<&str, Token> {
    value(
        Token::BoolLiteral(true),
        pair(
            char('t'),
            verify(
                recognize(many0(alt((alphanumeric1, tag("_"))))),
                |s: &str| s.to_lowercase() == "rue",
            ),
        ),
    )(input)
}

fn type_id(input: &str) -> IResult<&str, Token> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_uppercase()),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| Token::TypeId(s.to_string()),
    )(input)
}

fn ident(input: &str) -> IResult<&str, Token> {
    map(
        recognize(pair(
            take_while1(|c: char| c.is_lowercase()),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| Token::Ident(s.to_string()),
    )(input)
}
