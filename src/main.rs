use std::env;
use std::io;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};

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
                "No action specified. Usage: devbox <command> [arguments...]".to_string(),
            ));
        }
    };

    if action == "--help" || action == "-h" {
        println!("Usage: devbox <command> [arguments...]");
        println!("Commands:");
        println!("  start <name> <program> [args...] - Start a process.");
        println!("  list                              - List active sessions.");
        println!("  stop <session_id>                 - Stop a process.");
        println!("  shutdown                          - Shutdown the daemon.");
        return Ok(0);
    }

    let request = match action.as_str() {
        "start" => {
            let name = args.next().ok_or_else(|| {
                DevboxError::InvalidArguments(
                    "No name specified. Usage: start <name> <program> [args...]".to_string(),
                )
            })?;

            let program = args.next().ok_or_else(|| {
                DevboxError::InvalidArguments(
                    "No program specified. Usage: start <name> <program> [args...]".to_string(),
                )
            })?;

            Request::Start {
                name,
                program,
                args: args.collect(),
            }
        }
        "list" => {
            if args.next().is_some() {
                return Err(DevboxError::InvalidArguments(
                    "No arguments expected for 'list'. Usage: list".to_string(),
                ));
            }

            Request::List
        }
        "stop" => {
            let id = args.next().ok_or_else(|| {
                DevboxError::InvalidArguments(
                    "No session ID specified. Usage: stop <session_id>".to_string(),
                )
            })?;

            if args.next().is_some() {
                return Err(DevboxError::InvalidArguments(
                    "Only one argument expected for 'stop'. Usage: stop <session_id>".to_string(),
                ));
            }

            Request::Stop { id }
        }
        "shutdown" => {
            if args.next().is_some() {
                return Err(DevboxError::InvalidArguments(
                    "No arguments expected for 'shutdown'. Usage: shutdown".to_string(),
                ));
            }

            Request::Shutdown
        }
        _ => {
            return Err(DevboxError::InvalidArguments(format!(
                "Invalid action: {action}. Supported actions: start, list, stop shutdown."
            )));
        }
    };

    let mut stream = connect_to_daemon_with_retry(5, 100)?;

    write_message(&mut stream, &request).map_err(DevboxError::Ipc)?;

    let response: Response = read_message(&mut stream).map_err(DevboxError::Ipc)?;

    match response {
        Response::Start { session } => {
            if !matches!(request, Request::Start { .. }) {
                return Err(DevboxError::InvalidResponse);
            }
            println!("{session:?}");
        }
        Response::List { sessions } => {
            if !matches!(request, Request::List) {
                return Err(DevboxError::InvalidResponse);
            }

            if sessions.is_empty() {
                println!("No active sessions found.");
            } else {
                println!("{:<40} {:<20} {:<10} COMMAND", "ID", "NAME", "PID");
                println!("{}", "-".repeat(90));

                for session in sessions {
                    println!(
                        "{:<40} {:<20} {:<10} {}",
                        session.id, session.name, session.pid, session.command
                    );
                }
            }
        }
        Response::Stop => {
            if !matches!(request, Request::Stop { .. }) {
                return Err(DevboxError::InvalidResponse);
            }
            println!("Session stopped successfully.");
        }
        Response::Shutdown => {
            if !matches!(request, Request::Shutdown) {
                return Err(DevboxError::InvalidResponse);
            }

            println!("Daemon shutdown successfully.");
        }
        Response::Error { message } => {
            return Err(DevboxError::DaemonError(message));
        }
    }

    Ok(0)
}

fn get_daemon_path() -> io::Result<PathBuf> {
    let mut path = env::current_exe()?;
    path.pop();
    path.push("daemon");

    Ok(path)
}

fn connect_to_daemon_with_retry(retries: u32, delay_ms: u64) -> Result<UnixStream, DevboxError> {
    match UnixStream::connect("/tmp/devbox.sock") {
        Ok(stream) => return Ok(stream),
        Err(_) => {
            Command::new(get_daemon_path().map_err(DevboxError::FileOperation)?)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(DevboxError::ProcessLaunch)?;
        }
    }

    for attempt in 0..retries {
        match UnixStream::connect("/tmp/devbox.sock") {
            Ok(stream) => return Ok(stream),
            Err(error) => {
                if attempt + 1 < retries {
                    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                } else {
                    return Err(DevboxError::Ipc(error));
                }
            }
        }
    }

    Err(DevboxError::Ipc(io::Error::new(
        io::ErrorKind::Other,
        "Failed to connect to daemon after multiple attempts",
    )))
}
