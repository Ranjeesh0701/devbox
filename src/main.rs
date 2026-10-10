use std::env;
use std::process::{Command, ExitCode};
use std::fmt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

enum DevboxError {
    InvalidArguments(String),
    ProcessLaunch(std::io::Error),
    FileOperation(std::io::Error),
    Json(serde_json::Error),
    ExitCodeUnavailable,
    ExitCodeOutOfRange(i32),
}

#[derive(Serialize, Deserialize)]
struct ProcessSession {
    id: String,
    pid: u32,
    name: String,
    command: String,
}

impl fmt::Display for DevboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DevboxError::InvalidArguments(message) => {
                write!(f, "{message}")
            }
            DevboxError::ProcessLaunch(error) => {
                write!(f, "Failed to start process: {error}")
            }
            DevboxError::ExitCodeUnavailable => {
                write!(f, "Process terminated without a normal exit code")
            }
            DevboxError::ExitCodeOutOfRange(code) => {
                write!(f, "Cannot represent child exit code: {code}")
            }
            DevboxError::FileOperation(error) => {
                write!(f, "File operation failed: {error}")
            }
            DevboxError::Json(error) => {
                write!(f, "JSON operation failed: {error}")
            }
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<u8, DevboxError> {
    let mut args = env::args();

    args.next();

    let mut sessions: Vec<ProcessSession> = if Path::new("sessions.json").exists() {
        get_sessions()?
    } else {
        Vec::new()
    };

    let action = match args.next() {
        Some(a) => a,
        None => {
            return Err(DevboxError::InvalidArguments("No action specified. Usage: <action> <program> [args...]".to_string()));
        },
    };

    if action != "start" && action != "list" {
        return Err(DevboxError::InvalidArguments(format!("Invalid action: {}. Only 'start' and 'list' is supported.", action)));
    }

    if action == "start" {
        let name = match args.next() {
            Some(n) => n,
            None => {
                return Err(DevboxError::InvalidArguments("No name specified. Usage: <action> <name> <program> [args...]".to_string()));
            }
        };
        let program = match args.next() {
            Some(p) => p,
            None => {
                return Err(DevboxError::InvalidArguments("No process specified. Usage: <action> <name> <program> [args...]".to_string()));
            },
        };
        let args: Vec<String> = args.collect();

        let child = Command::new(&program)
            .args(args)
            .spawn()
            .map_err(DevboxError::ProcessLaunch)?;

        let pid = child.id();
        let session = ProcessSession {
            id: pid.to_string(),
            pid: pid,
            name: name,
            command: program,
        };

        println!("Started {} with PID {}", session.name, session.pid);
        sessions.push(session);

        let json = serde_json::to_string(&sessions)
            .map_err(DevboxError::Json)?;

        fs::write("sessions.json", &json)
            .map_err(DevboxError::FileOperation)?;
    } else if action == "list" {
        sessions = remove_inactive_sessions()?;

        if sessions.is_empty() {
            println!("No active sessions found.");
        } else {
            for session in sessions {
                println!("{}: {} (PID: {})", session.id, session.name, session.pid);
            }
        }
    }

    Ok(0)
}

fn get_sessions() -> Result<Vec<ProcessSession>, DevboxError> {
    let text = fs::read_to_string("sessions.json")
        .map_err(DevboxError::FileOperation)?;
    let sessions: Vec<ProcessSession> = serde_json::from_str(&text)
        .map_err(DevboxError::Json)?;

    Ok(sessions)
}

fn remove_inactive_sessions() -> Result<Vec<ProcessSession>, DevboxError> {
    let mut sessions = get_sessions()?;

    sessions.retain(|session| is_process_active(session.pid));

    let json = serde_json::to_string(&sessions)
        .map_err(DevboxError::Json)?;

    fs::write("sessions.json", &json)
        .map_err(DevboxError::FileOperation)?;

    Ok(sessions)
}

fn is_process_active(pid: u32) -> bool {
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
