use crate::lexer::*;

pub struct Parser {
    index: usize,
    pub lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {index: 0, lexer: lexer}
    }

    fn next_token(&mut self, peek: bool, peek_by: usize) -> Option<&Token> {
        if peek {
            self.index += peek_by;
            self.lexer.tokens.get(self.index)
        }
        else {
            self.lexer.tokens.get(self.index + peek_by)
        }
    }

    pub fn start_parsing(&mut self) -> Result<(), String> {
        while self.index < self.lexer.tokens.len() {
            self.parse()?;
            self.index += 1;
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), String> {
        match self.lexer.tokens.get(self.index).unwrap().0.clone() {
            // TODO!!
            //        organize into functions
            // ------------------------------

            Type::IntLiteral(int) => {
                println!("push {int}");
            },

            Type::Identifier(ident) => {
                if self.next_token(true, 1).unwrap().0 != Type::OpenParenthesis {
                    return Err(MSG_MISSING_OPEN_PARENTHESIS.to_string());
                }

                let mut i = 0;
                while self.next_token(true, 1).unwrap().0 != Type::CloseParenthesis {
                    i += 1;
                    self.parse();
                }

                println!("call {ident}");
                println!("sub %sp, {i}");
            },

            _ => {}
        }

        Ok(())
    }
}