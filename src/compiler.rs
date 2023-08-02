use std::{fs, path::Path};
use crate::error::TalkError;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub type FileId = usize;

pub fn file_name_to_id(file_name: &str) -> FileId {
    let mut hash: i32 = 0;
    for c in file_name.chars() {
        hash = hash.wrapping_mul(31).wrapping_add(c as usize as i32);
    }
    return hash as FileId;
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumericConstant {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(u64),
}

pub struct Compiler {
    raw_files: Vec<(String, Vec<u8>)>
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            raw_files: Vec::new()
        }
    }

    pub fn add_file(&mut self, file_name: &Path) -> Result<bool, TalkError> {
        let exists = file_name.exists();
        if !exists {
            return Err(TalkError::IOError(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found.")));
        }

        let file_contents = fs::read(file_name).unwrap();
        self.raw_files.push((file_name.to_string_lossy().to_string(), file_contents));

        return Ok(true);
    }

    pub fn compile(&mut self) -> Result<String, TalkError> {
        for (file_name, file_contents) in &self.raw_files {
            let mut lexer = Lexer::new(file_name_to_id(file_name), file_name.clone(), file_contents.clone());
            let tokens = lexer.lex();

            let mut parser = Parser::new(lexer.file_id, file_name.clone(), tokens);
            parser.parse();
        }

        return Ok(String::new());
    }
}