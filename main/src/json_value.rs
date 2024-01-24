#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl JsonValue {
    pub fn new_obj() -> JsonValue {
        JsonValue::Object(Vec::new())
    }

    pub fn get_arr<'a>(&'a self, index: usize) -> Option<&'a JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }
    pub fn get<'a>(&'a self, index: &str) -> Option<&'a JsonValue> {
        match self {
            JsonValue::Object(obj) => {
                for (name, val) in obj {
                    if name == index {
                        return Some(val);
                    }
                }
                None
            }
            _ => None,
        }
    }
}
