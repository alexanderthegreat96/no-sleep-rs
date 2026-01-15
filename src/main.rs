mod core;
use crate::core::afk_executor;
use std::io::{self, Write};

// intended to add \r\n after every log entry line on screen
struct RawWriter<W: Write>(W);

impl<W: Write> Write for RawWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // proper line terminations so we don't fuck the terminal aspect
        for &byte in buf {
            if byte == b'\n' {
                self.0.write_all(b"\r")?;
            }
            self.0.write_all(&[byte])?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_target(false)
        .with_thread_names(true)
        .without_time()
        .with_ansi(true)
        .with_writer(|| RawWriter(io::stderr()))
        .init();

    tracing::info!("Starting No-Sleep-RS v0.0.1");
    afk_executor::run();
}
