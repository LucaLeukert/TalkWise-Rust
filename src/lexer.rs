use crate::compiler::{FileId, NumericConstant};

#[derive(Clone, Debug)]
pub struct Span {
    pub file_id: FileId,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(file_id: FileId, start: usize, end: usize) -> Self {
        Self {
            file_id,
            start,
            end,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenContents {
    Identifier(String),
    Number(NumericConstant),
    String(String),

    Fun,
    Let,
    If,
    Else,
    While,
    For,
    Return,
    True,
    False,

    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Plus,
    Minus,
    Equal,
    Comma,
    Mut,
    Colon,
    ThinArrow,
    
    Garbage,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub contents: TokenContents,
    pub span: Span,
}

impl Token {
    pub fn new(contents: TokenContents, span: Span) -> Self {
        Self { contents, span }
    }

    pub fn unknown(span: Span) -> Self {
        Self {
            contents: TokenContents::Garbage,
            span,
        }
    }
}

pub struct Lexer {
    pub file_id: FileId,
    pub file_name: String,
    pub file_contents: Vec<u8>,
    pub position: usize,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(file_id: FileId, file_name: String, file_contents: Vec<u8>) -> Self {
        Self {
            file_id,
            file_name,
            file_contents,
            position: 0,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.position < self.file_contents.len() {
            self.skip_whitespace();

            let token = self.lex_token();
            self.tokens.push(token);
            self.advance();
        }

        self.tokens.push(Token::new(TokenContents::EOF, Span::new(self.file_id, self.position, self.position)));

        return self.tokens.clone();
    }

    fn lex_token(&mut self) -> Token {
        return match self.current() {
            b';' => Token::new(TokenContents::Semicolon, Span::new(self.file_id, self.position, self.position + 1)),
            b'(' => Token::new(TokenContents::LParen, Span::new(self.file_id, self.position, self.position + 1)),
            b')' => Token::new(TokenContents::RParen, Span::new(self.file_id, self.position, self.position + 1)),
            b'{' => Token::new(TokenContents::LBrace, Span::new(self.file_id, self.position, self.position + 1)),
            b'}' => Token::new(TokenContents::RBrace, Span::new(self.file_id, self.position, self.position + 1)),
            b'+' => Token::new(TokenContents::Plus, Span::new(self.file_id, self.position, self.position + 1)),
            b'-' => {
                if self.peek() == b'>' {
                    self.advance();
                    Token::new(TokenContents::ThinArrow, Span::new(self.file_id, self.position - 1, self.position + 1))
                } else {
                    Token::new(TokenContents::Minus, Span::new(self.file_id, self.position, self.position + 1))
                }  
            },
            b'=' => Token::new(TokenContents::Equal, Span::new(self.file_id, self.position, self.position + 1)),
            b',' => Token::new(TokenContents::Comma, Span::new(self.file_id, self.position, self.position + 1)),
            b':' => Token::new(TokenContents::Colon, Span::new(self.file_id, self.position, self.position + 1)),
            b'"' => {
                let mut string = String::new();
                self.advance();
                while !self.eof() && self.current() != b'"' {
                    string.push(self.current() as char);
                    self.advance();
                }
                let length = string.len();

                Token::new(TokenContents::String(string), Span::new(self.file_id, self.position + 1, self.position + length + 1))
            }
            b'0'..=b'9' => {
                let mut number = String::new();
                while !self.eof() && self.current().is_ascii_digit() {
                    number.push(self.current() as char);

                    if !self.peek().is_ascii_digit() {
                        break
                    }

                    self.advance();
                }

                let number = number.parse::<u64>().unwrap();
                Token::new(TokenContents::Number(NumericConstant::USize(number)), Span::new(self.file_id, self.position, self.position + number.to_string().len()))
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let mut identifier = String::new();
                while !self.eof() && (self.current().is_ascii_alphanumeric() || self.current() == b'_') {
                    identifier.push(self.current() as char);

                    if !self.peek().is_ascii_alphanumeric() || !self.peek() == b'_' {
                        break
                    }

                    self.advance();
                }

                self.parse_identifier(identifier)
            }
            _ => Token::unknown(Span::new(self.file_id, self.position, self.position + 1)),
        };
    }

    fn parse_identifier(&mut self, identifier: String) -> Token {
        let length = identifier.len();

        return match identifier.as_str() {
            "fun" => Token::new(TokenContents::Fun, Span::new(self.file_id, self.position, self.position + length)),
            "let" => Token::new(TokenContents::Let, Span::new(self.file_id, self.position, self.position + length)),
            "if" => Token::new(TokenContents::If, Span::new(self.file_id, self.position, self.position + length)),
            "else" => Token::new(TokenContents::Else, Span::new(self.file_id, self.position, self.position + length)),
            "while" => Token::new(TokenContents::While, Span::new(self.file_id, self.position, self.position + length)),
            "for" => Token::new(TokenContents::For, Span::new(self.file_id, self.position, self.position + length)),
            "return" => Token::new(TokenContents::Return, Span::new(self.file_id, self.position, self.position + length)),
            "true" => Token::new(TokenContents::True, Span::new(self.file_id, self.position, self.position + length)),
            "false" => Token::new(TokenContents::False, Span::new(self.file_id, self.position, self.position + length)),
            "mut" => Token::new(TokenContents::Mut, Span::new(self.file_id, self.position, self.position + length)),
            _ => Token::new(TokenContents::Identifier(identifier), Span::new(self.file_id, self.position, self.position + length)),
        };
    }

    fn skip_whitespace(&mut self) {
        while !self.eof() && self.current().is_ascii_whitespace() {
            self.advance();
        }
    }

    fn eof(&self) -> bool {
        self.position >= self.file_contents.len()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn current(&self) -> u8 {
        self.file_contents[self.position]
    }

    fn peek(&self) -> u8 {
        self.file_contents[self.position + 1]
    }

    fn peek_with_offset(&self, offset: usize) -> u8 {
        self.file_contents[self.position + offset]
    }
}