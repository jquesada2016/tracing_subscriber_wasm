#![deny(missing_docs)]

//! A [`MakeWriter`] implementation to allow directly using
//! [`tracing_subscriber`] in the browser or with NodeJS.
//!
//! The [`MakeConsoleWriter`] allows mapping arbitrary trace events to
//! any other console verbosity level. Check out the
//! [`MakeConsoleWriter::map_trace_level_to`] and similar methods when
//! building the writer.
//!
//! ### Important Note
//! In my testing, if you don't call `.without_time` on the
//! subscriber builder, a runtime exception will be raised.
//!
//! # Example
//! ```rust
//! use tracing_subscriber::fmt;
//! use tracing_subscriber_wasm::MakeConsoleWriter;
//!
//! fmt()
//!   .with_writer(
//!     // To avoide trace events in the browser from showing their
//!     // JS backtrace, which is very annoying, in my opinion
//!     MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
//!   )
//!   // For some reason, if we don't do this in the browser, we get
//!   // a runtime error.
//!   .without_time()
//!   .init();
//! ```

use std::io::{
  self,
  Write,
};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::fmt::MakeWriter;

/// This is the main type that is passed to `with_writer`.
///
/// Note that you can map the tracing levels to different console
/// verbosity levels. This is especially useful for mapping
/// [`tracing::Level::TRACE`] to a different console level,
/// such as `debug`, to avoid all `trace` tracing events
/// showing up in the browser's console with their backtrace
/// expanded.
#[derive(Clone, Copy, Debug, Default)]
pub struct MakeConsoleWriter(MappedLevels);

impl From<MappedLevels> for MakeConsoleWriter {
  fn from(mapped_levels: MappedLevels) -> Self {
    Self::from_mapped_levels(mapped_levels)
  }
}

impl MakeConsoleWriter {
  /// Creates a new [`MakeConsoleWriter`] with the default [`MappedLevels`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a new [`MakeConsoleWriter`] with a custom [`MappedLevels`].
  pub fn from_mapped_levels(mapped_levels: MappedLevels) -> Self {
    Self(mapped_levels)
  }

  /// Maps the [`tracing::Level::TRACE`] to another console level.
  pub fn map_trace_level_to(mut self, level: tracing::Level) -> Self {
    self.0.trace = level;

    self
  }

  /// Maps the [`tracing::Level::DEBUG`] to another console level.
  pub fn map_debug_level_to(mut self, level: tracing::Level) -> Self {
    self.0.debug = level;

    self
  }

  /// Maps the [`tracing::Level::INFO`] to another console level.
  pub fn map_info_level_to(mut self, level: tracing::Level) -> Self {
    self.0.info = level;

    self
  }

  /// Maps the [`tracing::Level::WARN`] to another console level.
  pub fn map_warn_level_to(mut self, level: tracing::Level) -> Self {
    self.0.warn = level;

    self
  }

  /// Maps the [`tracing::Level::ERROR`] to another console level.
  pub fn map_error_level_to(mut self, level: tracing::Level) -> Self {
    self.0.error = level;

    self
  }
}

impl<'a> MakeWriter<'a> for MakeConsoleWriter {
  type Writer = ConsoleWriter;

  fn make_writer(&'a self) -> Self::Writer {
    unimplemented!("use make_writer_for instead");
  }

  fn make_writer_for(&'a self, meta: &tracing::Metadata<'_>) -> Self::Writer {
    ConsoleWriter(*meta.level(), Vec::with_capacity(256))
  }
}

/// Allows mapping [`tracing::Level`] events to a different
/// console level.
#[derive(Clone, Copy, Debug)]
pub struct MappedLevels {
  /// The verbosity level [`tracing::Level::TRACE`] events should be mapped to
  /// in the console.
  pub trace: tracing::Level,
  /// The verbosity level [`tracing::Level::DEBUG`] events should be mapped to
  /// in the console.
  pub debug: tracing::Level,
  /// The verbosity level [`tracing::Level::INFO`] events should be mapped to
  /// in the console.
  pub info: tracing::Level,
  /// The verbosity level [`tracing::Level::WARN`] events should be mapped to
  /// in the console.
  pub warn: tracing::Level,
  /// The verbosity level [`tracing::Level::ERROR`] events should be mapped to
  /// in the console.
  pub error: tracing::Level,
}

impl Default for MappedLevels {
  fn default() -> Self {
    Self {
      trace: tracing::Level::TRACE,
      debug: tracing::Level::DEBUG,
      info: tracing::Level::INFO,
      warn: tracing::Level::WARN,
      error: tracing::Level::ERROR,
    }
  }
}

/// The type which is responsible for actually writing the tracing
/// event out to the console.
pub struct ConsoleWriter(tracing::Level, Vec<u8>);

impl io::Write for ConsoleWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.1.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    use gloo::console;
    use tracing::Level;

    let data = std::str::from_utf8(&self.1).map_err(|_| {
      io::Error::new(io::ErrorKind::InvalidData, "data not UTF-8")
    })?;

    match self.0 {
      Level::TRACE => console::debug!(data),
      Level::DEBUG => console::debug!(data),
      Level::INFO => console::log!(data),
      Level::WARN => console::warn!(data),
      Level::ERROR => console::error!(data),
    }

    Ok(())
  }
}

impl Drop for ConsoleWriter {
  fn drop(&mut self) {
    let _ = self.flush();
  }
}
