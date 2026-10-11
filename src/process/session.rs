use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessSession {
    pub id: String,
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub start_time_seconds: u64,
    pub start_time_useconds: u64,
}
