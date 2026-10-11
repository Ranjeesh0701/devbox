use std::io;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::thread;

use devbox::process::supervisor::ProcessSupervisor;
use devbox::protocol::{Request, Response, read_message, write_message};

fn main() -> io::Result<()> {
    let supervisor = Arc::new(Mutex::new(ProcessSupervisor::new()));

    let path = "/tmp/devbox.sock";

    if std::path::Path::new(path).exists() {
        std::fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                let supervisor = Arc::clone(&supervisor);
                println!("Received a CLI connection");
                thread::spawn(move || handle_client(stream, supervisor));
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
) -> io::Result<()> {
    let request: Request = read_message(&mut stream)?;
    println!("{request:?}");

    let response: Response = {
        let mut supervisor = supervisor
            .lock()
            .map_err(|_| io::Error::other("Supervisor lock poisoned"))?;

        match request {
            Request::Start {
                name,
                program,
                args,
            } => {
                let session = supervisor
                    .start(name, program, args)
                    .map_err(|_| io::Error::other("Failed to run supervisor operation"))?;

                Response::Start {
                    session: session.clone(),
                }
            }
            Request::List => {
                let sessions = supervisor
                    .list()
                    .map_err(|_| io::Error::other("Failed to run supervisor operation"))?;

                Response::List { sessions: sessions }
            }
            Request::Stop { id } => {
                supervisor
                    .stop(&id)
                    .map_err(|_| io::Error::other("Failed to run supervisor operation"))?;

                Response::Stop
            }
        }
    };

    write_message(&mut stream, &response)?;

    Ok(())
}
