use crate::json_value::JsonValue;
use crate::lexer::{Lexer, Token};

pub struct JsonParser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> JsonParser<'a> {
    pub fn new(input: &str) -> JsonParser {
        Self::from_lexer(Lexer::new(input))
    }

    pub fn from_lexer(mut lexer: Lexer) -> JsonParser {
        let current_token = lexer.next_token();
        JsonParser {
            lexer,
            current_token,
        }
    }

    fn consume(&mut self) -> Option<Token> {
        let token = std::mem::replace(&mut self.current_token, None);
        self.current_token = self.lexer.next_token();
        token
    }

    fn peek(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }
    pub fn parse(&mut self) -> Option<JsonValue> {
        self.parse_value()
    }

    fn parse_value(&mut self) -> Option<JsonValue> {
        match self.consume() {
            Some(Token::LBrace) => self.parse_object(),
            Some(Token::LBracket) => self.parse_array(),
            Some(Token::Str(s)) => Some(JsonValue::String(s)),
            Some(Token::Number(n)) => Some(JsonValue::Number(n)),
            Some(Token::Bool(b)) => Some(JsonValue::Bool(b)),
            Some(Token::Null) => Some(JsonValue::Null),
            _ => None,
        }
    }

    fn parse_object(&mut self) -> Option<JsonValue> {
        let mut result = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::RBrace => break,
                _ => {
                    if !result.is_empty() {
                        match token {
                            Token::Comma => {
                                self.consume();
                            }
                            _ => panic!("Expected comma after object element"),
                        }
                    }
                    let id = match self.parse_value() {
                        Some(JsonValue::String(s)) => s,
                        _ => panic!("Invalid object element id"),
                    };
                    if self.consume() != Some(Token::Colon) {
                        panic!("Expected colo after element id");
                    }

                    let value = self
                        .parse_value()
                        .unwrap_or_else(|| panic!("Unexpected end before closing object"));
                    result.push((id, value));
                }
            }
        }
        self.consume();
        Some(JsonValue::Object(result))
    }

    fn parse_array(&mut self) -> Option<JsonValue> {
        let mut result = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::RBracket => break,
                _ => {
                    if !result.is_empty() {
                        match token {
                            Token::Comma => {
                                self.consume();
                            }
                            _ => panic!("Expected comma after array element"),
                        }
                    }

                    if let Some(value) = self.parse_value() {
                        result.push(value);
                    } else {
                        panic!("Unexpected end before closing array");
                    }
                }
            }
        }
        self.consume();

        Some(JsonValue::Array(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_object() {
        let json_str = r#"
            {
                "name": "John Doe",
                "age": 30,
                "is_student": false
            }
        "#;
        let mut parser = JsonParser::new(json_str);
        let json_value = parser.parse().unwrap();

        assert_eq!(
            json_value,
            JsonValue::Object(vec![
                (
                    "name".to_string(),
                    JsonValue::String("John Doe".to_string())
                ),
                ("age".to_string(), JsonValue::Number(30.0)),
                ("is_student".to_string(), JsonValue::Bool(false)),
            ])
        );
    }

    #[test]
    fn test_parse_array() {
        let json_str = "[1, 2, 3, 4, 5]";
        let mut parser = JsonParser::new(json_str);
        let json_value = parser.parse().unwrap();

        assert_eq!(
            json_value,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
                JsonValue::Number(4.0),
                JsonValue::Number(5.0),
            ])
        );
    }

    #[test]
    fn test_parse_nested_object_and_array() {
        let json_str = r#"
            {
                "name": "John Doe",
                "age": 30,
                "is_student": false,
                "grades": [90, 85, 92],
                "info": {
                    "city": "New York",
                    "country": "USA"
                }
            }
        "#;
        let mut parser = JsonParser::new(json_str);
        let json_value = parser.parse().unwrap();

        assert_eq!(
            json_value,
            JsonValue::Object(vec![
                (
                    "name".to_string(),
                    JsonValue::String("John Doe".to_string())
                ),
                ("age".to_string(), JsonValue::Number(30.0)),
                ("is_student".to_string(), JsonValue::Bool(false)),
                (
                    "grades".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::Number(90.0),
                        JsonValue::Number(85.0),
                        JsonValue::Number(92.0)
                    ])
                ),
                (
                    "info".to_string(),
                    JsonValue::Object(vec![
                        (
                            "city".to_string(),
                            JsonValue::String("New York".to_string())
                        ),
                        ("country".to_string(), JsonValue::String("USA".to_string())),
                    ])
                ),
            ])
        );
    }

    #[test]
    fn test_parse_boolean_and_null() {
        let json_str = r#"
            {
                "is_true": true,
                "is_false": false,
                "is_null": null
            }
        "#;
        let mut parser = JsonParser::new(json_str);
        let json_value = parser.parse().unwrap();

        assert_eq!(
            json_value,
            JsonValue::Object(vec![
                ("is_true".to_string(), JsonValue::Bool(true)),
                ("is_false".to_string(), JsonValue::Bool(false)),
                ("is_null".to_string(), JsonValue::Null),
            ])
        );
    }

    #[test]
    fn test_parse_complex_json() {
        let json_str = r#"
            {
                "name": "John Doe",
                "age": 30,
                "is_student": false,
                "grades": [90, 85, 92],
                "info": {
                    "city": "New York",
                    "country": "USA"
                },
                "courses": [
                    {
                        "title": "Math",
                        "credits": 3
                    },
                    {
                        "title": "History",
                        "credits": 4
                    }
                ]
            }
        "#;
        let mut parser = JsonParser::new(json_str);
        let json_value = parser.parse().unwrap();

        assert_eq!(
            json_value,
            JsonValue::Object(vec![
                (
                    "name".to_string(),
                    JsonValue::String("John Doe".to_string())
                ),
                ("age".to_string(), JsonValue::Number(30.0)),
                ("is_student".to_string(), JsonValue::Bool(false)),
                (
                    "grades".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::Number(90.0),
                        JsonValue::Number(85.0),
                        JsonValue::Number(92.0)
                    ])
                ),
                (
                    "info".to_string(),
                    JsonValue::Object(vec![
                        (
                            "city".to_string(),
                            JsonValue::String("New York".to_string())
                        ),
                        ("country".to_string(), JsonValue::String("USA".to_string())),
                    ])
                ),
                (
                    "courses".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::Object(vec![
                            ("title".to_string(), JsonValue::String("Math".to_string())),
                            ("credits".to_string(), JsonValue::Number(3.0)),
                        ]),
                        JsonValue::Object(vec![
                            (
                                "title".to_string(),
                                JsonValue::String("History".to_string())
                            ),
                            ("credits".to_string(), JsonValue::Number(4.0)),
                        ]),
                    ])
                ),
            ])
        );
    }
}
