use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventListResponse {
    pub id: String,
    pub name: String,
    pub description: String,
}
