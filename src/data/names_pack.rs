use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NamesPack {
    pub id: String,
    pub first_names_male: Vec<String>,
    pub first_names_female: Vec<String>,
    pub last_names_male: Vec<String>,
    #[serde(default)]
    pub last_names_female: Vec<String>,
}
