use chrono::{DateTime, Local};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum LogEvent {
    Connect { server: String },
    ZoneChange { zone: String },
    GenerateZone { zone_level: i8, zone_name: String },
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Log {
    pub dt: DateTime<Local>,
    pub event: LogEvent,
}
