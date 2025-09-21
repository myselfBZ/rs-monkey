use crate::token;

pub struct Lexer{
   input:Vec<u8>,
   ch:u8,
   pos:usize,
   peek:usize,
}


impl Lexer {
    pub fn new(input:String) -> Self {
        let mut l = Lexer{input:input.into_bytes(), ch:0, pos:0, peek:0};
        l.read_char();
        l
    }

    fn read_char(&mut self){
        if self.peek >= self.input.len(){
            self.ch = b'\0'
        } else{
            self.ch = self.input[self.peek]
        }
        self.pos = self.peek;
        self.peek +=1;
    }


    fn skip_white(&mut self){
        while self.ch.is_ascii_whitespace(){
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token{
        self.skip_white();
        let tok = match self.ch{
            b'\0' => return token::Token::Eof,
            b'a'..=b'z' => {
                let v = self.read_word();
                match v.as_str() {
                    "let" => {
                        return token::Token::Let
                    },
                    "fn" => {
                        return token::Token::Func
                    },
                    "true" =>{ 
                       return token::Token::True
                    },
                    "false" => {
                        return token::Token::False
                    },
                    "func" => {
                       return token::Token::Func
                    },
                    "if" => {
                       return token::Token::If
                    },
                    "else" => {
                        return token::Token::Else
                    },
                    "return" => {
                        return token::Token::Return
                    },
                    _ => {
                        return token::Token::Ident(v)
                    }
                }
            },
            b';' => {
                token::Token::Semicolon
            },
            b'>' => {
                token::Token::Gt
            },
            b'<' => {
                token::Token::Lt
            }
            b'=' => {
                if self.peek == b'='.into(){
                    token::Token::Eq
                } else{
                    token::Token::Assing
                }
            },
            b',' => {
                token::Token::Comma
            },
            b'!' =>{
                if self.peek == b'='.into(){
                    token::Token::NotEq
                } else{
                    token::Token::Bang
                }
            },
            b'0'..=b'9' => {
                let number = self.read_number();
                return token::Token::Int(number)
            },
            b'{' =>{
                token::Token::Lbrace
            },
            b'}' =>{
                token::Token::Rbrace
            },

            b')' =>{
                token::Token::Rparen
            },
            b'(' =>{
                token::Token::Lparen
            },
            b'*' =>{
                token::Token::Asterisk
            },
            b'+' => {
                token::Token::Plus
            },
            b'-' => {
                token::Token::Minus
            },
            b'/' => {
                token::Token::Slash
            }
            _ => token::Token::Illgl((self.ch as char).to_string())
        };
        self.read_char();
        return tok
    }


    fn read_number(&mut self) -> String{
        let pos = self.pos;
        while self.ch.is_ascii_digit(){
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.pos]).to_string();
    }

    fn read_word(&mut self) -> String{
        let pos = self.pos;
        while self.ch.is_ascii_alphabetic(){
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.pos]).to_string();
    }

}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer; 
    use crate::token::Token;
    #[test]
    fn test_lexer(){
        let x = "let x = 21 * 23 - ;";
        let mut new_lexer = Lexer::new(String::from(x));
        let expected = [Token::Let, Token::Ident(String::from("x")), Token::Assing, Token::Int(String::from("21")), Token::Asterisk, Token::Int(String::from("23")),
        Token::Minus
        ];
        for tok in expected{
            assert_eq!(new_lexer.next_token(), tok);
        }
    }

}
