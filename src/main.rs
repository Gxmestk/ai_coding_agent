//! # AI Coding Agent - Binary Entry Point
//!
//! This is the main entry point for the ai_coding_agent CLI tool.
//! Currently, it implements a markdown reader as the first feature.
//!
//! ## Usage
//!
//! ```text
//! ai_coding_agent <markdown_file>
//!
//! Options:
//!     -h, --help    Display help information
//! ```
//!
//! ## Examples
//!
//! ```text
//! # Read a markdown file
//! $ ai_coding_agent README.md
//!
//! # Show help
//! $ ai_coding_agent --help
//! ```

// ============================================================================
// MODULE IMPORTS
// ============================================================================

// Import CLI utilities for argument parsing and help display
use ai_coding_agent::cli::{display_help, display_usage_error, parse_arguments};

// Import markdown reader functionality
use ai_coding_agent::markdown::{MarkdownError, read_markdown_file};

// Import standard library for exit codes
use std::process;

/// Exit code for successful execution
const EXIT_SUCCESS: i32 = 0;

/// Exit code for errors (non-zero)
const EXIT_ERROR: i32 = 1;

/// Main function - Entry point of the application
///
/// This function:
/// 1. Parses command-line arguments
/// 2. Displays help if requested
/// 3. Reads the specified markdown file
/// 4. Handles errors appropriately
/// 5. Returns appropriate exit codes
///
/// # Error Handling
///
/// The function handles errors gracefully by:
/// - Displaying user-friendly error messages
/// - Providing help for usage errors
/// - Returning appropriate exit codes for integration with shell scripts
fn main() {
    // Step 1: Parse command-line arguments
    // parse_arguments() returns Result<Option<String>, ParseError>
    // - Ok(Some(filename)): Valid filename provided
    // - Ok(None): Help was requested (don't process a file)
    // - Err(e): Invalid arguments
    let filename = match parse_arguments() {
        // User requested help, display it and exit successfully
        Ok(None) => {
            display_help();
            process::exit(EXIT_SUCCESS);
        }
        // Valid filename provided, continue to read the file
        Ok(Some(file)) => file,
        // Invalid arguments, display error and help, then exit with error code
        Err(e) => {
            display_usage_error(&format!("Invalid arguments: {}", e));
            process::exit(EXIT_ERROR);
        }
    };

    // Step 2: Read the markdown file
    // read_markdown_file() performs validation (file exists, has .md extension)
    // and returns the file content or a detailed error
    match read_markdown_file(&filename) {
        // File read successfully, display the content
        Ok(content) => {
            println!("{}", content);
            // Exit successfully
            process::exit(EXIT_SUCCESS);
        }
        // Error occurred while reading the file
        Err(error) => {
            // Display the error message to stderr
            eprintln!("Error: {}", error);

            // Provide helpful hints based on the error type
            match error {
                MarkdownError::FileNotFound(_) => {
                    eprintln!("Hint: Make sure the file path is correct and the file exists.");
                }
                MarkdownError::InvalidExtension(_, _) => {
                    eprintln!("Hint: Markdown files must have a .md or .markdown extension.");
                }
                MarkdownError::InvalidPath => {
                    eprintln!("Hint: Please provide a valid file path.");
                }
                MarkdownError::IoError(_) => {
                    eprintln!("Hint: Check file permissions and ensure the file is accessible.");
                }
                _ => {
                    eprintln!("Hint: An unexpected error occurred.");
                }
            }

            // Exit with error code
            process::exit(EXIT_ERROR);
        }
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

// These tests verify the CLI behavior as a whole.
// They run with `cargo test --test main` or `cargo test`.
//
// Note: Integration tests in main.rs can't easily test CLI argument parsing
// because we can't easily modify env::args() in tests. For comprehensive
// CLI testing, create a separate integration test file in the tests/ directory.

#[cfg(test)]
mod tests {
    // Import for testing
    use super::*;

    // -------------------------------------------------------------------------
    // TEST: Exit codes are correct
    // -------------------------------------------------------------------------

    #[test]
    fn test_exit_code_constants() {
        // Verify that exit codes are defined correctly
        assert_eq!(EXIT_SUCCESS, 0);
        assert_eq!(EXIT_ERROR, 1);
    }
}
