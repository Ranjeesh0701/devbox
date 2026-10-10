use std::{env, io};
use std::process::{Command, ExitCode};
use std::fmt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

enum DevboxError {
    InvalidArguments(String),
    ProcessLaunch(std::io::Error),
    ProcessNotFound(std::io::Error),
    FileOperation(std::io::Error),
    ProcessCheck(std::io::Error),
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
            DevboxError::ProcessCheck(error) => {
                write!(f, "Failed to check process: {error}")
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
            DevboxError::ProcessNotFound(error) => {
                write!(f, "Process not found: {error}")
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

    if action != "start" && action != "list" && action != "stop" {
        return Err(DevboxError::InvalidArguments(format!("Invalid action: {}. Only 'start', 'list' and 'stop' are supported.", action)));
    }

    if action == "start" {
        let name = match args.next() {
            Some(n) => n,
            None => {
                return Err(DevboxError::InvalidArguments("No name specified. Usage: start <name> <program> [args...]".to_string()));
            }
        };
        let program = match args.next() {
            Some(p) => p,
            None => {
                return Err(DevboxError::InvalidArguments("No process specified. Usage: start <name> <program> [args...]".to_string()));
            },
        };
        let args: Vec<String> = args.collect();

        let child = Command::new(&program)
            .args(args)
            .spawn()
            .map_err(DevboxError::ProcessLaunch)?;

        let pid = child.id();
        let session = ProcessSession {
            id: Uuid::new_v4().to_string(),
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
    } else if action == "stop" {
        let session_id = match args.next() {
            Some(id) => id,
            None => {
                return Err(DevboxError::InvalidArguments("No session ID specified. Usage: stop <session_id>".to_string()));
            }
        };

        let session = get_session(&session_id)?;
        match terminate_session(session.pid) {
            Ok(true) => {
                println!("Sent termination signal to {} (PID {})", session.name, session.pid);
            }
            Ok(false) => {
                println!("Process {} (PID {}) is already gone", session.name, session.pid);
            }
            Err(e) => {
                return  Err(DevboxError::ProcessCheck(e))
            }
        }
        remove_session(session.id)?;
    }

    Ok(0)
}

fn get_session(id: &str) -> Result<ProcessSession, DevboxError> {
    let sessions = get_sessions()?;
    sessions.into_iter()
        .find(|s| s.id == id)
        .ok_or_else(|| {
            DevboxError::ProcessNotFound(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Session with ID {} not found", id),
            ))
    })
}

fn get_sessions() -> Result<Vec<ProcessSession>, DevboxError> {
    let text = fs::read_to_string("sessions.json")
        .map_err(DevboxError::FileOperation)?;
    let sessions: Vec<ProcessSession> = serde_json::from_str(&text)
        .map_err(DevboxError::Json)?;

    Ok(sessions)
}

fn remove_session(id: String) -> Result<(), DevboxError> {
    let mut sessions = get_sessions()?;
    sessions.retain(|s| s.id != id);

    let json = serde_json::to_string(&sessions)
        .map_err(DevboxError::Json)?;

    fs::write("sessions.json", &json)
        .map_err(DevboxError::FileOperation)?;

    Ok(())
}

fn terminate_session(pid: u32) -> Result<bool, std::io::Error> {
    let pid: i32 = i32::try_from(pid)
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "pid out of range",
            )
        })?;

    let result = unsafe { libc::kill(pid, libc::SIGTERM) };

    if result == 0 {
        return Ok(true);
    }

    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::ESRCH) => Ok(false),
        _ => Err(err),
    }
}

fn remove_inactive_sessions() -> Result<Vec<ProcessSession>, DevboxError> {
    let sessions = get_sessions()?;

    let mut active_sessions = Vec::new();

    for session in sessions {
        if is_process_active(session.pid)
            .map_err(DevboxError::ProcessCheck)?
        {
            active_sessions.push(session);
        }
    }

    let json = serde_json::to_string(&active_sessions)
        .map_err(DevboxError::Json)?;

    fs::write("sessions.json", &json)
        .map_err(DevboxError::FileOperation)?;

    Ok(active_sessions)
}

fn is_process_active(pid: u32) -> Result<bool, std::io::Error> {
    let pid = i32::try_from(pid)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "pid out of range"))?;

    let result = unsafe { libc::kill(pid, 0) };

    if result == 0 {
        return Ok(true);
    }

    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::ESRCH) => Ok(false),
        Some(libc::EPERM) => Ok(true),
        _ => Err(err),
    }
}
