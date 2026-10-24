// SPDX-License-Identifier: GPL-3.0-or-later

//! FFI layer over `samare-core`.
//!
//! Built as a single shared library so that the native BLE service (via
//! UniFFI) and the Flutter UI (via flutter_rust_bridge) share the same core
//! instance in the same process.

/// Version of the core library, exposed to the bindings.
pub fn core_version() -> String {
    samare_core::version().to_owned()
}
