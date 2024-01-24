use std::str::Chars;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Str(String),
    Number(f64),
    Colon,
    Comma,
    Bool(bool),
    Null,
    Unknown(String),
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
    current_pos: u64,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        let input = source.chars();
        Self::from_chars(input)
    }

    pub fn from_chars(mut input: Chars) -> Lexer {
        let current = input.next();
        Lexer {
            input,
            current,
            current_pos: 0,
        }
    }

    fn current(&mut self) -> Option<char> {
        self.current
    }

    fn advance(&mut self) -> Option<char> {
        self.current = self.input.next();
        self.current_pos += 1;
        self.current
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(c) = self.current() {
            let token = match c {
                '{' => {
                    self.advance();
                    Token::LBrace
                }
                '}' => {
                    self.advance();
                    Token::RBrace
                }
                '[' => {
                    self.advance();
                    Token::LBracket
                }
                ']' => {
                    self.advance();
                    Token::RBracket
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                ':' => {
                    self.advance();
                    Token::Colon
                }
                '"' => self.lex_string(),
                '+' | '-' | '0'..='9' => self.lex_number(),
                _ if c.is_alphabetic() => self.lex_keywords(),
                _ => {
                    let mut str = String::new();
                    str.push(c);
                    Token::Unknown(str)
                }
            };

            Some(token)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn lex_keywords(&mut self) -> Token {
        let mut id = String::new();
        while let Some(c) = self.current() {
            if c.is_alphanumeric() {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }
        match id.as_str() {
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "null" => Token::Null,
            _ => Token::Unknown(id),
        }
    }

    fn consume_int_string(&mut self) -> Option<String> {
        let mut int_str = String::new();

        while let Some(c) = self.current() {
            match c {
                '0'..='9' => {
                    int_str.push(c);
                    self.advance();
                }
                _ => break,
            }
        }

        Some(int_str)
    }

    fn lex_number(&mut self) -> Token {
        let mut num_str = String::new();

        if self.current() == Some('-') {
            num_str.push('-');
            self.advance();
        } else if self.current() == Some('+') {
            self.advance();
        }

        {
            let s = self
                .consume_int_string()
                .unwrap_or_else(|| panic!("Invalid number"));
            num_str += &s;
        }

        if let Some('.') = self.current() {
            self.advance();
            let s = self
                .consume_int_string()
                .unwrap_or_else(|| panic!("Invalid fraction"));
            num_str.push('.');
            num_str += &s;
        }

        if matches!(self.current(), Some('e') | Some('E')) {
            num_str.push('e');
            match self.advance() {
                Some('-') => {
                    num_str.push('-');
                    self.advance();
                }
                Some('+') => {
                    self.advance();
                }
                _ => {}
            };
            let s = self
                .consume_int_string()
                .unwrap_or_else(|| panic!("Invalid exponent"));
            num_str += &s;
        }
        Token::Number(num_str.parse::<f64>().unwrap())
    }

    fn handle_escape_char(&mut self) -> Option<char> {
        match self.current() {
            Some('"') => Some('"'),
            Some('\\') => Some('\\'),
            Some('/') => Some('/'),
            Some('b') => Some('\u{0008}'),
            Some('f') => Some('\u{000c}'),
            Some('n') => Some('\n'),
            Some('r') => Some('\r'),
            Some('t') => Some('\t'),
            Some('u') => unimplemented!(),
            c => c,
        }
    }

    fn lex_string(&mut self) -> Token {
        let mut string = String::new();
        self.advance();

        while let Some(c) = self.current() {
            match c {
                '"' => break,
                '\\' => {
                    self.advance();
                    string.push(
                        self.handle_escape_char()
                            .unwrap_or_else(|| panic!("Unclosed string")),
                    );
                }
                _ => {
                    self.advance();
                    string.push(c)
                }
            }
        }
        if let None = self.current() {
            panic!("Unclosed string");
        }
        self.advance();
        Token::Str(string)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_json_lexer_simple() {
        let input = r#"
            {
                "name": "John",
                "age": 30,
                "is_student": false,
                "grades": [90.5, 85.2, 88.8],
                "address": {
                    "city": "New York",
                    "zip": "10001"
                },
                "is_null": null
            }
        "#;

        let expected_tokens = vec![
            Token::LBrace,
            Token::Str("name".to_string()),
            Token::Colon,
            Token::Str("John".to_string()),
            Token::Comma,
            Token::Str("age".to_string()),
            Token::Colon,
            Token::Number(30.0),
            Token::Comma,
            Token::Str("is_student".to_string()),
            Token::Colon,
            Token::Bool(false),
            Token::Comma,
            Token::Str("grades".to_string()),
            Token::Colon,
            Token::LBracket,
            Token::Number(90.5),
            Token::Comma,
            Token::Number(85.2),
            Token::Comma,
            Token::Number(88.8),
            Token::RBracket,
            Token::Comma,
            Token::Str("address".to_string()),
            Token::Colon,
            Token::LBrace,
            Token::Str("city".to_string()),
            Token::Colon,
            Token::Str("New York".to_string()),
            Token::Comma,
            Token::Str("zip".to_string()),
            Token::Colon,
            Token::Str("10001".to_string()),
            Token::RBrace,
            Token::Comma,
            Token::Str("is_null".to_string()),
            Token::Colon,
            Token::Null,
            Token::RBrace,
        ];

        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            if let Some(token) = lexer.next_token() {
                assert_eq!(token, expected_token);
            } else {
                panic!("Expected more tokens but lexer reached the end.");
            }
        }
        assert_eq!(lexer.next_token(), None);
    }
}
