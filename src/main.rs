use std::env;
use std::fs;
use std::process::ExitCode;

use devbox::error::DevboxError;
use devbox::process::supervisor::ProcessSupervisor;
use devbox::storage::ensure_storage_directory;

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
    let mut supervisor = ProcessSupervisor::new();

    args.next();

    let storage_directory = ensure_storage_directory()?;
    let temp_file = storage_directory.join(devbox::storage::SESSIONS_TMP);

    if temp_file.exists() {
        fs::remove_file(&temp_file).map_err(DevboxError::FileOperation)?;
    }

    let action = match args.next() {
        Some(a) => a,
        None => {
            return Err(DevboxError::InvalidArguments(
                "No action specified. Usage: <action> <program> [args...]".to_string(),
            ));
        }
    };

    if action != "start" && action != "list" && action != "stop" {
        return Err(DevboxError::InvalidArguments(format!(
            "Invalid action: {}. Only 'start', 'list' and 'stop' are supported.",
            action
        )));
    }

    if action == "start" {
        let name = match args.next() {
            Some(n) => n,
            None => {
                return Err(DevboxError::InvalidArguments(
                    "No name specified. Usage: start <name> <program> [args...]".to_string(),
                ));
            }
        };
        let program = match args.next() {
            Some(p) => p,
            None => {
                return Err(DevboxError::InvalidArguments(
                    "No process specified. Usage: start <name> <program> [args...]".to_string(),
                ));
            }
        };
        let args: Vec<String> = args.collect();

        supervisor.start(name, program, args)?;
    } else if action == "list" {
        let sessions = supervisor.list()?;

        if sessions.is_empty() {
            println!("No active sessions found.");
        } else {
            for session in sessions {
                println!(
                    "{}: {} (PID: {})\nCommand: {}",
                    session.id, session.name, session.pid, session.command
                );
            }
        }
    } else if action == "stop" {
        let session_id = match args.next() {
            Some(id) => id,
            None => {
                return Err(DevboxError::InvalidArguments(
                    "No session ID specified. Usage: stop <session_id>".to_string(),
                ));
            }
        };

        supervisor.stop(&session_id)?;
    }

    Ok(0)
}
