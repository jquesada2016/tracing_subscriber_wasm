# tracing-subscriber-wasm

A [`MakeWriter`] implementation to allow directly using
[`tracing_subscriber`] in the browser or with NodeJS.

The [`MakeConsoleWriter`] allows mapping arbitrary trace events to
any other console verbosity level. Check out the
[`MakeConsoleWriter::map_trace_level_to`] and similar methods when
building the writer.

#### Important Note

In my testing, if you don't call `.without_time` on the
subscriber builder, a runtime exception will be raised.

## Example

```rust
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

fmt()
  .with_writer(
    // To avoide trace events in the browser from showing their
    // JS backtrace, which is very annoying, in my opinion
    MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
  )
  // For some reason, if we don't do this in the browser, we get
  // a runtime error.
  .without_time()
  .init();
```

License: MIT
