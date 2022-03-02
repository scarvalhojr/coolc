//! Structures for formatting parse trees for printing.

use super::*;
use crate::util::escape_str;
use std::fmt::{Display, Formatter};
use FeatureData::*;

const INDENTATION: usize = 2;

pub struct ProgramFormatter<'a> {
    program: &'a Program<'a>,
}

impl<'a> ProgramFormatter<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self { program }
    }
}

impl Display for ProgramFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let line_number = self
            .program
            .classes
            .first()
            .map(|class| class.location.location_line())
            .unwrap_or(0);
        writeln!(
            f,
            "\
            #{line_number}\n\
            _program"
        )?;
        for class in self.program.classes.iter() {
            write!(f, "{}", class.format(INDENTATION))?;
        }
        Ok(())
    }
}

pub struct ClassFormatter<'a> {
    class: &'a Class<'a>,
    indent: usize,
}

impl<'a> ClassFormatter<'a> {
    pub fn new(class: &'a Class, indent: usize) -> Self {
        Self { class, indent }
    }
}

impl Display for ClassFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let line_number = self.class.location.location_line();
        let name = &self.class.name;
        let super_class = &self.class.super_class_name;
        let filename = self.class.location.extra;
        let indent = self.indent;
        let next_indent = self.indent + INDENTATION;
        writeln!(
            f,
            "\
            {empty:indent$}#{line_number}\n\
            {empty:indent$}_class\n\
            {empty:next_indent$}{name}\n\
            {empty:next_indent$}{super_class}\n\
            {empty:next_indent$}\"{filename}\"\n\
            {empty:next_indent$}(",
        )?;
        for feature in self.class.features.iter() {
            write!(f, "{}", feature.format(next_indent))?;
        }
        writeln!(f, "{empty:next_indent$})")
    }
}

pub struct FeatureFormatter<'a> {
    feature: &'a Feature<'a>,
    indent: usize,
}

impl<'a> FeatureFormatter<'a> {
    pub fn new(feature: &'a Feature, indent: usize) -> Self {
        Self { feature, indent }
    }
}

impl Display for FeatureFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        let line_num = self.feature.location.location_line();
        let feature = self.feature.data.format(indent);
        write!(f, "{empty:indent$}#{line_num}\n{feature}")
    }
}

pub struct FeatureDataFormatter<'a> {
    feature: &'a FeatureData<'a>,
    indent: usize,
}

impl<'a> FeatureDataFormatter<'a> {
    pub fn new(feature: &'a FeatureData, indent: usize) -> Self {
        Self { feature, indent }
    }
}

impl Display for FeatureDataFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        let next_indent = self.indent + INDENTATION;
        match self.feature {
            Attribute(name, type_id, init) => {
                writeln!(
                    f,
                    "\
                    {empty:indent$}_attr\n\
                    {empty:next_indent$}{name}\n\
                    {empty:next_indent$}{type_id}"
                )?;
                if let Some(expr) = init {
                    write!(f, "{}", expr.format(next_indent))
                } else {
                    write!(f, "{}", NoExpression::new(next_indent))
                }
            }
            Method(name, type_id, formals, expr) => {
                writeln!(
                    f,
                    "\
                    {empty:indent$}_method\n\
                    {empty:next_indent$}{name}"
                )?;
                for formal in formals {
                    write!(f, "{}", formal.format(next_indent))?;
                }
                write!(
                    f,
                    "\
                    {empty:next_indent$}{type_id}\n\
                    {}",
                    expr.format(next_indent)
                )
            }
        }
    }
}

pub struct FormalFormatter<'a> {
    formal: &'a Formal<'a>,
    indent: usize,
}

impl<'a> FormalFormatter<'a> {
    pub fn new(formal: &'a Formal, indent: usize) -> Self {
        Self { formal, indent }
    }
}

