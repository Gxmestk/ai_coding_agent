//! # Markdown Reader Module
//!
//! This module provides functionality for reading and validating markdown files.
//! It's the first component of the `ai_coding_agent` project.
//!
//! ## Structure
//!
//! This module is organized into two sub-modules:
//!
//! - **`reader`**: Contains the core logic for reading markdown files from disk
//! - **`error`**: Defines custom error types for better error handling
//!
//! ## Example Usage
//!
//! ```no_run
//! use ai_coding_agent::markdown;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let content = markdown::read_markdown_file("README.md")?;
//!     println!("{}", content);
//!     Ok(())
//! }
//! ```

// Re-export public items from sub-modules for cleaner imports
// This allows users to use `markdown::read_markdown_file()` instead of
// `markdown::reader::read_markdown_file()`

pub mod error;
pub mod reader;

// Re-export the main function and error type at the module level
pub use error::{MarkdownError, MarkdownResult};
pub use reader::read_markdown_file;

// Re-export commonly used types from std for convenience
pub use std::path::Path;
