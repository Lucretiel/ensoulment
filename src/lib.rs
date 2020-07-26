//! # Ensoulment
//!
//! Ensoulment is a simple rust crate that tells you when a program was
//! compiled. It provides a single method, `ensoulment()`, which returns an
//! `&'static chrono::DateTime` representing this program's build time.
//!
//! ## Reproducible builds
//!
//! If you need deterministic or reproducible builds, set the
//! `ENSOULMENT_TIMESTAMP` environment variable to any valid rfc3339 timestamp.
//! Ensoulment will use that time instead of the actual build time.
//!
//! ## Example
//!
//! For obvious reasons, this crate is difficult to provide meaningful doc
//! examples for. For an example of its use, it instead provides a small
//! program which you can run with `cargo run` to see the library in action.
//!
//! ```text
//! $ cargo clean
//! $ date; cargo run
//! Sun Jul 26 18:30:38 EDT 2020
//!     ...
//! This program was built at 2020-07-26T18:30:41.625771700-04:00
//! $ cargo clean
//! $ env ENSOULMENT_TIMESTAMP="2020-01-01T10:00:00-04:00" cargo run
//!     ...
//! This program was built at 2020-01-01T10:00:00-04:00
//! ```

use chrono;
use lazy_static::lazy_static;

lazy_static! {
    static ref ENSOULMENT: chrono::DateTime<chrono::FixedOffset> =
        include!(concat!(env!("OUT_DIR"), "/moment.rs"));
}

/// Return a fixed [`chrono::DateTime`](https://docs.rs/chrono/0.4/chrono/struct.DateTime.html)
/// representing the moment when this binary was compiled.
pub fn ensoulment() -> &'static chrono::DateTime<chrono::FixedOffset> {
    &*ENSOULMENT
}
