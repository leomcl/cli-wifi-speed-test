//! Library module for the swifi speed test tool.
//!
//! This crate provides CLI tools and libraries for testing `WiFi` download and upload speeds.

/// Command-line interface module.
mod cli;
/// Module for server funcs.
mod server;
/// Test module for speed testing funcs.
mod test;

pub use {
        cli::{CliArgs, Config, ConfigBuilder},
        server::{Server, ServerList},
        test::{Test, TestDirection},
};
