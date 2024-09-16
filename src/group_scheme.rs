use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    pub name: String,
    pub icon_path: Option<String>,
    pub theme: String,
}
