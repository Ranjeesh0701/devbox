use serde::{Deserialize, Serialize};

use super::process::session::ProcessSession;

#[derive(Debug, Serialize, Deserialize)]
enum Request {
    Start {
        name: String,
        program: String,
        args: Vec<String>,
    },
    List,
    Stop {
        id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
enum Response {
    Start { session: ProcessSession },
    List { sessions: Vec<ProcessSession> },
    Stop,
    Error { message: String },
}
