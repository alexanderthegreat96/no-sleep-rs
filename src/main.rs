mod core;
use crate::core::afk_executor;
use std::io::{self, Write};

// this is the type of stuff that gives me headaches in rust
// had to actually scratch my brain to do this
// what this RawWriter + implementation does is enable
// me to properly terminate each output to the terminal
// as well as add a prefix to the logging
// this is because enabling_raw_mode can cause
// some serious formatting issues and screen the terminal session
struct RawWriter<W: Write> {
    inner: W,
    new_line: bool,
}

impl<W: Write> RawWriter<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            new_line: true,
        }
    }
}

impl<W: Write> Write for RawWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &byte in buf {
            // write a prefix
            if self.new_line {
                self.inner.write_all(b"[no-sleep] ")?;
                self.new_line = false;
            }

            // write term char
            if byte == b'\n' {
                self.inner.write_all(b"\r")?;
                self.new_line = true;
            }

            self.inner.write_all(&[byte])?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
fn main() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .without_time()
        .with_ansi(true)
        .with_writer(|| RawWriter::new(io::stderr()))
        .init();

    tracing::info!("Starting No-Sleep-RS v0.0.1");
    afk_executor::run();
}
