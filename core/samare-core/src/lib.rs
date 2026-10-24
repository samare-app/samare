// SPDX-License-Identifier: GPL-3.0-or-later

//! Samare core: identity, crypto, packets, routing and storage.
//!
//! This crate holds all testable logic and has no FFI code. Bindings for
//! Kotlin, Swift and Dart live in `samare-ffi`.

/// Version of the core library.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }
}
