//! # AI Coding Agent Library
//!
//! This library provides the core functionality for the AI Coding Agent project.
//! It includes modules for reading and processing markdown files, CLI utilities,
//! and will expand to include AI-powered code generation features.
//!
//! ## Modules
//!
//! - [`markdown`]: Markdown file reading and validation
//! - [`cli`]: Command-line interface utilities
//!
//! ## Example Usage
//!
//! ```no_run
//! use ai_coding_agent::markdown::read_markdown_file;
//!
//! match read_markdown_file("README.md") {
//!     Ok(content) => println!("{}", content),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```

// Public module declarations
// These make our modules accessible to users of the library
pub mod cli;
pub mod markdown;
