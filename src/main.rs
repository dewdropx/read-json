use serde::{Deserialize, Serialize}; // serialization and deserialization

// create the required structs
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {

    // sample json text
    let json = r#"
    {
        "article": "how to work with json in rust",
        "author": "arjun",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of the paragraph"
            },
            {
                "name": "end of the paragraph"
            }
        ]
    }"#;

    // read the json text according to requirements
    let parsed: Article = read_json_typed(json);
    println!("\n\n the name of the first paragraph is: {}", parsed.paragraph[0].name);
}

// function to read json using serde crate
fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}