//! # tracing-serde-structured
//!
//! An alternative, structured, adapter for serializing [`tracing`] types using [`serde`].
//!
//! [![Documentation][docs-badge]][docs-url]
//!
//! [docs-badge]: https://docs.rs/tracing-serde-structured/badge.svg
//! [docs-url]: https://docs.rs/tracing-serde-structured
//!
//! ## Overview
//!
//! [`tracing`] is a framework for instrumenting Rust programs to collect
//! scoped, structured, and async-aware diagnostics.`tracing-serde-structured` enables
//! serializing `tracing` types using [`serde`].
//!
//! Traditional logging is based on human-readable text messages.
//! `tracing` gives us machine-readable structured diagnostic
//! information. This lets us interact with diagnostic data
//! programmatically. With `tracing-serde-structured`, you can implement a
//! `Subscriber` to serialize your `tracing` types and make use of the
//! existing ecosystem of `serde` serializers to talk with distributed
//! tracing systems.
//!
//! Serializing diagnostic information allows us to do more with our logged
//! values. For instance, when working with logging data in JSON gives us
//! pretty-print when we're debugging in development and you can emit JSON
//! and tracing data to monitor your services in production.
//!
//! The `tracing` crate provides the APIs necessary for instrumenting
//! libraries and applications to emit trace data.
//!
//! ## Differences with the `tracing-serde` crate
//!
//! Unlike the upstream [`tracing-serde`] crate, `tracing-serde-structured` does this serialization
//! in a structured manner, making the data compatible with binary formats such as [`postcard`],
//! while also allowing deserialization of the data.
//!
//! `tracing-serde-structured` is still compatible with serialization and deserialization to/from
//! JSON, though it does change the format of the JSON data, meaning it is not a 100% drop-in replacement.
//!
//! [`tracing-serde`]: https://docs.rs/tracing-serde
//! [`postcard`]: https://docs.rs/postcard
//!
//! ## Usage
//!
//! First, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tracing = "0.1"
//! tracing-serde-structured = "0.1"
//! ```
//!
//! Next, add this to your crate:
//!
//! ```rust
//! use tracing_serde_structured::AsSerde;
//! ```
//!
//! Please read the [`tracing` documentation](https://docs.rs/tracing/latest/tracing/index.html)
//! for more information on how to create trace data.
//!
//! This crate provides the `as_serde` function, via the `AsSerde` trait,
//! which enables serializing the `Attributes`, `Event`, `Id`, `Metadata`,
//! and `Record` `tracing` values.
//!
//! Implement a `Subscriber` to format the serialization of `tracing`
//! types how you'd like.
//!
//! ```rust
//! # use tracing_core::{Subscriber, Metadata, Event};
//! # use tracing_core::span::{Attributes, Id, Record};
//! # use std::sync::atomic::{AtomicUsize, Ordering};
//! use tracing_serde_structured::AsSerde;
//! use serde_json::json;
//!
//! pub struct JsonSubscriber {
//!     next_id: AtomicUsize, // you need to assign span IDs, so you need a counter
//! }
//!
//! impl Subscriber for JsonSubscriber {
//!
//!     fn new_span(&self, attrs: &Attributes<'_>) -> Id {
//!         let id = self.next_id.fetch_add(1, Ordering::Relaxed);
//!         let id = Id::from_u64(id as u64);
//!         let json = json!({
//!         "new_span": {
//!             "attributes": attrs.as_serde(),
//!             "id": id.as_serde(),
//!         }});
//!         println!("{}", json);
//!         id
//!     }
//!
//!     fn event(&self, event: &Event<'_>) {
//!         let json = json!({
//!            "event": event.as_serde(),
//!         });
//!         println!("{}", json);
//!     }
//!
//!     // ...
//!     # fn enabled(&self, _: &Metadata<'_>) -> bool { false }
//!     # fn enter(&self, _: &Id) {}
//!     # fn exit(&self, _: &Id) {}
//!     # fn record(&self, _: &Id, _: &Record<'_>) {}
//!     # fn record_follows_from(&self, _: &Id, _: &Id) {}
//! }
//! ```
//!
//! After you implement your `Subscriber`, you can use your `tracing`
//! subscriber (`JsonSubscriber` in the above example) to record serialized
//! trace data.
//!
//! ##  Crate Feature Flags
//!
//! The following crate feature flags are available:
//!
//! * `std`: Depend on the Rust standard library (enabled by default).
//!
//!   `no_std` users may disable this feature with `default-features = false`:
//!
//!   ```toml
//!   [dependencies]
//!   tracing-serde = { version = "0.2", default-features = false }
//!   ```
//!
//! ### Unstable Features
//!
//! These feature flags enable **unstable** features. The public API may break in 0.1.x
//! releases. To enable these features, the `--cfg tracing_unstable` must be passed to
//! `rustc` when compiling.
//!
//! The following unstable feature flags are currently available:
//!
//! * `valuable`: Enables [`Visit::record_value`] implementations, for
//!   serializing values recorded using the [`valuable`] crate.
//!
//! #### Enabling Unstable Features
//!
//! The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
//! env variable when running `cargo` commands:
//!
//! ```shell
//! RUSTFLAGS="--cfg tracing_unstable" cargo build
//! ```
//! Alternatively, the following can be added to the `.cargo/config` file in a
//! project to automatically enable the cfg flag for that project:
//!
//! ```toml
//! [build]
//! rustflags = ["--cfg", "tracing_unstable"]
//! ```
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//! [`valuable`]: https://crates.io/crates/valuable
//!
//! ## Provenance
//!
//! This crate is a fork of the [`tracing-serde`] library, as provided by the Tokio project.
//!
//! ## License
//!
//! This project is licensed under the [MIT license](LICENSE).
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this project by you, shall be licensed as MIT, without any additional
//! terms or conditions.
//!
//! [`tracing`]: https://crates.io/crates/tracing
//! [`serde`]: https://crates.io/crates/serde
#![doc(html_root_url = "https://docs.rs/tracing-serde-structured/0.1.3")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/logo-type.png",
    html_favicon_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/favicon.ico",
    issue_tracker_base_url = "https://github.com/tokio-rs/tracing/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
