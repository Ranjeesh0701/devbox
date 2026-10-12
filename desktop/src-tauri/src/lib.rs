use std::os::unix::net::UnixStream;

use devbox_core::process::session::ProcessSession;
use devbox_core::protocol::{Request, Response, read_message, write_message};

#[tauri::command]
fn list_sessions() -> Result<Vec<ProcessSession>, String> {
    let mut stream = UnixStream::connect("/tmp/devbox.sock")
        .map_err(|e| format!("Could not connect to Devbox daemon. Is it running? {e}"))?;

    write_message(&mut stream, &Request::List)
        .map_err(|e| format!("Failed to send request: {e}"))?;

    let response: Response =
        read_message(&mut stream).map_err(|e| format!("Failed to read daemon response: {e}"))?;

    match response {
        Response::List { sessions } => Ok(sessions),
        Response::Error { message } => Err(message),
        other => Err(format!("Unexpected daemon response: {other:?}")),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_sessions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
