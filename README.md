# tracing-serde-structured

An alternative, structured, adapter for serializing [`tracing`] types using [`serde`].

[![Documentation][docs-badge]][docs-url]

[docs-badge]: https://docs.rs/tracing-serde-structured/badge.svg
[docs-url]: https://docs.rs/tracing-serde-structured

## Overview

[`tracing`] is a framework for instrumenting Rust programs to collect
scoped, structured, and async-aware diagnostics.`tracing-serde-structured` enables
serializing `tracing` types using [`serde`].

Unlike the upstream [`tracing-serde`] crate, `tracing-serde-structured` does this
in a structured manner, making the data compatible with binary formats such as [`postcard`],
while also allowing deserialization of the data.

[`tracing-serde`]: https://docs.rs/tracing-serde
[`postcard`]: https://docs.rs/postcard

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
