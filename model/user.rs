#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct User {
    id: String,
    first_name: String,
    last_name: String,
}
