//! Structures for printing parse trees.

use super::*;
use std::fmt::{Display, Formatter};

const INDENTATION: usize = 2;

pub struct ProgramPrinter<'a> {
    program: &'a Program,
    filename: &'a str,
}

impl<'a> ProgramPrinter<'a> {
    pub fn new(program: &'a Program, filename: &'a str) -> Self {
        Self { program, filename }
    }
}

impl<'a> Display for ProgramPrinter<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "\
            #1\n\
            _program"
        )?;
        for class in self.program.classes.iter() {
            write!(f, "{}", class.print(self.filename, INDENTATION))?;
        }
        Ok(())
    }
}

pub struct ClassPrinter<'a> {
    class: &'a Class,
    filename: &'a str,
    indent: usize,
}

impl<'a> ClassPrinter<'a> {
    pub fn new(class: &'a Class, filename: &'a str, indent: usize) -> Self {
        Self {
            class,
            filename,
            indent,
        }
    }
}

impl<'a> Display for ClassPrinter<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let empty = "";
        let name = &self.class.name;
        let super_class = &self.class.super_class_name;
        let filename = self.filename;
        let indent = self.indent;
        let next_indent = self.indent + INDENTATION;
        writeln!(
            f,
            "\
            {empty:indent$}#0\n\
            {empty:indent$}_class\n\
            {empty:next_indent$}{name}\n\
            {empty:next_indent$}{super_class}\n\
            {empty:next_indent$}\"{filename}\"\n\
            {empty:next_indent$}(",
        )?;
        for feature in self.class.features.iter() {
            write!(f, "{}", feature.print(next_indent))?;
        }
        writeln!(f, "{empty:next_indent$})")
    }
}

pub struct FeaturePrinter<'a> {
    feature: &'a Feature,
    indent: usize,
}

impl<'a> FeaturePrinter<'a> {
    pub fn new(feature: &'a Feature, indent: usize) -> Self {
        Self { feature, indent }
    }
}

impl<'a> Display for FeaturePrinter<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.feature {
            Attribute(name, type_id, init) => {
                let empty = "";
                let indent = self.indent;
                let next_indent = self.indent + INDENTATION;
                writeln!(
                    f,
                    "\
                    {empty:indent$}#0\n\
                    {empty:indent$}_attr\n\
                    {empty:next_indent$}{name}\n\
                    {empty:next_indent$}{type_id}"
                )?;
                if init.is_none() {
                    writeln!(
                        f,
                        "\
                        {empty:next_indent$}#0\n\
                        {empty:next_indent$}_no_expr\n\
                        {empty:next_indent$}: _no_type"
                    )?;
                }
                Ok(())
            }
        }
    }
}
