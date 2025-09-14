use serde::Serialize;

#[derive(Debug, Serialize)]
struct RequestDTO {
    parameters: Vec<RequestParameter>,
    // contains_files: bool,
}

/// Represent a single parameter to be sent along with a request to the Bot API.
#[derive(Debug, Serialize)]
struct RequestParameter {
    name: String, // TODO: Convert this to enum instead of String
    value: serde_json::Value,
    // input_files: Vec<InputFile>, // TODO: add features for input file
}
