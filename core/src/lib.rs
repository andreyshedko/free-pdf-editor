//! Core application logic.
//!
//! # Thread ownership
//! The `CoreLoop` owns `AppState` and runs on its own dedicated thread.
//! Worker threads are spawned for rendering and send results back via the
//! event sender.
//!
//! # Safety
//! No unsafe code in this crate.

pub mod cache;
pub mod command_loop;
pub mod crash;
pub mod state;
