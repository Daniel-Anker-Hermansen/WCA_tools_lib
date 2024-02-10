use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub registration_open: String,
    pub registration_close: String,
    pub announced_at: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub competitor_limit: Option<u64>,
    pub cancelled_at: Option<String>,
    pub url: String,
    pub website: String,
    pub short_name: String,
    pub city: String,
    pub venue_address: String,
    pub venue_details: Option<String>,
    pub latitude_degrees: f64,
    pub longitude_degrees: f64,
    pub country_iso2: String,
    pub event_ids: Vec<String>,
    pub delegates: Vec<serde_json::Value>,
    pub organizers: Vec<serde_json::Value>,
}

impl Competition {
    pub fn from_json(json: &str) -> Vec<Competition> {
        serde_json::from_str(json).unwrap()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
