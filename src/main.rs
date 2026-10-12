use std::env;
use std::os::unix::net::UnixStream;
use std::process::ExitCode;

use devbox::error::DevboxError;
use devbox::protocol::{Request, Response, read_message, write_message};

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

    let mut stream = UnixStream::connect("/tmp/devbox.sock").map_err(DevboxError::FileOperation)?;

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
        let request = Request::Start {
            name,
            program,
            args,
        };

        write_message(&mut stream, &request).map_err(DevboxError::Ipc)?;

        let response: Response = read_message(&mut stream).map_err(DevboxError::Ipc)?;

        match response {
            Response::Start { session } => {
                println!("{session:?}");
            }
            _ => return Err(DevboxError::InvalidResponse),
        }
    } else if action == "list" {
        let request = Request::List {};
        write_message(&mut stream, &request).map_err(DevboxError::Ipc)?;
        let response: Response = read_message(&mut stream).map_err(DevboxError::Ipc)?;

        match response {
            Response::List { sessions } => {
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
            }
            _ => return Err(DevboxError::InvalidResponse),
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

        let request = Request::Stop { id: session_id };
        write_message(&mut stream, &request).map_err(DevboxError::Ipc)?;

        let response: Response = read_message(&mut stream).map_err(DevboxError::Ipc)?;

        match response {
            Response::Stop => {}
            _ => return Err(DevboxError::InvalidResponse),
        }
    }

    Ok(0)
}
