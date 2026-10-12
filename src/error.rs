use std::fmt;

#[derive(Debug)]
pub enum DevboxError {
    InvalidArguments(String),
    ProcessLaunch(std::io::Error),
    ProcessNotFound(std::io::Error),
    FileOperation(std::io::Error),
    ProcessCheck(std::io::Error),
    DaemonError(String),
    Ipc(std::io::Error),
    InvalidResponse,
    Json(serde_json::Error),
    StorageDirectoryUnavailable,
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
            DevboxError::Ipc(error) => {
                write!(f, "IPC operation failed: {error}")
            }
            DevboxError::Json(error) => {
                write!(f, "JSON operation failed: {error}")
            }
            DevboxError::InvalidResponse => {
                write!(f, "Invalid response received")
            }
            DevboxError::ProcessNotFound(error) => {
                write!(f, "Process not found: {error}")
            }
            DevboxError::StorageDirectoryUnavailable => {
                write!(f, "Could not determine the application data directory")
            }
            DevboxError::DaemonError(message) => {
                write!(f, "Daemon error: {message}")
            }
        }
    }
}
