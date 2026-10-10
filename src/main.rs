use std::env;
use std::process::{Command, ExitCode};
use std::fmt;

enum DevboxError {
    InvalidArguments(String),
    ProcessLaunch(std::io::Error),
    ExitCodeUnavailable,
    ExitCodeOutOfRange(i32),
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

    let action = args.next();
    let process = args.next();
    let args: Vec<String> = args.collect();

    let confirm_action = match action {
        Some(a) => a,
        None => {
            return Err(DevboxError::InvalidArguments("No action specified. Usage: <program> <action> <process> [args...]".to_string()));
        },
    };

    if confirm_action != "run" {
        return Err(DevboxError::InvalidArguments(format!("Invalid action: {}. Only 'run' is supported.", confirm_action)));
    }

    let confirm_process = match process {
        Some(p) => p,
        None => {
            return Err(DevboxError::InvalidArguments("No process specified. Usage: <program> <action> <process> [args...]".to_string()));
        },
    };

    let status = Command::new(confirm_process)
        .args(args)
        .status()
        .map_err(DevboxError::ProcessLaunch)?;


    match status.code() {
        Some(code) => u8::try_from(code)
            .map_err(|_| DevboxError::ExitCodeOutOfRange(code)),
        None => Err(DevboxError::ExitCodeUnavailable),
    }
}
