use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDTO {
    pub name: String,
    pub hash: String,
}