impl Display for FormalFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        let next_indent = self.indent + INDENTATION;
        let line_num = self.formal.location.location_line();
        let name = &self.formal.name;
        let type_id = &self.formal.type_id;
        writeln!(
            f,
            "\
            {empty:indent$}#{line_num}\n\
            {empty:indent$}_formal\n\
            {empty:next_indent$}{name}\n\
            {empty:next_indent$}{type_id}"
        )
    }
}

pub struct ExpressionFormatter<'a> {
    expression: &'a Expression<'a>,
    indent: usize,
}

impl<'a> ExpressionFormatter<'a> {
    pub fn new(expression: &'a Expression, indent: usize) -> Self {
        Self { expression, indent }
    }
}

impl Display for ExpressionFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        let line_num = self.expression.location.location_line();
        let expression = self.expression.data.format(indent);
        write!(f, "{empty:indent$}#{line_num}\n{expression}")
    }
}

pub struct ExpressionDataFormatter<'a> {
    expression: &'a ExpressionData<'a>,
    indent: usize,
}

impl<'a> ExpressionDataFormatter<'a> {
    pub fn new(expression: &'a ExpressionData, indent: usize) -> Self {
        Self { expression, indent }
    }
}

impl Display for ExpressionDataFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        let next_indent = self.indent + INDENTATION;

        match self.expression {
            Block(expressions) => {
                writeln!(f, "{empty:indent$}_block")?;
                for expr in expressions.iter() {
                    write!(f, "{}", expr.format(next_indent))?;
                }
                writeln!(f, "{empty:indent$}: _no_type")
            }
            Conditional(if_expr, then_expr, else_expr) => {
                let if_expression = if_expr.format(next_indent);
                let then_expression = then_expr.format(next_indent);
                let else_expression = else_expr.format(next_indent);
                writeln!(
                    f,
                    "\
                    {empty:indent$}_cond\n\
                    {if_expression}\
                    {then_expression}\
                    {else_expression}\
                    {empty:indent$}: _no_type"
                )
            }
            Loop(cond_expr, loop_expr) => {
                let cond_expression = cond_expr.format(next_indent);
                let loop_expression = loop_expr.format(next_indent);
                writeln!(
                    f,
                    "\
                    {empty:indent$}_loop\n\
                    {cond_expression}\
                    {loop_expression}\
                    {empty:indent$}: _no_type"
                )
            }
            Case(case_expr, branches) => {
                let case_expression = case_expr.format(next_indent);
                write!(
                    f,
                    "\
                    {empty:indent$}_typcase\n\
                    {case_expression}"
                )?;
                for branch in branches.iter() {
                    write!(f, "{}", branch.format(next_indent))?;
                }
                writeln!(f, "{empty:indent$}: _no_type")
            }
            Let(ident, type_id, opt_bind, expr) => {
                writeln!(
                    f,
                    "\
                    {empty:indent$}_let\n\
                    {empty:next_indent$}{ident}\n\
                    {empty:next_indent$}{type_id}"
                )?;
                if let Some(bind) = &**opt_bind {
                    write!(f, "{}", bind.format(next_indent))?;
                } else {
                    write!(f, "{}", NoExpression::new(next_indent))?;
                }
                writeln!(
                    f,
                    "\
                    {}\
                    {empty:indent$}: _no_type",
                    expr.format(next_indent),
                )
            }
            Assign(ident, expr) => {
                let expression = expr.format(next_indent);
                writeln!(
                    f,
                    "\
                    {empty:indent$}_assign\n\
                    {empty:next_indent$}{ident}\n\
                    {expression}\
                    {empty:indent$}: _no_type"
                )
            }
            New(type_id) => {
                writeln!(
                    f,
                    "\
                    {empty:indent$}_new\n\
                    {empty:next_indent$}{type_id}\n\
                    {empty:indent$}: _no_type"
                )
            }
            UnaryOperation(operator, operand) => {
                let oper = match operator {
                    UnaryOperator::Not => "_comp",
                    UnaryOperator::Negative => "_neg",
                    UnaryOperator::IsVoid => "_isvoid",
                };
                let op = operand.format(next_indent);
                writeln!(
                    f,
                    "\
                    {empty:indent$}{oper}\n\
                    {op}\
                    {empty:indent$}: _no_type"
                )
            }
            BinaryOperation(operator, operand1, operand2) => {
                let oper = match operator {
                    BinaryOperator::Equals => "_eq",
                    BinaryOperator::LessThanOrEquals => "_leq",
                    BinaryOperator::LessThan => "_lt",
                    BinaryOperator::Add => "_plus",
                    BinaryOperator::Subtract => "_sub",
                    BinaryOperator::Multiply => "_mul",
                    BinaryOperator::Divide => "_divide",
                };
                let op1 = operand1.format(next_indent);
                let op2 = operand2.format(next_indent);
                writeln!(
                    f,
                    "\
                    {empty:indent$}{oper}\n\
                    {op1}\
                    {op2}\
                    {empty:indent$}: _no_type"
                )
            }
            Object(ident) => {
                writeln!(
                    f,
                    "\
                    {empty:indent$}_object\n\
                    {empty:next_indent$}{ident}\n\
                    {empty:indent$}: _no_type"
                )
            }
            MethodCall(object, static_type, ident, params) => {
                let expression = object.format(next_indent);
                if let Some(type_id) = static_type {
                    writeln!(
                        f,
                        "\
                        {empty:indent$}_static_dispatch\n\
                        {expression}\
                        {empty:next_indent$}{type_id}\n\
                        {empty:next_indent$}{ident}\n\
                        {empty:next_indent$}(",
                    )?;
                } else {
                    writeln!(
                        f,
                        "\
                        {empty:indent$}_dispatch\n\
                        {expression}\
                        {empty:next_indent$}{ident}\n\
                        {empty:next_indent$}(",
                    )?;
                }
                for param in params.iter() {
                    write!(f, "{}", param.format(next_indent))?;
                }
                writeln!(
                    f,
                    "\
                    {empty:next_indent$})\n\
                    {empty:indent$}: _no_type"
                )
            }
            IntLiteral(integer) => {
                writeln!(
                    f,
                    "\
                    {empty:indent$}_int\n\
                    {empty:next_indent$}{integer}\n\
                    {empty:indent$}: _no_type"
                )
            }
            StrLiteral(string) => {
                let escaped_str = escape_str(string);
                writeln!(
                    f,
                    "\
                    {empty:indent$}_string\n\
                    {empty:next_indent$}\"{escaped_str}\"\n\
                    {empty:indent$}: _no_type"
                )
            }
            BoolLiteral(boolean) => {
                let int_value = *boolean as i32;
                writeln!(
                    f,
                    "\
                    {empty:indent$}_bool\n\
                    {empty:next_indent$}{int_value}\n\
                    {empty:indent$}: _no_type"
                )
            }
        }
    }
}

pub struct CaseBranchFormatter<'a> {
    branch: &'a CaseBranch<'a>,
    indent: usize,
}

impl<'a> CaseBranchFormatter<'a> {
    pub fn new(branch: &'a CaseBranch, indent: usize) -> Self {
        Self { branch, indent }
    }
}

impl Display for CaseBranchFormatter<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        let next_indent = self.indent + INDENTATION;
        let line_num = self.branch.location.location_line();
        let ident = &self.branch.ident;
        let type_id = &self.branch.type_id;
        let expression = self.branch.expression.format(next_indent);
        write!(
            f,
            "\
            {empty:indent$}#{line_num}\n\
            {empty:indent$}_branch\n\
            {empty:next_indent$}{ident}\n\
            {empty:next_indent$}{type_id}\n\
            {expression}"
        )
    }
}

struct NoExpression {
    indent: usize,
}

impl NoExpression {
    fn new(indent: usize) -> Self {
        Self { indent }
    }
}

impl Display for NoExpression {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        writeln!(
            f,
            "\
            {empty:indent$}#0\n\
            {empty:indent$}_no_expr\n\
            {empty:indent$}: _no_type"
        )
    }
}
