//! # CLI Module
//!
//! This module handles command-line interface operations for the ai_coding_agent.
//! It provides utilities for parsing command-line arguments, displaying help text,
//! and managing user interactions.
//!
//! ## Features
//! - Command-line argument parsing using `std::env`
//! - Help text display
//! - Error handling for invalid arguments
//!
//! ## Usage Example
//! ```rust,no_run
//! use ai_coding_agent::cli::parse_arguments;
//!
//! let args = parse_arguments();
//! match args {
//!     Ok(Some(filename)) => println!("Reading: {}", filename),
//!     Ok(None) => println!("Help requested"),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```

pub mod argument_parser;
pub mod help;

/// Re-exports commonly used items for convenience
pub use argument_parser::parse_arguments;
pub use help::{display_help, display_usage_error};