#![warn(
    missing_debug_implementations,
    // missing_docs, // TODO: add documentation
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
// Support using tracing-serde without the standard library!
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;
use core::fmt::Arguments;
use core::hash::Hash;
use core::num::NonZeroU64;
use core::ops::Deref;

use serde::{
    ser::{SerializeMap, SerializeSeq, Serializer},
    Deserialize, Serialize,
};

use tracing_core::{
    event::Event,
    field::{Field, FieldSet, Visit},
    metadata::{Level, Metadata},
    span::{Attributes, Id, Record},
};

#[derive(Debug, Deserialize, Eq, PartialOrd, Ord)]
#[serde(from = "&'a str")]
pub enum CowString<'a> {
    Borrowed(&'a str),
    #[cfg(feature = "std")]
    Owned(String),
}

impl<'a> Deref for CowString<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> CowString<'a> {
    pub fn as_str(&'a self) -> &'a str {
        match self {
            CowString::Borrowed(b) => b,
            #[cfg(feature = "std")]
            CowString::Owned(o) => o.as_str(),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> CowString<'a> {
    pub fn to_owned(&'a self) -> CowString<'static> {
        CowString::Owned(self.as_str().to_string())
    }
}

impl<'a> Hash for CowString<'a> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<'a> hash32::Hash for CowString<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash32::Hasher
    {
        <str as hash32::Hash>::hash(self.as_str(), state)
    }
}

impl<'a> PartialEq for CowString<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl<'a> From<&'a str> for CowString<'a> {
    fn from(other: &'a str) -> Self {
        Self::Borrowed(other)
    }
}

impl<'a> Serialize for CowString<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

#[cfg(not(feature = "std"))]
type TracingVec<T> = heapless::Vec<T, 32>;

#[cfg(not(feature = "std"))]
type TracingMap<K, V> = heapless::FnvIndexMap<K, V, 32>;

#[cfg(feature = "std")]
type TracingVec<T> = std::vec::Vec<T>;

#[cfg(feature = "std")]
type TracingMap<K, V> = std::collections::BTreeMap<K, V>;

#[derive(Debug, Deserialize)]
#[serde(from = "TracingVec<CowString<'a>>")]
pub enum SerializeFieldSet<'a> {
    Ser(&'a FieldSet),
    #[serde(borrow)]
    De(TracingVec<CowString<'a>>),
}

impl<'a> Serialize for SerializeFieldSet<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerializeFieldSet::Ser(sfs) => {
                let mut seq = serializer.serialize_seq(Some(sfs.len()))?;
                for element in sfs.iter() {
                    seq.serialize_element(element.name())?;
                }
                seq.end()
            }
            SerializeFieldSet::De(dfs) => dfs.serialize(serializer),
        }
    }
}

