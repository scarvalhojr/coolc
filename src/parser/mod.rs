//! The parsing functions take a series of tokens (produced by the lexer from
//! Cool source code) and produce a parse tree.

use crate::ptree::*;
use crate::tokens::{Ident, Span, Token, TokenKind, Tokens, TypeId};
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::{eof, map, map_res, opt, peek};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;
use FeatureData::*;

#[cfg(test)]
mod tests;

// TODO: handle parsing errors

pub fn parse_program<'a>(
    tokens: &'a [Token],
) -> IResult<Tokens<'a>, Program<'a>> {
    program(Tokens::new(tokens))
}

fn program(input: Tokens) -> IResult<Tokens, Program> {
    map(
        terminated(many1(terminated(class, semicolon_token)), eof),
        Program::new,
    )(input)
}

fn class(input: Tokens) -> IResult<Tokens, Class> {
    let (_, location) = current_location(input)?;
    map(
        tuple((
            preceded(class_token, type_id),
            opt(preceded(inherits_token, type_id)),
            delimited(open_braces_token, many0(feature), close_braces_token),
        )),
        move |(name, super_name, features)| {
            Class::new(name, super_name, features, location)
        },
    )(input)
}

fn feature(input: Tokens) -> IResult<Tokens, Feature> {
    // TODO: methods
    terminated(attribute, semicolon_token)(input)
}

fn attribute(input: Tokens) -> IResult<Tokens, Feature> {
    let (_, location) = current_location(input)?;
    map(
        tuple((
            terminated(ident, colon_token),
            type_id,
            opt(preceded(assign_token, expression)),
        )),
        move |(obj_id, type_id, expr)| {
            Feature::new(Attribute(obj_id, type_id, expr), location)
        },
    )(input)
}

fn expression<'a>(input: Tokens<'a>) -> IResult<Tokens, Expression<'a>> {
    let (rest, operand1) = term(input)?;
    many0(tuple((alt((add_token, subtract_token)), term)))(rest).map(
        |(tokens, operations)| (tokens, build_arith_expr(operand1, operations)),
    )
}

fn term<'a>(input: Tokens<'a>) -> IResult<Tokens, Expression<'a>> {
    let (rest, operand1) = factor(input)?;
    many0(tuple((alt((multiply_token, divide_token)), factor)))(rest).map(
        |(tokens, operations)| (tokens, build_arith_expr(operand1, operations)),
    )
}

fn factor<'a>(input: Tokens<'a>) -> IResult<Tokens, Expression<'a>> {
    alt((
        // TODO: lots more here...
        parens_expression,
        literal,
    ))(input)
}

fn parens_expression<'a>(input: Tokens<'a>) -> IResult<Tokens, Expression<'a>> {
    delimited(open_parens_token, expression, close_parens_token)(input)
}

fn build_arith_expr<'a>(
    first_operand: Expression<'a>,
    operations: Vec<(Token<'a>, Expression<'a>)>,
) -> Expression<'a> {
    operations
        .into_iter()
        .fold(first_operand, |acc, (token, expr)| {
            let expr_data =
                ExpressionData::new_arith_operation(acc, &token.kind, expr);
            let location = token.location;
            Expression::new(expr_data, location)
        })
}

fn literal(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(
        map_res(take(1_usize), |current: Tokens| {
            current.array.first().ok_or(()).and_then(|token| {
                match &token.kind {
                    TokenKind::BoolLiteral(b) => {
                        Ok(ExpressionData::BoolLiteral(*b))
                    }
                    TokenKind::IntLiteral(i) => {
                        Ok(ExpressionData::IntLiteral(*i))
                    }
                    TokenKind::StrLiteral(s) => {
                        Ok(ExpressionData::StrLiteral(s.clone()))
                    }
                    _ => Err(()),
                }
            })
        }),
        move |expr_data| Expression::new(expr_data, location),
    )(input)
}

fn type_id(input: Tokens) -> IResult<Tokens, TypeId> {
    map_res(take(1_usize), |current: Tokens| {
        current.array.first().ok_or(()).and_then(|token| {
            if let TokenKind::TypeId(name) = &token.kind {
                Ok(name.clone())
            } else {
                Err(())
            }
        })
    })(input)
}

fn ident(input: Tokens) -> IResult<Tokens, Ident> {
    map_res(take(1_usize), |current: Tokens| {
        current.array.first().ok_or(()).and_then(|token| {
            if let TokenKind::Ident(name) = &token.kind {
                Ok(name.clone())
            } else {
                Err(())
            }
        })
    })(input)
}

macro_rules! token_kind (
    ($func_name:ident, $kind: expr) => (
        fn $func_name(tokens: Tokens) -> IResult<Tokens, Token> {
            map_res(take(1_usize), |current: Tokens| {
                current.array.first().ok_or(()).and_then(|token| {
                    if token.kind == $kind {
                        Ok(token.clone())
                    } else {
                        Err(())
                    }
                })
            })(tokens)
        }
    )
);

token_kind!(class_token, TokenKind::Class);
token_kind!(inherits_token, TokenKind::Inherits);
token_kind!(if_token, TokenKind::If);
token_kind!(then_token, TokenKind::Then);
token_kind!(else_token, TokenKind::Else);
token_kind!(fi_token, TokenKind::Fi);
token_kind!(let_token, TokenKind::Let);
token_kind!(in_token, TokenKind::In);
token_kind!(while_token, TokenKind::While);
token_kind!(loop_token, TokenKind::Loop);
token_kind!(pool_token, TokenKind::Pool);
token_kind!(case_token, TokenKind::Case);
token_kind!(of_token, TokenKind::Of);
token_kind!(esac_token, TokenKind::Esac);
token_kind!(new_token, TokenKind::New);
token_kind!(isvoid_token, TokenKind::IsVoid);
token_kind!(not_token, TokenKind::Not);
token_kind!(at_token, TokenKind::At);
token_kind!(assign_token, TokenKind::Assign);
token_kind!(double_arrow_token, TokenKind::DoubleArrow);
token_kind!(open_braces_token, TokenKind::OpenBraces);
token_kind!(close_braces_token, TokenKind::CloseBraces);
token_kind!(open_parens_token, TokenKind::OpenParens);
token_kind!(close_parens_token, TokenKind::CloseParens);
token_kind!(dot_token, TokenKind::Dot);
token_kind!(comma_token, TokenKind::Comma);
token_kind!(colon_token, TokenKind::Colon);
token_kind!(semicolon_token, TokenKind::SemiColon);
token_kind!(equals_token, TokenKind::Equals);
token_kind!(add_token, TokenKind::Add);
token_kind!(subtract_token, TokenKind::Subtract);
token_kind!(multiply_token, TokenKind::Multiply);
token_kind!(divide_token, TokenKind::Divide);
token_kind!(negative_token, TokenKind::Negative);
token_kind!(less_than_or_equals_token, TokenKind::LessThanOrEquals);
token_kind!(less_than_token, TokenKind::LessThan);

fn current_location(input: Tokens) -> IResult<Tokens, Span> {
    map_res(peek(take(1_usize)), |current: Tokens| {
        current.array.first().ok_or(()).map(|token| token.location)
    })(input)
}
