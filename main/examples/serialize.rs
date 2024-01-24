use yi_json::{JsonSerializable, JsonDeserializable, JsonValue};

#[derive(Debug, PartialEq, JsonSerializable, JsonDeserializable)]
struct Person {
    pub name: String,
    pub age: u32,
    pub is_student: bool,
}

fn main() {
    // Creating a Person instance
    let person = Person {
        name: "Alice".to_string(),
        age: 28,
        is_student: false,
    };

    // Serializing the Person instance to JSON
    let json_representation = person.to_json();

    // Displaying the serialized JSON
    println!("Serialized JSON: {:?}", json_representation);

    // Deserializing JSON back to a Person instance
    let deserialized_person = Person::from_json(&json_representation);

    // Displaying the deserialized Person instance
    match deserialized_person {
        Some(person) => println!("Deserialized Person: {:?}", person),
        None => println!("Failed to deserialize JSON to Person"),
    }
}