impl<'a> From<TracingVec<CowString<'a>>> for SerializeFieldSet<'a> {
    fn from(other: TracingVec<CowString<'a>>) -> Self {
        SerializeFieldSet::De(other)
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SerializeLevel {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = 0,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = 1,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = 2,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn = 3,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 4,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerializeId {
    pub id: NonZeroU64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializeMetadata<'a> {
    #[serde(borrow)]
    pub name: CowString<'a>,
    pub target: CowString<'a>,
    pub level: SerializeLevel,
    pub module_path: Option<CowString<'a>>,
    pub file: Option<CowString<'a>>,
    pub line: Option<u32>,
    pub fields: SerializeFieldSet<'a>,
    pub is_span: bool,
    pub is_event: bool,
}

/// Implements `serde::Serialize` to write `Event` data to a serializer.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializeEvent<'a> {
    #[serde(borrow)]
    pub fields: SerializeRecordFields<'a>,
    pub metadata: SerializeMetadata<'a>,
    pub parent: Option<SerializeId>,
}

/// Implements `serde::Serialize` to write `Attributes` data to a serializer.
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializeAttributes<'a> {
    #[serde(borrow)]
    pub metadata: SerializeMetadata<'a>,
    pub parent: Option<SerializeId>,
    pub is_root: bool,
    #[serde(borrow)]
    pub fields: SerializeSpanFields<'a>,
}

type RecordMap<'a> = TracingMap<CowString<'a>, SerializeValue<'a>>;

/// Implements `serde::Serialize` to write `Record` data to a serializer.
#[derive(Debug, Deserialize)]
#[serde(from = "RecordMap<'a>")]
pub enum SerializeRecord<'a> {
    #[serde(borrow)]
    Ser(&'a Record<'a>),
    De(RecordMap<'a>),
}

impl<'a> Serialize for SerializeRecord<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerializeRecord::Ser(serf) => {
                let items = serf.len();

                let serializer = serializer.serialize_map(Some(items))?;
                let mut ssv = SerdeMapVisitor::new(serializer);
                serf.record(&mut ssv);
                ssv.finish()
            }
            SerializeRecord::De(derf) => derf.serialize(serializer),
        }
    }
}

impl<'a> From<RecordMap<'a>> for SerializeRecord<'a> {
    fn from(other: RecordMap<'a>) -> Self {
        Self::De(other)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SerializeValue<'a> {
    #[serde(borrow)]
    Debug(DebugRecord<'a>),
    Str(CowString<'a>),
    F64(f64),
    I64(i64),
    U64(u64),
    Bool(bool),
}

#[derive(Debug, Deserialize)]
#[serde(from = "CowString<'a>")]
pub enum DebugRecord<'a> {
    #[serde(borrow)]
    Ser(&'a Arguments<'a>),
    De(CowString<'a>),
}

impl<'a> From<CowString<'a>> for DebugRecord<'a> {
    fn from(other: CowString<'a>) -> Self {
        Self::De(other)
    }
}

impl<'a> Serialize for DebugRecord<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DebugRecord::Ser(args) => args.serialize(serializer),
            DebugRecord::De(msg) => msg.serialize(serializer),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "RecordMap<'a>")]
