use std::fs;
use std::io;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use devbox::process::supervisor::ProcessSupervisor;
use devbox::protocol::{Request, Response, read_message, write_message};
use devbox::storage::ensure_storage_directory;

fn main() -> io::Result<()> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let supervisor = Arc::new(Mutex::new(ProcessSupervisor::new().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize supervisor: {}", e),
        )
    })?));

    let path = "/tmp/devbox.sock";

    let storage_directory = ensure_storage_directory().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create directory: {}", e),
        )
    })?;
    let temp_file = storage_directory.join(devbox::storage::SESSIONS_TMP);

    if temp_file.exists() {
        fs::remove_file(&temp_file)?;
    }

    if Path::new(path).exists() {
        match UnixStream::connect(path) {
            Ok(_) => {
                println!("Deamon already running");
                return Ok(());
            }
            Err(error) => {
                eprintln!("Failed to connect: {error}");
                fs::remove_file(path)?;
            }
        }
    }

    let listener = UnixListener::bind(path)?;
    listener.set_nonblocking(true)?;

    loop {
        if shutdown.load(Ordering::SeqCst) {
            println!("Daemon shutting down");
            break;
        }

        match listener.accept() {
            Ok((stream, _address)) => {
                let supervisor = Arc::clone(&supervisor);
                let shutdown_signal = Arc::clone(&shutdown);

                println!("Received a CLI connection");

                thread::spawn(move || {
                    if let Err(error) = handle_client(stream, supervisor, shutdown_signal) {
                        eprintln!("Failed to handle client: {error}");
                    }
                });
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(50));
            }
            Err(error) => {
                eprintln!("Failed to accept connection: {error}")
            }
        }
    }

    Ok(())
}

fn handle_client(
    mut stream: UnixStream,
    supervisor: Arc<Mutex<ProcessSupervisor>>,
    shutdown: Arc<AtomicBool>,
) -> io::Result<()> {
    let request: Request = read_message(&mut stream)?;
    let is_shutdown = matches!(&request, Request::Shutdown);

    println!("{request:?}");

    let response: Response = match supervisor.lock() {
        Err(_) => Response::Error {
            message: "Supervisor lock poisoned".to_string(),
        },
        Ok(mut supervisor) => match request {
            Request::Start {
                name,
                program,
                args,
            } => match supervisor.start(name, program, args) {
                Ok(session) => Response::Start {
                    session: session.clone(),
                },
                Err(e) => Response::Error {
                    message: format!("Failed to start: {e}"),
                },
            },
            Request::List => match supervisor.list() {
                Ok(sessions) => Response::List { sessions },
                Err(e) => Response::Error {
                    message: format!("Failed to list: {e}"),
                },
            },
            Request::Stop { id } => match supervisor.stop(&id) {
                Ok(()) => Response::Stop,
                Err(e) => Response::Error {
                    message: format!("Failed to stop: {e}"),
                },
            },
            Request::Shutdown => {
                println!("Shutting down daemon");
                Response::Shutdown
            }
        },
    };

    write_message(&mut stream, &response)?;

    if is_shutdown {
        shutdown.store(true, Ordering::SeqCst);
    }

    Ok(())
}
