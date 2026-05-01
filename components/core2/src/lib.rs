// Copyright (c) core2 contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Forked from core2 v0.4.0 by Brendan Molloy.
// Stripped to io-only. Error trait polyfill removed (stable in core since Rust 1.81).

//! # core2
//!
//! The bare essentials of `std::io` for use in `no_std` environments.
//! Local compatibility shim for [`core2`](https://crates.io/crates/core2).
//!
//! When the `std` feature is enabled (default), this crate re-exports
//! `std::io` directly with zero overhead. When running
//! without `std`, it provides its own implementations of the core I/O
//! traits and types.
//!
//! ## Usage
//!
//! ```rust
//! use core2::io::{Read, Write, Cursor};
//! ```
//!
//! ## Features
//!
//! - `std` (default) - re-exports `std::io`
//! - `alloc` - enables `Vec<u8>` impls and allocating `Read` methods
//!
//! ## Workspace use
//!
//! ```toml
//! core2 = { version = "0.3", default-features = false, features = ["alloc"] }
//! ```
//!
//! Existing `core2::io` imports continue to work.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(dead_code))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub mod io;

#[cfg(feature = "std")]
pub use std::io;
