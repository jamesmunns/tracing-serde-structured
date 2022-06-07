# tracing-serde-structured

An alternative, structured, adapter for serializing [`tracing`] types using [`serde`].

[![Documentation][docs-badge]][docs-url]

[docs-badge]: https://docs.rs/tracing-serde-structured/badge.svg
[docs-url]: https://docs.rs/tracing-serde-structured

## Overview

[`tracing`] is a framework for instrumenting Rust programs to collect
scoped, structured, and async-aware diagnostics.`tracing-serde-structured` enables
serializing `tracing` types using [`serde`].

Traditional logging is based on human-readable text messages.
`tracing` gives us machine-readable structured diagnostic
information. This lets us interact with diagnostic data
programmatically. With `tracing-serde-structured`, you can implement a
`Subscriber` to serialize your `tracing` types and make use of the
existing ecosystem of `serde` serializers to talk with distributed
tracing systems.

Serializing diagnostic information allows us to do more with our logged
values. For instance, when working with logging data in JSON gives us
pretty-print when we're debugging in development and you can emit JSON
and tracing data to monitor your services in production.

The `tracing` crate provides the APIs necessary for instrumenting
libraries and applications to emit trace data.

## Differences with the `tracing-serde` crate

Unlike the upstream [`tracing-serde`] crate, `tracing-serde-structured` does this serialization
in a structured manner, making the data compatible with binary formats such as [`postcard`],
while also allowing deserialization of the data.

`tracing-serde-structured` is still compatible with serialization and deserialization to/from
JSON, though it does change the format of the JSON data, meaning it is not a 100% drop-in replacement.

[`tracing-serde`]: https://docs.rs/tracing-serde
[`postcard`]: https://docs.rs/postcard

The following is an example of the difference between `tracing-serde` and `tracing-serde-structured`
data:

```rust
pub fn main() {
    // 1 - new span
    let span = tracing::span!(Level::TRACE, "outer_span");
    // 2 - enter span
    let _span = span.enter();
    do_thing::doit();
    // 7 - exit span
}

mod do_thing {
    pub fn doit() {
        // 3 - new span
        let span = tracing::span!(Level::TRACE, "my span");
        // 4- enter span
        span.in_scope(|| {
            // 5 - event
            event!(Level::INFO, "something has happened!");
            // 6 - exit span
        });
    }
}
```

```diff
# 1 - new span
- '{"name":"outer_span","target":"tracing_playground","level":"TRACE","module_path":"tracing_playground","file":"src/main.rs","line":34,"fields":[],"is_span":true,"is_event":false}'
+ '{"name":"outer_span","target":"tracing_playground","level":"TRACE","module_path":"tracing_playground","file":"src/main.rs","line":34,"fields":[],"is_span":true,"is_event":false}'
# 2 - enter span
- '{"metadata":{"name":"outer_span","target":"tracing_playground","level":"TRACE","module_path":"tracing_playground","file":"src/main.rs","line":34,"fields":[],"is_span":true,"is_event":false},"parent":null,"is_root":false}'
+ '{"metadata":{"name":"outer_span","target":"tracing_playground","level":"TRACE","module_path":"tracing_playground","file":"src/main.rs","line":34,"fields":[],"is_span":true,"is_event":false},"parent":null,"is_root":false}'
- '[1]'
+ '{"id":1}'
# 3 - new span
- '{"name":"my span","target":"tracing_playground::do_thing","level":"TRACE","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":74,"fields":[],"is_span":true,"is_event":false}'
+ '{"name":"my span","target":"tracing_playground::do_thing","level":"TRACE","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":74,"fields":[],"is_span":true,"is_event":false}'
# 4 - enter span
- '{"metadata":{"name":"my span","target":"tracing_playground::do_thing","level":"TRACE","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":74,"fields":[],"is_span":true,"is_event":false},"parent":null,"is_root":false}'
+ '{"metadata":{"name":"my span","target":"tracing_playground::do_thing","level":"TRACE","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":74,"fields":[],"is_span":true,"is_event":false},"parent":null,"is_root":false}'
- '[2]'
+ '{"id":2}'
# 5 - event
- '{"name":"event src/main.rs:76","target":"tracing_playground::do_thing","level":"INFO","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":76,"fields":["message"],"is_span":false,"is_event":true}'
+ '{"name":"event src/main.rs:76","target":"tracing_playground::do_thing","level":"INFO","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":76,"fields":["message"],"is_span":false,"is_event":true}'
- '{"metadata":{"name":"event src/main.rs:76","target":"tracing_playground::do_thing","level":"INFO","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":76,"fields":["message"],"is_span":false,"is_event":true},"message":"something has happened!"}'
+ '{"fields":{"message":{"Debug":"something has happened!"}},"metadata":{"name":"event src/main.rs:76","target":"tracing_playground::do_thing","level":"INFO","module_path":"tracing_playground::do_thing","file":"src/main.rs","line":76,"fields":["message"],"is_span":false,"is_event":true},"parent":null}'
# 6 - exit span
- '[2]'
+ '{"id":2}'
# 7 - exit span
- '[1]'
+ '{"id":1}'
```

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
tracing = "0.1"
tracing-serde-structured = "0.1"
```

Next, add this to your crate:

```rust
use tracing_serde::AsSerde;
```

Please read the [`tracing` documentation](https://docs.rs/tracing/latest/tracing/index.html)
for more information on how to create trace data.

This crate provides the `as_serde` function, via the `AsSerde` trait,
which enables serializing the `Attributes`, `Event`, `Id`, `Metadata`,
and `Record` `tracing` values.

Implement a `Subscriber` to format the serialization of `tracing`
types how you'd like.

```rust
pub struct JsonSubscriber {
    next_id: AtomicUsize, // you need to assign span IDs, so you need a counter
}

impl Subscriber for JsonSubscriber {

    fn new_span(&self, attrs: &Attributes) -> Id {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = Id::from_u64(id as u64);
        let json = json!({
        "new_span": {
            "attributes": attrs.as_serde(),
            "id": id.as_serde(),
        }});
        println!("{}", json);
        id
    }
    // ...
}
```

After you implement your `Subscriber`, you can use your `tracing`
subscriber (`JsonSubscriber` in the above example) to record serialized
trace data.

##  Crate Feature Flags

The following crate feature flags are available:

* `std`: Depend on the Rust standard library (enabled by default).

  `no_std` users may disable this feature with `default-features = false`:

  ```toml
  [dependencies]
  tracing-serde-structured = { version = "0.1", default-features = false }
  ```

### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x
releases. To enable these features, the `--cfg tracing_unstable` must be passed to
`rustc` when compiling.

The following unstable feature flags are currently available:

* `valuable`: Enables [`Visit::record_value`] implementations, for
  serializing values recorded using the [`valuable`] crate.

#### Enabling Unstable Features

The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
env variable when running `cargo` commands:

```shell
RUSTFLAGS="--cfg tracing_unstable" cargo build
```
Alternatively, the following can be added to the `.cargo/config` file in a
project to automatically enable the cfg flag for that project:

```toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```

[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[`valuable`]: https://crates.io/crates/valuable

## Provenance

This crate is a fork of the [`tracing-serde`] library, as provided by the Tokio project.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, shall be licensed as MIT, without any additional
terms or conditions.

[`tracing`]: https://crates.io/crates/tracing
[`serde`]: https://crates.io/crates/serde
