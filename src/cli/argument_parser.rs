//! # Command-Line Argument Parser
//!
//! This module handles parsing and validation of command-line arguments for the
//! ai_coding_agent markdown reader. It processes user input from the command line
//! and provides structured output for the application.
//!
//! ## Features
//!
//! - Parses markdown file paths from command line
//! - Supports `--help` flag for displaying usage information
//! - Validates that arguments are provided
//! - Provides clear error messages for invalid input
//!
//! ## Example Usage
//!
//! ```no_run
//! use ai_coding_agent::cli::argument_parser::parse_arguments;
//!
//! fn main() {
//!     match parse_arguments() {
//!         Ok(Some(filename)) => println!("Reading: {}", filename),
//!         Ok(None) => println!("No file specified"),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```

use std::env;

/// Result type for argument parsing
///
/// Returns `Ok(Some(filename))` if a valid filename is provided,
/// `Ok(None)` if no filename is needed (e.g., help was displayed),
/// or an error if the arguments are invalid.
pub type ParseResult = Result<Option<String>, ParseError>;

/// Errors that can occur during argument parsing
///
/// This enum provides specific error types for different failure scenarios,
/// making error handling more precise and user-friendly.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// No arguments were provided to the program
    NoArguments,

    /// The provided path is empty or contains only whitespace
    EmptyPath,

    /// The argument provided is not a valid file path
    InvalidPath(String),

    /// Unknown command-line flag was provided
    UnknownFlag(String),
}

/// Parses command-line arguments for the markdown reader
///
/// This function reads arguments from `std::env::args()` and processes them.
/// It handles special flags like `--help` and validates file paths.
///
/// # Returns
///
/// * `Ok(Some(filename))` - A valid markdown file path was provided
/// * `Ok(None)` - Help flag was encountered, no further processing needed
/// * `Err(ParseError)` - Invalid arguments were provided
///
/// # Examples
///
/// ```no_run
/// use ai_coding_agent::cli::argument_parser::parse_arguments;
///
/// // Simulating: cargo run -- README.md
/// let result = parse_arguments();
/// ```
///
/// # Behavior
///
/// 1. Skips the program name (first argument)
/// 2. Checks for `--help` or `-h` flags
/// 3. Validates that a file path is provided
/// 4. Returns the filename or appropriate error
///
/// # Error Cases
///
/// - No arguments: Returns `ParseError::NoArguments`
/// - Empty path: Returns `ParseError::EmptyPath`
/// - Unknown flags: Returns `ParseError::UnknownFlag`
pub fn parse_arguments() -> ParseResult {
    // Get command-line arguments from environment
    // args() returns an iterator of Strings
    let args: Vec<String> = env::args().collect();

    // Check if any arguments were provided beyond the program name
    // args[0] is always the program name/path
    if args.len() < 2 {
        return Err(ParseError::NoArguments);
    }

    // Get the first argument (skip program name at index 0)
    let first_arg = &args[1];

    // Check for help flags
    match first_arg.as_str() {
        "--help" | "-h" => {
            // Help was requested, return None to signal that help should be displayed
            return Ok(None);
        }
        arg if arg.starts_with("--") => {
            // Unknown flag starting with --
            return Err(ParseError::UnknownFlag(arg.to_string()));
        }
        arg if arg.starts_with('-') => {
            // Unknown single dash flag
            return Err(ParseError::UnknownFlag(arg.to_string()));
        }
        _ => {
            // It's a file path, continue processing
        }
    }

    // Check if the argument is empty or only whitespace
    if first_arg.trim().is_empty() {
        return Err(ParseError::EmptyPath);
    }

    // Validate that the argument looks like a file path
    // We don't check if the file exists here - that's done by the markdown reader
    if !is_valid_path(first_arg) {
        return Err(ParseError::InvalidPath(first_arg.clone()));
    }

    // Return the validated filename
    Ok(Some(first_arg.clone()))
}

