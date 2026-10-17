use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread::{self, JoinHandle};

pub fn default_shell() -> (&'static str, Vec<String>) {
    #[cfg(target_os = "macos")]
    {
        ("/bin/zsh", vec![])
    }

    #[cfg(target_os = "windows")]
    {
        ("cmd.exe", vec![])
    }
}

pub struct TerminalSession {
    master: Option<Box<dyn MasterPty + Send>>,
    writer: Box<dyn Write + Send>,
    output_rx: Receiver<Vec<u8>>,
    reader_thread: Option<JoinHandle<()>>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
}

impl TerminalSession {
    pub fn spawn(
        program: &str,
        args: &[String],
        cwd: &Path,
        rows: u16,
        cols: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let pty_system = native_pty_system();

        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let portable_pty::PtyPair { master, slave } = pair;

        let mut command = CommandBuilder::new(program);

        for arg in args {
            command.arg(arg);
        }

        command.cwd(cwd);

        let child = slave.spawn_command(command)?;
        drop(slave);

        let writer = master.take_writer()?;
        let mut reader = master.try_clone_reader()?;

        let (sender, output_rx) = mpsc::channel::<Vec<u8>>();

        let reader_thread = thread::spawn(move || {
            let mut buffer = [0_u8; 4096];

            loop {
                let bytes_read = match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                        continue;
                    }
                    Err(_) => break,
                };

                if sender.send(buffer[..bytes_read].to_vec()).is_err() {
                    break;
                }
            }
        });

        Ok(Self {
            master: Some(master),
            writer,
            output_rx,
            reader_thread: Some(reader_thread),
            child,
        })
    }

    pub fn send_input(&mut self, input: &[u8]) -> io::Result<()> {
        self.writer.write_all(input)?;
        self.writer.flush()
    }

    pub fn try_read_output(&self) -> Result<Vec<u8>, TryRecvError> {
        self.output_rx.try_recv()
    }

    pub fn resize(&self, rows: u16, cols: u16) -> Result<(), Box<dyn std::error::Error>> {
        let master = self
            .master
            .as_ref()
            .ok_or("PTY master has already been closed")?;

        master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        Ok(())
    }

    pub fn try_wait(&mut self) -> io::Result<Option<portable_pty::ExitStatus>> {
        self.child
            .try_wait()
            .map_err(|error| io::Error::other(error.to_string()))
    }

    pub fn terminate(&mut self) -> io::Result<()> {
        self.child
            .kill()
            .map_err(|error| io::Error::other(error.to_string()))?;

        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = self.child.kill();

        self.master.take();

        if let Some(reader_thread) = self.reader_thread.take() {
            let _ = reader_thread.join();
        }
    }
}
