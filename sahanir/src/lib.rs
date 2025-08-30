//! SahaniR: A next-generation backend framework for Rust.
//!
//! SahaniR is designed for building secure, scalable, and performant applications,
//! inspired by cosmological principles.

// Top-level modules for organizing the framework
pub mod core;
pub mod security;
pub mod web;

// A prelude module for easy importing of common types
pub mod prelude {
    pub use crate::core::{Orchestrator, PocketUniverse, UniverseConfiguration};
    pub use sahanir_macros::*;
}

// The prelude is the intended public API for easy use.
