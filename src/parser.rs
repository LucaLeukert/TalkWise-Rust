use crate::compiler::FileId;
use crate::lexer::{Span, Token, TokenContents};

#[derive(Debug)]
pub struct ParsedBlock {
    pub statements: Vec<ParsedStatement>,
    pub span: Span,
}
impl ParsedBlock {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            span: Span::new(0, 0, 0)
        }
    }
    pub fn new_with_span(span: Span) -> Self {
        Self {
            statements: Vec::new(),
            span
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
    Return,
}

pub struct Parser {
    pub file_id: FileId,
    pub file_name: String,
    pub position: usize,
    pub tokens: Vec<Token>,
}
impl Parser {
    pub fn new(file_id: FileId, file_name: String, tokens: Vec<Token>) -> Self {
        Self {
            file_id,
            file_name,
            position: 0,
            tokens,
        }
    }

    pub fn parse(&mut self) {
        println!("DEBUG: Parsing file: {}", self.file_name);

        while !self.eof() {
            let token = self.current();

            match token {
                Token { contents: TokenContents::Fun, span: _} => {
                    println!("{:?}", self.parse_function());
                }
                _ => {
                    println!("DEBUG: Unknown token: {:?}", token);
                    self.advance();
                }
            }
        }
    }

    fn parse_function(&mut self) -> ParsedFunction {
        let mut function = ParsedFunction::new();

        self.match_token(TokenContents::Fun);
        match self.current() {
            Token { contents: TokenContents::Identifier(name), span } => {
                function.name = name.clone();
                function.name_span = span.clone();
                self.advance();
            }
            _ => panic!("Expected identifier"),
        }

        self.match_token(TokenContents::LParen);

        let mut params = Vec::new();
        let mut current_param_requires_label = true;
        let mut current_param_is_mutable = false;

        while !self.eof() {
            match &self.current().contents {
                TokenContents::RParen => {
                    self.advance();
                    break;
                }
                TokenContents::Comma => {
                    current_param_requires_label = true;
                    self.advance();
                }
                TokenContents::Mut => {
                    current_param_is_mutable = true;
                    self.advance();
                }
                TokenContents::Identifier(name) => {
                    let name_copy = name.clone();
                    self.advance();
                    self.match_token(TokenContents::Colon);
                    self.match_token(TokenContents::Identifier(String::from("i32")));

                    params.push((name_copy, current_param_requires_label, current_param_is_mutable));
                }
                _ => panic!("Expected identifier"),
            }
        }

        self.match_token(TokenContents::ThinArrow);
        self.match_token(TokenContents::Identifier(String::from("i32")));
        println!("{:?}", self.current());

        function.block = self.parse_block();
        return function;
    }

    fn parse_block(&mut self) -> ParsedBlock {
        let mut block = ParsedBlock::new();
        self.match_token(TokenContents::LBrace);

        while !self.eof() && self.current().contents != TokenContents::RBrace {
            let statement = match self.current().contents {
                TokenContents::Return => ParsedStatement::Return,
                _ => panic!("Unknown statement"),
            };

            block.statements.push(statement);
            self.advance();
        }

        self.match_token(TokenContents::RBrace);

        return block;
    }

    fn match_token(&mut self, token: TokenContents) {
        println!("DEBUG: Matching token: {:?}, current {:?}", token, self.current());
        if self.current().contents == token {
            self.advance();
            return
        }

        //FIXME: This should be an error
        panic!("Expected token {:?} but found {:?}", token, self.current());
    }

    fn eof(&self) -> bool {
        self.current().contents == TokenContents::EOF
    }

    fn advance(&mut self) -> &Token {
        self.position += 1;
        self.previous()
    }

    fn previous(&self) -> &Token {
        if self.position == 0 {
            return &self.tokens[0];
        }

        &self.tokens[self.position - 1]
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position + 1]
    }

    fn peek_with_offset(&self, offset: usize) -> &Token {
        if self.position + offset >= self.tokens.len() {
            return &self.tokens[self.tokens.len() - 1];
        }

        &self.tokens[self.position + offset]
    }
}