pub enum SerializeRecordFields<'a> {
    #[serde(borrow)]
    Ser(&'a Event<'a>),
    De(RecordMap<'a>),
}


impl<'a> From<RecordMap<'a>> for SerializeRecordFields<'a> {
    fn from(other: RecordMap<'a>) -> Self {
        Self::De(other)
    }
}

impl<'a> Serialize for SerializeRecordFields<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerializeRecordFields::Ser(serf) => {
                let items = serf.fields().count();

                let serializer = serializer.serialize_map(Some(items))?;
                let mut ssv = SerdeMapVisitor::new(serializer);
                serf.record(&mut ssv);
                ssv.finish()
            }
            SerializeRecordFields::De(derf) => derf.serialize(serializer),
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(from = "RecordMap<'a>")]
pub enum SerializeSpanFields<'a> {
    #[serde(borrow)]
    Ser(&'a tracing_core::field::ValueSet<'a>),
    De(RecordMap<'a>),
}

impl<'a> From<RecordMap<'a>> for SerializeSpanFields<'a> {
    fn from(other: RecordMap<'a>) -> Self {
        Self::De(other)
    }
}

impl<'a> Serialize for SerializeSpanFields<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerializeSpanFields::Ser(serf) => {
                let items = serf.len();

                let serializer = serializer.serialize_map(Some(items))?;
                let mut ssv = SerdeMapVisitor::new(serializer);
                serf.record(&mut ssv);
                ssv.finish()
            }
            SerializeSpanFields::De(derf) => derf.serialize(serializer),
        }
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeSpanFields<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeSpanFields<'a> {
    pub fn to_owned(&self) -> SerializeSpanFields<'static> {
        match self {
            SerializeSpanFields::Ser(e) => {
                let mut hv = HashVisit(std::collections::BTreeMap::new());
                e.record(&mut hv);
                SerializeSpanFields::De(hv.0)
            }
            SerializeSpanFields::De(dsrf) => SerializeSpanFields::De(
                dsrf.iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect(),
            ),
        }
    }
}

/// Implements `tracing_core::field::Visit` for some `serde::ser::SerializeMap`.
#[derive(Debug)]
pub struct SerdeMapVisitor<S: SerializeMap> {
    serializer: S,
    state: Result<(), S::Error>,
}

impl<S> SerdeMapVisitor<S>
where
    S: SerializeMap,
{
    /// Create a new map visitor.
    pub fn new(serializer: S) -> Self {
        Self {
            serializer,
            state: Ok(()),
        }
    }

    /// Completes serializing the visited object, returning `Ok(())` if all
    /// fields were serialized correctly, or `Error(S::Error)` if a field could
    /// not be serialized.
    pub fn finish(self) -> Result<S::Ok, S::Error> {
        self.state?;
        self.serializer.end()
    }

    /// Completes serializing the visited object, returning ownership of the underlying serializer
    /// if all fields were serialized correctly, or `Err(S::Error)` if a field could not be
    /// serialized.
    pub fn take_serializer(self) -> Result<S, S::Error> {
        self.state?;
        Ok(self.serializer)
    }
}

impl<S> Visit for SerdeMapVisitor<S>
where
    S: SerializeMap,
{
    #[cfg(all(tracing_unstable, feature = "valuable"))]
    #[cfg_attr(docsrs, doc(cfg(all(tracing_unstable, feature = "valuable"))))]
    fn record_value(&mut self, field: &Field, value: valuable_crate::Value<'_>) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &valuable_serde::Serializable::new(value));
        }
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        // If previous fields serialized successfully, continue serializing,
        // otherwise, short-circuit and do nothing.
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &SerializeValue::Bool(value))
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if self.state.is_ok() {
            self.state = self.serializer.serialize_entry(
                field.name(),
                &SerializeValue::Debug(DebugRecord::Ser(&format_args!("{:?}", value))),
            )
        }
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &SerializeValue::U64(value))
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &SerializeValue::I64(value))
        }
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &SerializeValue::F64(value))
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if self.state.is_ok() {
            self.state = self
                .serializer
                .serialize_entry(field.name(), &SerializeValue::Str(value.into()))
        }
    }
}

