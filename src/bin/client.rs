use std::os::unix::net::UnixStream;

use devbox::protocol::{Request, Response, read_message, write_message};

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/devbox.sock")?;

    // let request = Request::Start {
    //     name: "test".to_string(),
    //     program: "sh".to_string(),
    //     args: vec!["-c".to_string(), "sleep 60; echo hello".to_string()],
    // };

    // let request = Request::Stop {
    //     id: "5d5c955f-6d08-40c8-9cc2-2ec8aba4b7ab".to_string(),
    // };

    let request = Request::List;

    write_message(&mut stream, &request)?;
    let response: Response = read_message(&mut stream)?;
    println!("{response:?}");

    Ok(())
}
