use std::io::{self, Read, Write};
use std::time::Duration;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::{Listener, connect, connect_with_retry, prepare_listener, remove_socket};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{Listener, connect, connect_with_retry, prepare_listener, remove_socket};

pub(super) trait ReadWrite: Read + Write + Send {
    fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()>;
}

pub struct Connection {
    inner: Box<dyn ReadWrite>,
}

impl Connection {
    pub(crate) fn new<T: ReadWrite + 'static>(stream: T) -> Self {
        Self {
            inner: Box::new(stream),
        }
    }

    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.inner.set_read_timeout(timeout)
    }
}

impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
