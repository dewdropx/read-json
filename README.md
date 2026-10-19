# JSON Parsing in Rust Using Serde

## Overview
A simple Rust program demonstrating how to deserialize JSON data into strongly typed Rust structs using the `serde` and `serde_json` crates.

## Features
- Parses JSON text into Rust structs.
- Uses Serde's `Serialize` and `Deserialize` traits.
- Represents nested JSON objects using structs and vectors.
- Accesses specific fields from the parsed JSON data.

## How It Works
1. Defines `Paragraph` and `Article` structs to represent the JSON structure.
2. Stores sample JSON data as a raw string literal.
3. Uses `serde_json::from_str()` to deserialize the JSON into an `Article` instance.
4. Accesses and prints the name of the first paragraph.

## Usage
Run the following command from the project directory:

```bash
cargo run
```

## Sample Output
```text
the name of the first paragraph is: starting sentence
```

## Limitations
- The program uses hardcoded JSON input rather than reading from a file.
- The JSON field names and data types must match the Rust struct definitions.
