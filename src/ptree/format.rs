//! Structures for formatting parse trees for printing.

use super::*;
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

impl<'a> Display for ProgramFormatter<'a> {
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

impl<'a> Display for ClassFormatter<'a> {
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

impl<'a> Display for FeatureFormatter<'a> {
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

impl<'a> Display for FeatureDataFormatter<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.feature {
            Attribute(name, type_id, init) => {
                let empty = "";
                let indent = self.indent;
                let next_indent = self.indent + INDENTATION;
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
                    writeln!(
                        f,
                        "\
                        {empty:next_indent$}#0\n\
                        {empty:next_indent$}_no_expr\n\
                        {empty:next_indent$}: _no_type"
                    )
                }
            }
        }
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

impl<'a> Display for ExpressionFormatter<'a> {
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

impl<'a> Display for ExpressionDataFormatter<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let indent = self.indent;
        let next_indent = self.indent + INDENTATION;

        match self.expression {
            IntLiteral(integer) => {
                writeln!(
                    f,
                    "\
                    {empty:indent$}_int\n\
                    {empty:next_indent$}{integer}\n\
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
            _ => unimplemented!(),
        }
    }
}