pub trait AsSerde<'a>: self::sealed::Sealed {
    type Serializable: serde::Serialize + 'a;

    /// `as_serde` borrows a `tracing` value and returns the serialized value.
    fn as_serde(&'a self) -> Self::Serializable;
}

impl<'a> AsSerde<'a> for tracing_core::Metadata<'a> {
    type Serializable = SerializeMetadata<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeMetadata {
            name: self.name().into(),
            target: self.target().into(),
            level: self.level().as_serde(),
            module_path: self.module_path().map(Into::into),
            file: self.file().map(Into::into),
            line: self.line(),
            fields: SerializeFieldSet::Ser(self.fields()),
            is_span: self.is_span(),
            is_event: self.is_event(),
        }
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeFieldSet<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeFieldSet<'a> {
    pub fn to_owned(&self) -> SerializeFieldSet<'static> {
        match self {
            SerializeFieldSet::Ser(sfs) => SerializeFieldSet::De(
                sfs.iter()
                    .map(|i| CowString::from(i.name()).to_owned())
                    .collect(),
            ),
            SerializeFieldSet::De(dfs) => {
                SerializeFieldSet::De(dfs.iter().map(CowString::to_owned).collect())
            }
        }
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeMetadata<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeMetadata<'a> {
    pub fn to_owned(&self) -> SerializeMetadata<'static> {
        SerializeMetadata {
            name: self.name.to_owned(),
            target: self.target.to_owned(),
            level: self.level,
            module_path: self.module_path.as_ref().map(CowString::to_owned),
            file: self.file.as_ref().map(CowString::to_owned),
            line: self.line,
            fields: self.fields.to_owned(),
            is_span: self.is_span,
            is_event: self.is_event,
        }
    }
}

impl<'a> AsSerde<'a> for tracing_core::Event<'a> {
    type Serializable = SerializeEvent<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeEvent {
            fields: SerializeRecordFields::Ser(self),
            metadata: self.metadata().as_serde(),
            parent: self.parent().map(|p| p.as_serde()),
        }
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for DebugRecord<'static> {}

#[cfg(feature = "std")]
impl<'a> DebugRecord<'a> {
    pub fn to_owned(&self) -> DebugRecord<'static> {
        match self {
            DebugRecord::Ser(args) => DebugRecord::De(CowString::Owned(args.to_string())),
            DebugRecord::De(d) => DebugRecord::De(d.to_owned()),
        }
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeValue<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeValue<'a> {
    pub fn to_owned(&self) -> SerializeValue<'static> {
        match self {
            SerializeValue::Debug(dr) => SerializeValue::Debug(dr.to_owned()),
            SerializeValue::Str(s) => SerializeValue::Str(s.to_owned()),
            SerializeValue::F64(x) => SerializeValue::F64(*x),
            SerializeValue::I64(x) => SerializeValue::I64(*x),
            SerializeValue::U64(x) => SerializeValue::U64(*x),
            SerializeValue::Bool(x) => SerializeValue::Bool(*x),
        }
    }
}

#[cfg(feature = "std")]
struct HashVisit(std::collections::BTreeMap<CowString<'static>, SerializeValue<'static>>);

