use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub location: String,
    pub min_participants: u32,
    pub max_participants: u32,
    pub event_date: String,
}
