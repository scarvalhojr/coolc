//! Define the tokens of valid Cool source code.
//! The code here is largely a copy of the Monkey tokenizer from
//! https://github.com/Rydgel/monkey-rust

use core::slice::Iter;
use nom::{InputIter, InputLength, InputTake, Needed, Slice};
use std::fmt::{Display, Formatter};
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

// TODO: add line number to tokens

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Reserved words
    Class,
    Inherits,
    If,
    Then,
    Else,
    Fi,
    Let,
    In,
    While,
    Loop,
    Pool,
    Case,
    Of,
    Esac,
    New,
    IsVoid,
    Not,

    // Symbols (punctuation and operators)
    At,
    Assign,
    DoubleArrow,
    OpenBraces,
    CloseBraces,
    OpenParens,
    CloseParens,
    Dot,
    Comma,
    Colon,
    SemiColon,
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    Negative,
    LessThanOrEquals,
    LessThan,

    // Literals
    IntLiteral(i32),
    StrLiteral(String),
    BoolLiteral(bool),

    // Type and object identifiers
    TypeId(String),
    Ident(String),
}

// The format used here mimics the output of the reference lexer implementation
// used in the Compilers course.
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let escape = |s: &str| {
            s.chars().fold(String::new(), |mut string, ch| {
                match ch {
                    '\\' => string.push_str(r"\\"),
                    '\n' => string.push_str(r"\n"),
                    '\t' => string.push_str(r"\t"),
                    '\u{08}' => string.push_str(r"\b"),
                    '\u{0C}' => string.push_str(r"\f"),
                    c => string.push(c),
                };
                string
            })
        };

        let line_num = 1;
        let output = match self {
            Self::IntLiteral(value) => format!("INT_CONST {value}"),
            Self::StrLiteral(value) => {
                format!("STR_CONST \"{}\"", escape(value))
            }
            Self::BoolLiteral(value) => format!("BOOL_CONST {value}"),
            Self::TypeId(id) => format!("TYPEID {id}"),
            Self::Ident(id) => format!("OBJECTID {id}"),
            other => match other {
                Self::Class => "CLASS",
                Self::Inherits => "INHERITS",
                Self::If => "IF",
                Self::Then => "THEN",
                Self::Else => "ELSE",
                Self::Fi => "FI",
                Self::Let => "LET",
                Self::In => "IN",
                Self::While => "WHILE",
                Self::Loop => "LOOP",
                Self::Pool => "POOL",
                Self::Case => "CASE",
                Self::Of => "OF",
                Self::Esac => "ESAC",
                Self::New => "NEW",
                Self::IsVoid => "ISVOID",
                Self::Not => "NOT",
                Self::At => "'@'",
                Self::Assign => "ASSIGN",
                Self::DoubleArrow => "DARROW",
                Self::OpenBraces => r"'{'",
                Self::CloseBraces => r"'}'",
                Self::OpenParens => "'('",
                Self::CloseParens => "')'",
                Self::Dot => "'.'",
                Self::Comma => "','",
                Self::Colon => "':'",
                Self::SemiColon => "';'",
                Self::Equals => "'='",
                Self::Plus => "'+'",
                Self::Minus => "'-'",
                Self::Multiply => "'*'",
                Self::Divide => "'/'",
                Self::Negative => "'~'",
                Self::LessThanOrEquals => "LE",
                Self::LessThan => "'<'",
                _ => unreachable!(),
            }
            .to_string(),
        };
        write!(f, "#{line_num} {output}")
    }
}
// #[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tokens: &'a [Token],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Tokens {
            tokens,
            start: 0,
            end: tokens.len(),
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    fn take(&self, count: usize) -> Self {
        Tokens {
            tokens: &self.tokens[0..count],
            start: 0,
            end: count,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        let first = Tokens {
            tokens: prefix,
            start: 0,
            end: prefix.len(),
        };
        let second = Tokens {
            tokens: suffix,
            start: 0,
            end: suffix.len(),
        };
        (second, first)
    }
}

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tokens: self.tokens.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    fn slice(&self, _: RangeFull) -> Self {
        Tokens {
            tokens: self.tokens,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token;
    type Iter = Enumerate<Iter<'a, Token>>;
    type IterElem = Iter<'a, Token>;

    fn iter_indices(&self) -> Enumerate<Iter<'a, Token>> {
        self.tokens.iter().enumerate()
    }

    fn iter_elements(&self) -> Iter<'a, Token> {
        self.tokens.iter()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tokens.iter().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}
