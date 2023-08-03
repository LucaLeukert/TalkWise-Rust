use crate::compiler::FileId;
use crate::lexer::{Token, TokenContents};
use crate::parsed_types::{ParsedBlock, ParsedExpression, ParsedFunction, ParsedProgram, ParsedStatement};

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

    pub fn parse(&mut self) -> ParsedProgram {
        let mut program = ParsedProgram::new();
        println!("DEBUG: Parsing file: {}", self.file_name);

        while !self.eof() {
            let token = self.current();

            match token {
                Token { contents: TokenContents::Fun, span: _ } => {
                    let function = self.parse_function();
                    println!("{:?}", function);

                    program.functions.push(function);
                }
                _ => {
                    panic!("DEBUG: Unknown token: {:?}", token);
                }
            }
        }

        return program;
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
            block.statements.push(self.parse_statement(self.tokens[self.position].clone()));
            self.advance();
        }

        self.match_token(TokenContents::RBrace);

        return block;
    }

    fn parse_statement(&mut self, current: Token) -> ParsedStatement {
        println!("DEBUG: Parsing statement: {:?}", current);

        return match current {
            Token { contents: TokenContents::Return, span } => {
                self.advance();
                let expression = self.parse_expression();
                let merged_span = span.merge(&expression.span().clone());

                ParsedStatement::Return(expression, merged_span)
            }
            _ => panic!("Unknown statement type"),
        };
    }

    fn parse_expression(&mut self) -> ParsedExpression {
        return match &self.current().contents {
            TokenContents::Number(numeric) => ParsedExpression::NumericLiteral(numeric.clone(), self.advance().span.clone()),
            _ => panic!("Unknown expression"),
        };
    }

    fn match_token(&mut self, token: TokenContents) {
        println!("DEBUG: Matching token: {:?}, current {:?}", token, self.current());
        if self.current().contents == token {
            self.advance();
            return;
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