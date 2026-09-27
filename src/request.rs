use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub enum Condition {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

#[derive(Debug)]
pub struct Request {
    pub request_number: u32,
    pub user_name: String,
    pub query: String,
    pub location: Location,
    pub time: DateTime<Utc>,
    pub condition: Condition,
}
