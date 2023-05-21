use std::error::Error;

const MSG_EXIN: &str = "exhaustive input"; 
const _MSG_EXSTR: &str = "exhaustive string";

const MSG_INVALID_AFTER_IDENT: &str = "invalid character after ident";
const MSG_INVALID_AFTER_INT: &str = "invalid character after int-literal";

pub const MSG_MISSING_OPEN_PARENTHESIS: &str = "missing opening parenthesis '('";
pub const MSG_MISSING_CLOSE_PARENTHESIS: &str = "missing opening parenthesis ')'";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Identifier(String),
    IntLiteral(i32),
    _StrLiteral(String),
    
    OpenParenthesis,
    CloseParenthesis
}

/// Token = type: Type, char: usize, line: usize, file_id: usize
#[derive(Debug, Clone)]
pub struct Token(pub Type, pub usize, pub usize, pub usize);

/// ErrorToken = msg: String, char: usize, line: usize, file_id: usize
#[derive(Debug, Clone)]
pub struct ErrorToken(pub String, pub usize, pub usize, pub usize);

pub struct Lexer {
    index: usize,
    file_index: usize,
    /// text: String, filename: String
    pub input: Vec<(String, String)>,
    line: usize,
    position: usize,
    pub tokens: Vec<Token>
}

trait Extchar {
    fn is_ident(&self) -> bool;
    fn valid_after_ident(&self) -> bool;
    fn valid_after_int(&self) -> bool;
}

impl Extchar for char {
    fn is_ident(&self) -> bool {
        match self {
            '_' |
            'a'..='z'|
            'A'..='Z' => true,
            '0'..='9' => true,
            _ => false
        }
    }

    fn valid_after_ident(&self) -> bool {
        match self {
            '(' | ')' => true,
            ' ' | '\n' | '\t' | '\0' => true,
            _ => false
        }
    }

    fn valid_after_int(&self) -> bool {
        match self {
            '(' | ')' => true,
            ' ' | '\n' | '\t' | '\0' => true,
            _ => false
        }
    }
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {index: 0, file_index: 0, input: vec![], line: 0, position: 0, tokens: vec![]}
    }

    //
    fn build_error(&self, msg: String) -> ErrorToken {
        ErrorToken {0: msg, 1: self.index, 2: self.line, 3: self.file_index}
    }
    //




    pub fn _feed(&mut self, input: String) {
        self.input[self.file_index].1.push_str(input.as_str());
    }

    pub fn feed_file(&mut self, input: (String, String)) {
        self.input.push(input);
    }

    fn next_char(&mut self, peek: bool, peek_by: usize) -> Option<char> {
        if peek {
            self.index += peek_by; self.position += peek_by;
            self.input.get(self.file_index)?.0.chars().nth(self.index)
        }
        else {
            self.input.get(self.file_index)?.0.chars().nth(self.index + peek_by)
        }
    }

    fn get_token(&mut self) -> Result<Token, ErrorToken> {
        match self.next_char(false, 0).unwrap_or('\0') {
            // todo: hex and float
            '0'..='9' => {
                let mut int = self.next_char(false, 0).unwrap_or('\0').to_string();
                while self.next_char(false, 1).unwrap_or('\0').is_numeric() {
                    int.push(self.next_char(true, 1).unwrap_or('\0'));
                }
                if !self.next_char(true, 1).unwrap_or('\0').valid_after_int() {
                    return Err(self.build_error(MSG_INVALID_AFTER_INT.to_string()));
                }
                Ok(Token(Type::IntLiteral(int.parse().unwrap()), self.position, self.line, self.file_index))
            }

            '_' | 'a'..='z' | 'A'..='Z' => {
                let mut ident = self.next_char(false, 0).unwrap_or('\0').to_string();
                while self.next_char(false, 1).unwrap_or('\0').is_ident() {
                    ident.push(self.next_char(true, 1).unwrap_or('\0'));
                }
                if !self.next_char(false, 1).unwrap_or('\0').valid_after_ident() {
                    return Err(self.build_error(MSG_INVALID_AFTER_IDENT.to_string()));
                }
                Ok(Token(Type::Identifier(ident), self.position, self.line, self.file_index))
            }

            '(' => Ok(Token(Type::OpenParenthesis, self.position, self.line, self.file_index)),
            ')' => Ok(Token(Type::CloseParenthesis, self.position, self.line, self.file_index)),

            // TODO! Unknown token
            _ => Err(self.build_error(MSG_EXIN.to_string()))
        }
    }

    pub fn lex(&mut self) -> Result<(), ErrorToken> {
        for _x in self.input.clone() {
            self.index = 0;

            while self.index < self.input[self.file_index].0.len() {
                if self.next_char(false, 0).ok_or(self.build_error(MSG_EXIN.to_string()))? == '\n' {
                    self.line += 1;
                    self.position = 1;
                }
                else if !self.next_char(false, 0).ok_or(self.build_error(MSG_EXIN.to_string()))?.is_whitespace() {
                    let token = self.get_token()?;
                    self.tokens.push(token);
                }
                
                self.position += 1;
                self.index += 1;
            }

            self.file_index += 1;
        }

        Ok(())
    }
}