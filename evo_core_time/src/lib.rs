//#=================================================================================================
// CyborgAI
// CC BY-NC-ND 4.0 Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International
// github: https://github.com/cyborg-ai-git
//#=================================================================================================

//--------------------------------------------------------------------------------------------------
//! # Time Management System
//! 
//! This module provides comprehensive time handling capabilities for the Evo Framework,
//! facilitating precise time measurement, synchronization, and conversion across
//! different platforms and environments.
//!
//! ## Core Functionality
//!
//! * **Time Representation**: Unified representation of time points and durations
//! * **Cross-Platform Consistency**: Consistent time handling across all supported platforms
//! * **High Precision**: Nanosecond-level precision for accurate timing operations
//! * **Time Synchronization**: Utilities for synchronizing time across distributed systems
//!
//! ## Primary Components
//!
//! * `UTime`: Core time utility providing time measurement and manipulation functions
//!   - Creation of time points from various sources
//!   - Conversion between different time formats
//!   - Time arithmetic operations
//!   - Formatting and parsing time representations
//!
//! ## System Integration
//!
//! The time module integrates with platform-specific time APIs while providing
//! a consistent interface that abstracts away platform differences, enabling
//! applications to use the same time-related code regardless of deployment environment.
//!
//! ## Implementation Details
//!
//! The module uses conditional compilation to provide optimized implementations
//! for both native Rust and WebAssembly targets while maintaining a consistent API.
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/129898917?v=4")]
#![doc(html_no_source)]
//--------------------------------------------------------------------------------------------------
//RUST
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub use evo_core_time_rust::*;
//--------------------------------------------------------------------------------------------------
//WASM
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub use evo_core_time_rust::*;
//--------------------------------------------------------------------------------------------------