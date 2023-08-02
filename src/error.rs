use crate::lexer::Span;

#[derive(Debug)]
pub enum TalkError {
    IOError(std::io::Error),
    ParserError(String, Span),
    ValidationError(String, Span),
    TypecheckError(String, Span),
}

impl From<std::io::Error> for TalkError {
    fn from(x: std::io::Error) -> Self {
        TalkError::IOError(x)
    }
}
