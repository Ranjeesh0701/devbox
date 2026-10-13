use std::collections::HashMap;
use std::env;
use std::io;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};

use devbox::config::{find_config, load_config};
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
        println!("  logs <session_id>                 - Show logs for a session.");
        println!("  shutdown                          - Shutdown the daemon.");
        println!(
            "  up                                - Start all enabled services from devbox.yaml."
        );
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
                cwd: env::current_dir().map_err(DevboxError::FileOperation)?,
                env: HashMap::new(),
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
        "logs" => {
            let id = args.next().ok_or_else(|| {
                DevboxError::InvalidArguments(
                    "No session ID specified. Usage: logs <session_id>".to_string(),
                )
            })?;

            let follow = match args.next().as_deref() {
                None => false,
                Some("--follow") => true,
                Some(arg) => {
                    return Err(DevboxError::InvalidArguments(format!(
                        "Unexpected argument: {arg}. Usage: logs <session_id> [--follow]"
                    )));
                }
            };

            Request::Logs { id, follow }
        }
        "shutdown" => {
            if args.next().is_some() {
                return Err(DevboxError::InvalidArguments(
                    "No arguments expected for 'shutdown'. Usage: shutdown".to_string(),
                ));
            }

            Request::Shutdown
        }
        "up" => {
            if args.next().is_some() {
                return Err(DevboxError::InvalidArguments(
                    "No arguments expected for 'up'. Usage: up".to_string(),
                ));
            }

            let path = find_config()
                .map_err(DevboxError::ConfigNotFound)?
                .canonicalize()
                .map_err(DevboxError::FileOperation)?;
            let config = load_config(&path).map_err(|_| DevboxError::InvalidResponse)?;
            let project_dir = path
                .parent()
                .ok_or(DevboxError::InvalidResponse)?
                .to_path_buf();

            Request::Up {
                project_dir,
                services: config
                    .enabled_services()
                    .map(|(name, service)| (name.clone(), service.clone()))
                    .collect(),
            }
        }
        "down" => {
            if args.next().is_some() {
                return Err(DevboxError::InvalidArguments(
                    "No arguments expected for 'down'. Usage: down".to_string(),
                ));
            }

            let path = find_config()
                .map_err(DevboxError::ConfigNotFound)?
                .canonicalize()
                .map_err(DevboxError::FileOperation)?;
            let project_dir = path
                .parent()
                .ok_or(DevboxError::InvalidResponse)?
                .to_path_buf();

            Request::Down { project_dir }
        }
        _ => {
            return Err(DevboxError::InvalidArguments(format!(
                "Invalid action: {action}. Supported actions: start, list, stop, logs, shutdown, up."
            )));
        }
    };

    let mut stream = connect_to_daemon_with_retry(50, 100)?;

    write_message(&mut stream, &request).map_err(DevboxError::Ipc)?;

    let mut response = read_message::<_, Response>(&mut stream).map_err(DevboxError::Ipc)?;

    loop {
        match response {
            Response::Start { session } => {
                if !matches!(request, Request::Start { .. }) {
                    return Err(DevboxError::InvalidResponse);
                }
                println!("{session:?}");
                break;
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
                break;
            }
            Response::LogChunk { logs } => {
                if !matches!(request, Request::Logs { .. }) {
                    return Err(DevboxError::InvalidResponse);
                }

                print!("{logs}");
                use std::io::Write;
                std::io::stdout()
                    .flush()
                    .map_err(DevboxError::FileOperation)?;

                response = read_message::<_, Response>(&mut stream).map_err(DevboxError::Ipc)?;
            }
            Response::LogStreamEnd => {
                if !matches!(request, Request::Logs { .. }) {
                    return Err(DevboxError::InvalidResponse);
                }
                break;
            }
            Response::Stop => {
                if !matches!(request, Request::Stop { .. }) {
                    return Err(DevboxError::InvalidResponse);
                }
                println!("Session stopped successfully.");
                break;
            }
            Response::Shutdown => {
                if !matches!(request, Request::Shutdown) {
                    return Err(DevboxError::InvalidResponse);
                }
                println!("Daemon shutdown successfully.");
                break;
            }
            Response::Up { results } => {
                if !matches!(request, Request::Up { .. }) {
                    return Err(DevboxError::InvalidResponse);
                }

                let mut failed = false;

                for result in results {
                    if let Some(session) = result.session {
                        println!(
                            "Service '{}' started successfully with session ID: {}",
                            result.name, session.id
                        );
                    } else if let Some(error) = result.error {
                        failed = true;
                        eprintln!("Failed to start service '{}': {}", result.name, error);
                    }
                }

                if failed {
                    return Ok(1);
                }

                break;
            }
            Response::Down { results } => {
                if !matches!(request, Request::Down { .. }) {
                    return Err(DevboxError::InvalidResponse);
                }

                if results.is_empty() {
                    println!("No running services found for this project.");
                    break;
                }

                let mut failed = false;

                for result in results {
                    if let Some(error) = result.error {
                        failed = true;
                        eprintln!("Failed to stop service '{}': {}", result.name, error);
                    } else {
                        println!("Service '{}' stopped successfully.", result.name);
                    }
                }

                if failed {
                    return Ok(1);
                }

                break;
            }
            Response::Logs { .. } => {
                return Err(DevboxError::InvalidResponse);
            }
            Response::Error { message } => {
                return Err(DevboxError::DaemonError(message));
            }
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
