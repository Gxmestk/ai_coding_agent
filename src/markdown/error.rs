//! # Error Types for Markdown Reader
//!
//! This module defines custom error types for the markdown reader functionality.
//! Using custom errors provides better context and more specific error messages
//! compared to using generic IO errors directly.

use std::fmt;
use std::io;

/// Type alias for Results that use MarkdownError
///
/// This makes it easier to write functions that return markdown-related errors
/// Instead of writing `Result<String, MarkdownError>`, you can write `MarkdownResult<String>`
pub type MarkdownResult<T> = Result<T, MarkdownError>;

/// Maximum file size to read (10MB)
/// This prevents reading extremely large files that could cause memory issues
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

/// Custom error type for markdown reader operations
///
/// This enum provides specific error types for different failure scenarios,
/// making error handling more precise and user-friendly.
#[derive(Debug)]
pub enum MarkdownError {
    /// The specified file path does not exist
    /// Contains the file path that was not found
    FileNotFound(String),

    /// The path exists but is not a file (e.g., it's a directory)
    /// Contains the path that is not a file
    NotAFile(String),

    /// The file does not have a valid markdown extension
    /// Contains the file path and the actual extension found
    InvalidExtension(String, String),

    /// An error occurred while reading the file
    /// Contains the file path and the underlying I/O error
    ReadError(String, io::Error),

    /// The file is too large to read safely
    /// Contains the file path and the file size in bytes
    FileTooLarge(String, usize),

    /// The provided file path is empty or invalid
    InvalidPath,

    /// A generic IO error occurred
    /// This wraps standard IO errors that don't fit into other categories
    IoError(io::Error),
}

/// Implement Display trait for human-readable error messages
/// This allows us to use {} to format errors
impl fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkdownError::FileNotFound(path) => {
                write!(f, "File not found: '{}'", path)
            }
            MarkdownError::NotAFile(path) => {
                write!(f, "Path is not a file: '{}'", path)
            }
            MarkdownError::InvalidExtension(path, ext) => {
                write!(
                    f,
                    "File '{}' has invalid extension '{}', expected '.md' or '.markdown'",
                    path, ext
                )
            }
            MarkdownError::ReadError(path, io_err) => {
                write!(f, "Failed to read file '{}': {}", path, io_err)
            }
            MarkdownError::FileTooLarge(path, size) => {
                write!(
                    f,
                    "File '{}' is too large ({} bytes), maximum allowed is {} bytes",
                    path, size, MAX_FILE_SIZE
                )
            }
            MarkdownError::InvalidPath => {
                write!(f, "Invalid file path provided")
            }
            MarkdownError::IoError(err) => {
                write!(f, "I/O error: {}", err)
            }
        }
    }
}

/// Implement std::error::Error trait for our custom error
/// This allows our error to be used anywhere std::error::Error is required
impl std::error::Error for MarkdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Provide the underlying IO error when applicable
        match self {
            MarkdownError::ReadError(_, io_err) => Some(io_err),
            MarkdownError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

/// Implement conversion from io::Error to MarkdownError
/// This allows us to use the ? operator directly with IO operations
impl From<io::Error> for MarkdownError {
    fn from(error: io::Error) -> Self {
        // Check if it's a "Not Found" error and convert to FileNotFound
        if error.kind() == io::ErrorKind::NotFound {
            MarkdownError::FileNotFound("file".to_string())
        } else {
            MarkdownError::IoError(error)
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io;

    // -------------------------------------------------------------------------
    // TEST: MarkdownError Display
    // -------------------------------------------------------------------------

    #[test]
    fn test_error_display_file_not_found() {
        let error = MarkdownError::FileNotFound("test.md".to_string());
        let msg = format!("{}", error);
        assert!(msg.contains("File not found"));
        assert!(msg.contains("test.md"));
    }

    #[test]
    fn test_error_display_not_a_file() {
        let error = MarkdownError::NotAFile("directory".to_string());
        let msg = format!("{}", error);
        assert!(msg.contains("not a file"));
        assert!(msg.contains("directory"));
    }

    #[test]
    fn test_error_display_invalid_extension() {
        let error = MarkdownError::InvalidExtension("test.txt".to_string(), "txt".to_string());
        let msg = format!("{}", error);
        assert!(msg.contains("invalid extension"));
        assert!(msg.contains("txt"));
        assert!(msg.contains("expected '.md'"));
    }

    #[test]
    fn test_error_display_read_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
        let error = MarkdownError::ReadError("file.md".to_string(), io_err);
        let msg = format!("{}", error);
        assert!(msg.contains("Failed to read"));
        assert!(msg.contains("Permission denied"));
    }

    #[test]
    fn test_error_display_file_too_large() {
        let error = MarkdownError::FileTooLarge("big.md".to_string(), 20 * 1024 * 1024);
        let msg = format!("{}", error);
        assert!(msg.contains("too large"));
        assert!(msg.contains("20971520"));
    }

    #[test]
    fn test_error_display_invalid_path() {
        let error = MarkdownError::InvalidPath;
        let msg = format!("{}", error);
        assert!(msg.contains("Invalid file path"));
    }

    // -------------------------------------------------------------------------
    // TEST: From<io::Error> conversion
    // -------------------------------------------------------------------------

    #[test]
    fn test_from_io_error_not_found() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "No such file");
        let md_error: MarkdownError = io_err.into();

        match md_error {
            MarkdownError::FileNotFound(_) => (), // Expected
            _ => panic!("Expected FileNotFound variant"),
        }
    }

    #[test]
    fn test_from_io_error_other() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
        let md_error: MarkdownError = io_err.into();

        match md_error {
            MarkdownError::IoError(_) => (), // Expected
            _ => panic!("Expected IoError variant"),
        }
    }

    // -------------------------------------------------------------------------
    // TEST: Error source
    // -------------------------------------------------------------------------

    #[test]
    fn test_error_source_with_io_error() {
        let io_err = io::Error::new(io::ErrorKind::Other, "Test error");
        let error = MarkdownError::ReadError("test.md".to_string(), io_err);

        assert!(error.source().is_some());
        // Note: We don't check the exact Debug format of io::Error
        // because it can vary between Rust versions
    }

    #[test]
    fn test_error_source_without_io_error() {
        let error = MarkdownError::FileNotFound("test.md".to_string());

        assert!(error.source().is_none());
    }

    // -------------------------------------------------------------------------
    // TEST: MarkdownResult type alias
    // -------------------------------------------------------------------------

    #[test]
    fn test_markdown_result_ok() {
        let result: MarkdownResult<String> = Ok("Success".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
    }

    #[test]
    fn test_markdown_result_err() {
        let error = MarkdownError::InvalidPath;
        let result: MarkdownResult<String> = Err(error);
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "Invalid file path provided"
        );
    }
}
