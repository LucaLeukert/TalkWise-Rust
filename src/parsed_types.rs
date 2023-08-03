use crate::compiler::NumericConstant;
use crate::lexer::Span;

#[derive(Debug)]
pub struct ParsedBlock {
    pub statements: Vec<ParsedStatement>,
    pub span: Span,
}
impl ParsedBlock {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            span: Span::new(0, 0, 0),
        }
    }
    pub fn new_with_span(span: Span) -> Self {
        Self {
            statements: Vec::new(),
            span,
        }
    }
}

#[derive(Debug)]
pub struct ParsedFunction {
    pub name: String,
    pub name_span: Span,

    pub block: ParsedBlock,

    pub span: Span,
}
impl ParsedFunction {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            name_span: Span::new(0, 0, 0),
            block: ParsedBlock::new(),
            span: Span::new(0, 0, 0),
        }
    }
}

pub struct ParsedProgram {
    pub functions: Vec<ParsedFunction>,
    pub span: Span,
}
impl ParsedProgram {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            span: Span::new(0, 0, 0),
        }
    }
}

#[derive(Debug)]
pub enum ParsedStatement {
    Return(ParsedExpression, Span),
}

impl ParsedStatement {
    pub fn span(&self) -> Span {
        match self {
            ParsedStatement::Return(_, span) => span.clone(),
        }
    }
}

#[derive(Debug)]
pub enum ParsedExpression {
    Identifier(String, Span),
    NumericLiteral(NumericConstant, Span),
    BooleanLiteral(bool, Span),
    StringLiteral(String, Span),
}

impl ParsedExpression {
    pub fn span(&self) -> Span {
        match self {
            ParsedExpression::Identifier(_, span) => span.clone(),
            ParsedExpression::NumericLiteral(_, span) => span.clone(),
            ParsedExpression::BooleanLiteral(_, span) => span.clone(),
            ParsedExpression::StringLiteral(_, span) => span.clone(),
        }
    }
}
