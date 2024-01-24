pub mod json_value;
pub mod lexer;
pub mod parser;
pub use macros::{JsonDeserializable, JsonSerializable};
// mod macros;

pub use json_value::JsonValue;

pub trait JsonSerializable {
    fn to_json(&self) -> JsonValue;
}
pub trait JsonDeserializable {
    fn from_json(value: &JsonValue) -> Option<Self>
    where
        Self: Sized;
}

impl JsonSerializable for String {
    fn to_json(&self) -> JsonValue {
        JsonValue::String(self.clone())
    }
}

impl JsonDeserializable for String {
    fn from_json(value: &JsonValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let JsonValue::String(s) = value {
            Some(s.clone())
        } else {
            None
        }
    }
}

impl JsonSerializable for u32 {
    fn to_json(&self) -> JsonValue {
        JsonValue::Number(self.clone() as f64)
    }
}

impl JsonDeserializable for u32 {
    fn from_json(value: &JsonValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let JsonValue::Number(n) = value {
            Some(n.clone() as u32)
        } else {
            None
        }
    }
}

impl JsonSerializable for bool {
    fn to_json(&self) -> JsonValue {
        JsonValue::Bool(self.clone())
    }
}

impl JsonDeserializable for bool {
    fn from_json(value: &JsonValue) -> Option<Self>
    where
        Self: Sized,
    {
        if let JsonValue::Bool(b) = value {
            Some(b.clone())
        } else {
            None
        }
    }
}

mod tests {
    use super::*;

    #[derive(Debug, PartialEq, JsonSerializable, JsonDeserializable)]
    struct Person {
        pub name: String,
        pub age: u32,
        pub is_student: bool,
    }

    #[test]
    fn test_to_json() {
        let person = Person {
            name: "John".to_string(),
            age: 25,
            is_student: true,
        };

        let expected_json = JsonValue::Object(vec![
            ("name".to_string(), JsonValue::String("John".to_string())),
            ("age".to_string(), JsonValue::Number(25.0)),
            ("is_student".to_string(), JsonValue::Bool(true)),
        ]);

        assert_eq!(person.to_json(), expected_json);
    }

    #[test]
    fn test_from_json() {
        let json_value = JsonValue::Object(vec![
            ("name".to_string(), JsonValue::String("Jane".to_string())),
            ("age".to_string(), JsonValue::Number(30.0)),
            ("is_student".to_string(), JsonValue::Bool(false)),
        ]);

        let expected_person = Person {
            name: "Jane".to_string(),
            age: 30,
            is_student: false,
        };

        assert_eq!(Person::from_json(&json_value), Some(expected_person));
    }

    // Add more tests for different scenarios as needed
}
