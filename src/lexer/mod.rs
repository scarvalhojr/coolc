//! The lexer functions read Cool source code as a string and produce a series
//! of tokens.

use crate::tokens::{Span, Token, TokenKind};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while1};
use nom::character::complete::{
    alphanumeric1, anychar, char, digit1, multispace1, none_of, not_line_ending,
};
use nom::combinator::{eof, map, map_res, peek, recognize, value, verify};
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use nom_locate::position;
use std::str::FromStr;

#[cfg(test)]
mod tests;

// TODO: handle errors

pub fn lex_tokens<'a>(
    input: &'a str,
    filename: &'a str,
) -> IResult<Span<'a>, Vec<Token<'a>>> {
    tokens(Span::new_extra(input, filename))
}

fn tokens(input: Span) -> IResult<Span, Vec<Token>> {
    terminated(
        many0(preceded(many0(discarded), token)),
        preceded(many0(discarded), eof),
    )(input)
}

fn discarded(input: Span) -> IResult<Span, ()> {
    alt((value((), multispace1), line_comment, block_comment))(input)
}

fn line_comment(input: Span) -> IResult<Span, ()> {
    value((), preceded(tag("--"), not_line_ending))(input)
}

fn block_comment(input: Span) -> IResult<Span, ()> {
    delimited(
        tag("(*"),
        value((), many0(alt((block_comment, block_comment_contents)))),
        tag("*)"),
    )(input)
}

fn block_comment_contents(input: Span) -> IResult<Span, ()> {
    alt((
        value((), preceded(char('('), peek(none_of("*")))),
        value((), preceded(char('*'), peek(none_of(")")))),
        value((), is_not("*(")),
    ))(input)
}

fn token(input: Span) -> IResult<Span, Token> {
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

fn reserved_word(input: Span) -> IResult<Span, Token> {
    let (_, pos) = position(input)?;
    map(
        map_res(
            // Capture the longest sequence of alphanumeric characters and
            // '_' to make sure we don't confuse the prefix of a valid identifier
            // or type with a reserved word. For instance, "class1" and "If2" should
            // not be confused with reserved words.
            recognize(many0(alt((alphanumeric1, tag("_"))))),
            |s: Span| match s.to_lowercase().as_str() {
                "class" => Ok(TokenKind::Class),
                "inherits" => Ok(TokenKind::Inherits),
                "if" => Ok(TokenKind::If),
                "then" => Ok(TokenKind::Then),
                "else" => Ok(TokenKind::Else),
                "fi" => Ok(TokenKind::Fi),
                "let" => Ok(TokenKind::Let),
                "in" => Ok(TokenKind::In),
                "while" => Ok(TokenKind::While),
                "loop" => Ok(TokenKind::Loop),
                "pool" => Ok(TokenKind::Pool),
                "case" => Ok(TokenKind::Case),
                "of" => Ok(TokenKind::Of),
                "esac" => Ok(TokenKind::Esac),
                "new" => Ok(TokenKind::New),
                "isvoid" => Ok(TokenKind::IsVoid),
                "not" => Ok(TokenKind::Not),
                _ => Err(()),
            },
        ),
        move |kind| Token::new(kind, pos),
    )(input)
}

fn symbol(input: Span) -> IResult<Span, Token> {
    let (_, pos) = position(input)?;
    map(
        alt((
            value(TokenKind::At, tag("@")),
            value(TokenKind::Assign, tag("<-")),
            value(TokenKind::DoubleArrow, tag("=>")),
            value(TokenKind::OpenBraces, tag("{")),
            value(TokenKind::CloseBraces, tag("}")),
            value(TokenKind::OpenParens, tag("(")),
            value(TokenKind::CloseParens, tag(")")),
            value(TokenKind::Dot, tag(".")),
            value(TokenKind::Comma, tag(",")),
            value(TokenKind::Colon, tag(":")),
            value(TokenKind::SemiColon, tag(";")),
            value(TokenKind::Equals, tag("=")),
            value(TokenKind::Add, tag("+")),
            value(TokenKind::Subtract, tag("-")),
            value(TokenKind::Multiply, tag("*")),
            value(TokenKind::Divide, tag("/")),
            value(TokenKind::Negative, tag("~")),
            value(TokenKind::LessThanOrEquals, tag("<=")),
            value(TokenKind::LessThan, tag("<")),
        )),
        move |kind| Token::new(kind, pos),
    )(input)
}

fn int_literal(input: Span) -> IResult<Span, Token> {
    let (_, pos) = position(input)?;
    map(
        map_res(digit1, |s: Span| i32::from_str(&s)),
        move |integer| Token::new(TokenKind::IntLiteral(integer), pos),
    )(input)
}

fn str_literal(input: Span) -> IResult<Span, Token> {
    let build_string =
        fold_many0(str_fragment, String::new, |mut string, fragment| {
            match fragment {
                StrFragment::UnescapedFragment(s) => string.push_str(&s),
                StrFragment::EscapedChar(c) => string.push(c),
            }
            string
        });

    let (_, pos) = position(input)?;
    map(
        delimited(char('"'), build_string, char('"')),
        move |string| Token::new(TokenKind::StrLiteral(string), pos),
    )(input)
}

enum StrFragment<'a> {
    UnescapedFragment(Span<'a>),
    EscapedChar(char),
}

fn str_fragment(input: Span) -> IResult<Span, StrFragment> {
    alt((
        map(unescaped_fragment, StrFragment::UnescapedFragment),
        map(escaped_char, StrFragment::EscapedChar),
    ))(input)
}

fn unescaped_fragment(input: Span) -> IResult<Span, Span> {
    take_while1(|c: char| c != '\\' && c != '"')(input)
}

fn escaped_char(input: Span) -> IResult<Span, char> {
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

fn bool_literal(input: Span) -> IResult<Span, Token> {
    alt((false_literal, true_literal))(input)
}

fn false_literal(input: Span) -> IResult<Span, Token> {
    let (_, pos) = position(input)?;
    value(
        Token::new(TokenKind::BoolLiteral(false), pos),
        pair(
            char('f'),
            verify(
                recognize(many0(alt((alphanumeric1, tag("_"))))),
                |s: &Span| s.to_lowercase() == "alse",
            ),
        ),
    )(input)
}

fn true_literal(input: Span) -> IResult<Span, Token> {
    let (_, pos) = position(input)?;
    value(
        Token::new(TokenKind::BoolLiteral(true), pos),
        pair(
            char('t'),
            verify(
                recognize(many0(alt((alphanumeric1, tag("_"))))),
                |s: &Span| s.to_lowercase() == "rue",
            ),
        ),
    )(input)
}

fn type_id(input: Span) -> IResult<Span, Token> {
    let (_, pos) = position(input)?;
    map(
        recognize(pair(
            take_while1(|c: char| c.is_uppercase()),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        move |s: Span| Token::new(TokenKind::TypeId(s.to_string()), pos),
    )(input)
}

fn ident(input: Span) -> IResult<Span, Token> {
    let (_, pos) = position(input)?;
    map(
        recognize(pair(
            take_while1(|c: char| c.is_lowercase()),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        move |s: Span| Token::new(TokenKind::Ident(s.to_string()), pos),
    )(input)
}
