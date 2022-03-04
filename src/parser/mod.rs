//! The parsing functions take a series of tokens (produced by the lexer from
//! Cool source code) and produce a parse tree.

use crate::ptree::*;
use crate::tokens::{Ident, Span, Token, TokenKind, Tokens, TypeId};
use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::{eof, map, map_res, opt, peek};
use nom::multi::{many0, many1, separated_list0, separated_list1};
use nom::sequence::{
    delimited, pair, preceded, separated_pair, terminated, tuple,
};
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
    terminated(alt((attribute, method)), semicolon_token)(input)
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

fn method(input: Tokens) -> IResult<Tokens, Feature> {
    let (_, location) = current_location(input)?;
    map(
        tuple((
            ident,
            delimited(
                open_parens_token,
                separated_list0(comma_token, formal),
                close_parens_token,
            ),
            preceded(colon_token, type_id),
            delimited(open_braces_token, expression, close_braces_token),
        )),
        move |(obj_id, formals, type_id, expr)| {
            Feature::new(Method(obj_id, type_id, formals, expr), location)
        },
    )(input)
}

fn formal(input: Tokens) -> IResult<Tokens, Formal> {
    let (_, location) = current_location(input)?;
    map(
        separated_pair(ident, colon_token, type_id),
        move |(id, type_id)| Formal::new(id, type_id, location),
    )(input)
}

// Operator precedence (lowest to highest):
// assign
// not
// less_than_or_equals, less_than, equals
// add, subtract
// multiply, divide
// isvoid
// negative
// at
// dot

// To implement the precedence above, expressions are composed as follows:
// - expression
//     - assign
//     - bool_not_oper
//     - comparison_oper
//         - add_sub_oper
//             - mul_div_oper
//                 - operand
//                     - isvoid_oper
//                         - term
//                     - negative_oper
//                         - term
//                     - term
//                         - implicit_self_method_call
//                         - method_call
//                             - value
//                                 - parens_expression
//                                 - expression_block
//                                 - conditional_expression
//                                 - loop_expression
//                                 - case_expression
//                                 - let_expression
//                                 - new_object
//                                 - object
//                                 - literal

fn expression(input: Tokens) -> IResult<Tokens, Expression> {
    alt((assign, bool_not_oper, comparison_oper))(input)
}

fn assign(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(
        separated_pair(ident, assign_token, expression),
        move |(id, expr)| {
            Expression::new(ExpressionData::new_assign(id, expr), location)
        },
    )(input)
}

fn bool_not_oper(input: Tokens) -> IResult<Tokens, Expression> {
    map(pair(not_token, expression), |(token, expr)| {
        let expr_data = ExpressionData::new_unary_operation(&token.kind, expr);
        let location = token.location;
        Expression::new(expr_data, location)
    })(input)
}

fn comparison_oper(input: Tokens) -> IResult<Tokens, Expression> {
    let (rest, operand1) = add_sub_oper(input)?;
    many0(pair(
        alt((less_than_or_equals_token, less_than_token, equals_token)),
        add_sub_oper,
    ))(rest)
    .map(|(tokens, operations)| {
        (tokens, build_binary_expr(operand1, operations))
    })
}

fn add_sub_oper(input: Tokens) -> IResult<Tokens, Expression> {
    let (rest, operand1) = mul_div_oper(input)?;
    many0(pair(alt((add_token, subtract_token)), mul_div_oper))(rest).map(
        |(tokens, operations)| {
            (tokens, build_binary_expr(operand1, operations))
        },
    )
}

fn mul_div_oper(input: Tokens) -> IResult<Tokens, Expression> {
    let (rest, operand1) = operand(input)?;
    many0(pair(alt((multiply_token, divide_token)), operand))(rest).map(
        |(tokens, operations)| {
            (tokens, build_binary_expr(operand1, operations))
        },
    )
}

fn build_binary_expr<'a>(
    first_operand: Expression<'a>,
    operations: Vec<(Token<'a>, Expression<'a>)>,
) -> Expression<'a> {
    operations
        .into_iter()
        .fold(first_operand, |acc, (token, expr)| {
            let expr_data =
                ExpressionData::new_binary_operation(acc, &token.kind, expr);
            let location = token.location;
            Expression::new(expr_data, location)
        })
}

fn operand(input: Tokens) -> IResult<Tokens, Expression> {
    alt((isvoid_oper, negative_oper, term))(input)
}

fn isvoid_oper(input: Tokens) -> IResult<Tokens, Expression> {
    map(pair(isvoid_token, term), |(token, expr)| {
        let expr_data = ExpressionData::new_unary_operation(&token.kind, expr);
        let location = token.location;
        Expression::new(expr_data, location)
    })(input)
}

fn negative_oper(input: Tokens) -> IResult<Tokens, Expression> {
    map(pair(negative_token, term), |(token, expr)| {
        let expr_data = ExpressionData::new_unary_operation(&token.kind, expr);
        let location = token.location;
        Expression::new(expr_data, location)
    })(input)
}

fn term(input: Tokens) -> IResult<Tokens, Expression> {
    alt((implicit_self_method_call, method_call))(input)
}

type CallInfo<'a> = (Span<'a>, Option<TypeId>, Ident, Vec<Expression<'a>>);

