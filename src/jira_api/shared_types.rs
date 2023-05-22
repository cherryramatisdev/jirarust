use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Status {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub fields: Field,
}
