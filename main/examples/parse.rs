use yi_json::parser::JsonParser;

fn main() {
    // Example JSON string
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

    // Create a new JsonParser instance
    let mut parser = JsonParser::new(json_str);

    // Parse the JSON string into a JsonValue
    match parser.parse() {
        Some(json_value) => {
            // Display the parsed JSON value
            println!("Parsed JSON:\n{:#?}", json_value);
        }
        None => {
            // Handle parsing errors
            eprintln!("Error parsing JSON");
        }
    }
}