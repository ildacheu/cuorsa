use serde::Deserialize;

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct RegisterRequest {
    pub(crate) num: String,
    pub(crate) prenum: String,
    pub(crate) email: String,
    pub(crate) agen_vehichel: Option<String>,
}