/// Checks if a string is a valid file path
///
/// This function performs basic validation to ensure the argument
/// looks like a file path. It doesn't check if the file actually exists.
///
/// # Arguments
///
/// * `path` - A string slice that might be a file path
///
/// # Returns
///
/// * `true` if the string appears to be a valid file path
/// * `false` otherwise
///
/// # Examples
///
/// ```
/// use ai_coding_agent::cli::argument_parser::is_valid_path;
///
/// assert!(is_valid_path("README.md"));
/// assert!(is_valid_path("docs/guide.md"));
/// assert!(is_valid_path("./file.md"));
/// assert!(!is_valid_path(""));  // Empty string
/// assert!(!is_valid_path("   "));  // Only whitespace
/// ```
pub fn is_valid_path(path: &str) -> bool {
    // Empty strings are not valid paths
    if path.is_empty() {
        return false;
    }

    // Strings containing only whitespace are not valid paths
    if path.trim().is_empty() {
        return false;
    }

    // Check if it contains invalid characters for paths
    // On Unix-like systems, null character is invalid
    // On Windows, additional characters like <, >, :, ", |, ?, * are invalid
    // We'll do a basic check that works across platforms
    let invalid_chars = ['\0', '<', '>', ':', '"', '|', '?', '*'];
    if path.chars().any(|c| invalid_chars.contains(&c)) {
        return false;
    }

    // If we get here, it looks like a valid path
    true
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

// Implement Display trait for user-friendly error messages
// This allows using to_string() on ParseError
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NoArguments => {
                write!(f, "No arguments provided")
            }
            ParseError::EmptyPath => {
                write!(f, "File path cannot be empty")
            }
            ParseError::InvalidPath(path) => {
                write!(f, "Invalid file path: '{}'", path)
            }
            ParseError::UnknownFlag(flag) => {
                write!(f, "Unknown flag: '{}'", flag)
            }
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // TEST: is_valid_path()
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_valid_path_with_simple_filename() {
        // Test simple filenames
        assert!(is_valid_path("README.md"));
        assert!(is_valid_path("file.md"));
        assert!(is_valid_path("test.MD"));
    }

    #[test]
    fn test_is_valid_path_with_directory_paths() {
        // Test paths with directories
        assert!(is_valid_path("docs/guide.md"));
        assert!(is_valid_path("./README.md"));
        assert!(is_valid_path("../parent/file.md"));
        assert!(is_valid_path("/absolute/path/file.md"));
    }

    #[test]
    fn test_is_valid_path_with_invalid_inputs() {
        // Test empty string
        assert!(!is_valid_path(""));

        // Test whitespace-only strings
        assert!(!is_valid_path("   "));
        assert!(!is_valid_path("\t"));

        // Test paths with invalid characters (platform-specific)
        // These characters are generally invalid in file paths
        assert!(!is_valid_path("file<.md"));
        assert!(!is_valid_path("file>.md"));
        assert!(!is_valid_path("file:.md"));
        assert!(!is_valid_path("file\".md"));
        assert!(!is_valid_path("file|.md"));
        assert!(!is_valid_path("file?.md"));
        assert!(!is_valid_path("file*.md"));
    }

    #[test]
    fn test_is_valid_path_with_special_paths() {
        // Test current and parent directory references
        assert!(is_valid_path("."));
        assert!(is_valid_path(".."));
        assert!(is_valid_path("./"));
        assert!(is_valid_path("../"));
    }

    // -------------------------------------------------------------------------
    // TEST: parse_arguments()
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_arguments_with_valid_filename() {
        // We can't easily test parse_arguments directly since it uses env::args()
        // Instead, we test is_valid_path which is the core validation logic
        // Integration tests will test the full argument parsing
    }

    // -------------------------------------------------------------------------
    // TEST: ParseError
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_error_no_arguments() {
        let error = ParseError::NoArguments;
        assert_eq!(error, ParseError::NoArguments);
    }

    #[test]
    fn test_parse_error_empty_path() {
        let error = ParseError::EmptyPath;
        assert_eq!(error, ParseError::EmptyPath);
    }

    #[test]
    fn test_parse_error_invalid_path() {
        let error = ParseError::InvalidPath("bad<path".to_string());
        assert_eq!(error, ParseError::InvalidPath("bad<path".to_string()));
    }

    #[test]
    fn test_parse_error_unknown_flag() {
        let error = ParseError::UnknownFlag("--unknown".to_string());
        assert_eq!(error, ParseError::UnknownFlag("--unknown".to_string()));
    }

    // -------------------------------------------------------------------------
    // TEST: Error Display (for Debug trait)
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_error_debug() {
        let error = ParseError::NoArguments;
        assert_eq!(format!("{:?}", error), "NoArguments");

        let error = ParseError::InvalidPath("test.md".to_string());
        assert_eq!(format!("{:?}", error), "InvalidPath(\"test.md\")");
    }
}