#[cfg(feature = "std")]
impl Visit for HashVisit {
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.insert(
            CowString::Owned(field.name().to_string()),
            SerializeValue::Bool(value),
        );
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.0.insert(
            CowString::Owned(field.name().to_string()),
            SerializeValue::Debug(DebugRecord::De(CowString::Owned(format!("{:?}", value)))),
        );
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(
            CowString::Owned(field.name().to_string()),
            SerializeValue::U64(value),
        );
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.insert(
            CowString::Owned(field.name().to_string()),
            SerializeValue::I64(value),
        );
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0.insert(
            CowString::Owned(field.name().to_string()),
            SerializeValue::F64(value),
        );
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(
            CowString::Owned(field.name().to_string()),
            SerializeValue::Str(CowString::Owned(value.to_string())),
        );
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeRecordFields<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeRecordFields<'a> {
    pub fn to_owned(&self) -> SerializeRecordFields<'static> {
        match self {
            SerializeRecordFields::Ser(e) => {
                let mut hv = HashVisit(std::collections::BTreeMap::new());
                e.record(&mut hv);
                SerializeRecordFields::De(hv.0)
            }
            SerializeRecordFields::De(dsrf) => SerializeRecordFields::De(
                dsrf.iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect(),
            ),
        }
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeEvent<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeEvent<'a> {
    pub fn to_owned(&self) -> SerializeEvent<'static> {
        SerializeEvent {
            fields: self.fields.to_owned(),
            metadata: self.metadata.to_owned(),
            parent: self.parent.clone(),
        }
    }
}

impl<'a> AsSerde<'a> for tracing_core::span::Attributes<'a> {
    type Serializable = SerializeAttributes<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeAttributes {
            metadata: self.metadata().as_serde(),
            parent: self.parent().map(|p| p.as_serde()),
            is_root: self.is_root(),
            fields: SerializeSpanFields::Ser(self.values())
        }
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeAttributes<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeAttributes<'a> {
    pub fn to_owned(&self) -> SerializeAttributes<'static> {
        SerializeAttributes {
            metadata: self.metadata.to_owned(),
            parent: self.parent.clone(),
            is_root: self.is_root,
            fields: self.fields.to_owned()

        }
    }
}

impl<'a> AsSerde<'a> for tracing_core::span::Id {
    type Serializable = SerializeId;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeId {
            id: self.into_non_zero_u64(),
        }
    }
}

#[cfg(feature = "std")]
impl SerializeId {
    pub fn to_owned(&self) -> Self {
        self.clone()
    }
}

impl<'a> AsSerde<'a> for tracing_core::span::Record<'a> {
    type Serializable = SerializeRecord<'a>;

    fn as_serde(&'a self) -> Self::Serializable {
        SerializeRecord::Ser(self)
    }
}

/// SAFETY: If all data is 'static and/or owned, it is safe
/// to send between threads.
unsafe impl Send for SerializeRecord<'static> {}

#[cfg(feature = "std")]
impl<'a> SerializeRecord<'a> {
    pub fn to_owned(&self) -> SerializeRecord<'static> {
        match self {
            SerializeRecord::Ser(s) => {
                let mut hv = HashVisit(std::collections::BTreeMap::new());
                s.record(&mut hv);
                SerializeRecord::De(hv.0)
            },
            SerializeRecord::De(d) => {
                SerializeRecord::De(
                    d.iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()
                )
            },
        }
    }
}

impl<'a> AsSerde<'a> for Level {
    type Serializable = SerializeLevel;

    fn as_serde(&'a self) -> Self::Serializable {
        match self {
            &Level::ERROR => SerializeLevel::Error,
            &Level::WARN => SerializeLevel::Warn,
            &Level::INFO => SerializeLevel::Info,
            &Level::DEBUG => SerializeLevel::Debug,
            &Level::TRACE => SerializeLevel::Trace,
        }
    }
}

#[cfg(feature = "std")]
impl SerializeLevel {
    pub fn to_owned(&self) -> Self {
        self.clone()
    }
}

impl<'a> self::sealed::Sealed for Event<'a> {}

impl<'a> self::sealed::Sealed for Attributes<'a> {}

impl self::sealed::Sealed for Id {}

impl self::sealed::Sealed for Level {}

impl<'a> self::sealed::Sealed for Record<'a> {}

impl<'a> self::sealed::Sealed for Metadata<'a> {}

mod sealed {
    pub trait Sealed {}
}
