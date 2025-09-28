struct HttpInput {
    endpoint: String,
    method: String,
    codec: String,
}

struct HttpOutput {
    endpoint: String,
    method: String,
    default_headers: Vec<String>,
}