fn implicit_self_method_call(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(
        pair(call, many0(chained_call)),
        move |((id, params), chained)| {
            let callee = Expression::new(
                ExpressionData::Object("self".to_string()),
                location,
            );
            let first_call_info = (location, None, id, params);
            build_chained_calls(callee, first_call_info, chained)
        },
    )(input)
}

fn method_call(input: Tokens) -> IResult<Tokens, Expression> {
    map(
        pair(
            value,
            opt(tuple((
                opt(preceded(at_token, type_id)),
                dot_token,
                call,
                many0(chained_call),
            ))),
        ),
        move |(callee, calls)| match calls {
            Some((static_type, dot, (id, params), chained)) => {
                let first_call_info = (dot.location, static_type, id, params);
                build_chained_calls(callee, first_call_info, chained)
            }
            None => callee,
        },
    )(input)
}

fn build_chained_calls<'a>(
    callee: Expression<'a>,
    first_call: CallInfo<'a>,
    chained: Vec<CallInfo<'a>>,
) -> Expression<'a> {
    let (location, static_type, id, params) = first_call;
    let base = Expression::new(
        ExpressionData::new_method_call(callee, static_type, id, params),
        location,
    );
    chained.into_iter().fold(
        base,
        |acc, (location, static_type, id, params)| {
            Expression::new(
                ExpressionData::new_method_call(acc, static_type, id, params),
                location,
            )
        },
    )
}

fn call(input: Tokens) -> IResult<Tokens, (Ident, Vec<Expression>)> {
    pair(
        ident,
        delimited(
            open_parens_token,
            separated_list0(comma_token, expression),
            close_parens_token,
        ),
    )(input)
}

fn chained_call(input: Tokens) -> IResult<Tokens, CallInfo> {
    map(
        tuple((opt(preceded(at_token, type_id)), dot_token, call)),
        |(static_type, dot, (ident, exprs))| {
            (dot.location, static_type, ident, exprs)
        },
    )(input)
}

fn value(input: Tokens) -> IResult<Tokens, Expression> {
    alt((
        parens_expression,
        expression_block,
        conditional_expression,
        loop_expression,
        case_expression,
        let_expression,
        new_object,
        object,
        literal,
    ))(input)
}

fn parens_expression(input: Tokens) -> IResult<Tokens, Expression> {
    delimited(open_parens_token, expression, close_parens_token)(input)
}

fn expression_block(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(
        delimited(
            open_braces_token,
            many1(terminated(expression, semicolon_token)),
            close_braces_token,
        ),
        move |expressions| {
            Expression::new(ExpressionData::Block(expressions), location)
        },
    )(input)
}

fn conditional_expression(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(
        tuple((
            preceded(if_token, expression),
            preceded(then_token, expression),
            delimited(else_token, expression, fi_token),
        )),
        move |(if_expr, then_expr, else_expr)| {
            Expression::new(
                ExpressionData::new_conditional(if_expr, then_expr, else_expr),
                location,
            )
        },
    )(input)
}

fn loop_expression(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(
        pair(
            preceded(while_token, expression),
            delimited(loop_token, expression, pool_token),
        ),
        move |(cond_expr, loop_expr)| {
            Expression::new(
                ExpressionData::new_loop(cond_expr, loop_expr),
                location,
            )
        },
    )(input)
}

fn case_expression(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(
        pair(
            preceded(case_token, expression),
            delimited(of_token, many1(case_branch), esac_token),
        ),
        move |(case_expr, branches)| {
            Expression::new(
                ExpressionData::new_case(case_expr, branches),
                location,
            )
        },
    )(input)
}

fn case_branch(input: Tokens) -> IResult<Tokens, CaseBranch> {
    let (_, location) = current_location(input)?;
    map(
        tuple((
            terminated(ident, colon_token),
            terminated(type_id, double_arrow_token),
            terminated(expression, semicolon_token),
        )),
        move |(id, type_id, expr)| CaseBranch::new(id, type_id, expr, location),
    )(input)
}

fn let_expression(input: Tokens) -> IResult<Tokens, Expression> {
    map(
        pair(
            preceded(let_token, separated_list1(comma_token, let_binding)),
            preceded(in_token, expression),
        ),
        move |(bindings, expression)| {
            bindings.into_iter().rev().fold(
                expression,
                |acc, (ident, type_id, opt_bind, location)| {
                    Expression::new(
                        ExpressionData::new_let(ident, type_id, opt_bind, acc),
                        location,
                    )
                },
            )
        },
    )(input)
}

fn let_binding(
    input: Tokens,
) -> IResult<Tokens, (Ident, TypeId, Option<Expression>, Span)> {
    let (_, location) = current_location(input)?;
    map(
        tuple((
            terminated(ident, colon_token),
            type_id,
            opt(preceded(assign_token, expression)),
        )),
        move |(ident, type_id, opt_expr)| (ident, type_id, opt_expr, location),
    )(input)
}

fn new_object(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(preceded(new_token, type_id), move |id| {
        Expression::new(ExpressionData::New(id), location)
    })(input)
}

fn object(input: Tokens) -> IResult<Tokens, Expression> {
    let (_, location) = current_location(input)?;
    map(ident, move |id| {
        Expression::new(ExpressionData::Object(id), location)
    })(input)
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
