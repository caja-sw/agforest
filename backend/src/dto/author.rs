use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub hash: String,
}
