//! # Markdown Reader Module
//!
//! This module provides functionality to read and validate markdown files.
//! It handles:
//! - Reading markdown files from the filesystem
//! - Validating that files have the correct `.md` extension
//! - Proper error handling for various failure scenarios
//!
//! ## Example
//!
//! ```no_run
//! use ai_coding_agent::markdown::reader::read_markdown_file;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let content = read_markdown_file("README.md")?;
//!     println!("{}", content);
//!     Ok(())
//! }
//! ```

use std::fs;
use std::path::Path;

// Import the error types from the error module
// This allows us to use MarkdownError instead of defining our own error type
use crate::markdown::error::{MarkdownError, MarkdownResult};

/// Reads the contents of a markdown file
///
/// This function performs the following checks:
/// 1. Validates the file path exists
/// 2. Validates the file has a `.md` extension (case-insensitive)
/// 3. Reads the file contents into a String
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the markdown file
///
/// # Returns
///
/// * `Ok(String)` - The contents of the file as a String
/// * `Err(MarkdownError)` - An error if the file cannot be read or is invalid
///
/// # Examples
///
/// ```no_run
/// use ai_coding_agent::markdown::reader::read_markdown_file;
///
/// fn main() {
///     match read_markdown_file("README.md") {
///         Ok(content) => println!("File content:\n{}", content),
///         Err(e) => eprintln!("Error: {}", e),
///     }
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The file path does not exist
/// - The file does not have a `.md` extension
/// - The file cannot be read (permission denied, I/O error, etc.)
/// - The file content is not valid UTF-8
/// - The file is larger than the maximum allowed size (10MB)
pub fn read_markdown_file(path: &str) -> MarkdownResult<String> {
    // Convert the path string to a Path object for easier manipulation
    let file_path = Path::new(path);

    // Step 1: Validate that the file exists
    // We check this first to provide a clear error message
    if !file_path.exists() {
        return Err(MarkdownError::FileNotFound(path.to_string()));
    }

    // Step 2: Validate that it's actually a file (not a directory)
    if !file_path.is_file() {
        return Err(MarkdownError::NotAFile(path.to_string()));
    }

    // Step 3: Validate the file extension
    // Markdown files should have .md or .markdown extension
    if !is_markdown_file(file_path) {
        return Err(MarkdownError::InvalidExtension(
            path.to_string(),
            file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("none")
                .to_string(),
        ));
    }

    // Step 4: Read the file contents
    // fs::read_to_string handles opening the file, reading it, and converting to UTF-8
    let content =
        fs::read_to_string(file_path).map_err(|e| MarkdownError::ReadError(path.to_string(), e))?;

    // Step 5: Optional: Validate file size (prevent reading huge files)
    // This is already done implicitly by reading the content, but we check here too
    if content.len() > 10 * 1024 * 1024 {
        return Err(MarkdownError::FileTooLarge(path.to_string(), content.len()));
    }

    Ok(content)
}

/// Checks if a file path points to a markdown file
///
/// This function checks the file extension to determine if it's a markdown file.
/// It supports both `.md` and `.markdown` extensions (case-insensitive).
///
/// # Arguments
///
/// * `path` - A reference to a Path object
///
/// # Returns
///
/// * `true` if the file has a markdown extension
/// * `false` otherwise
///
/// # Examples
///
/// ```
/// use ai_coding_agent::markdown::reader::is_markdown_file;
/// use std::path::Path;
///
/// assert!(is_markdown_file(Path::new("README.md")));
/// assert!(is_markdown_file(Path::new("docs/index.MD")));
/// assert!(!is_markdown_file(Path::new("data.txt")));
/// ```
pub fn is_markdown_file(path: &Path) -> bool {
    // Get the file extension from the path
    // extension() returns Option<&OsStr>, which we convert to a lowercase string
    match path.extension() {
        Some(ext) => {
            // Convert the extension to a lowercase string for comparison
            let ext_lower = ext.to_string_lossy().to_lowercase();
            // Accept both .md and .markdown extensions
            ext_lower == "md" || ext_lower == "markdown"
        }
        None => false,
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================
// These tests verify the functionality of this module.
// They run with `cargo test` and are isolated from other tests.

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    /// Helper function to create a temporary markdown file for testing
    /// Returns the path to the created file
    fn create_temp_markdown_file(content: &str) -> String {
        // Create a unique temporary file path
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!(
            "test_{}.md",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        // Write content to the file
        let mut file = fs::File::create(&file_path).expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");

        // Convert to string and return
        file_path.to_string_lossy().to_string()
    }

    /// Helper function to create a temporary non-markdown file for testing
    fn create_temp_text_file(content: &str) -> String {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!(
            "test_{}.txt",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        let mut file = fs::File::create(&file_path).expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");

        file_path.to_string_lossy().to_string()
    }

    // -------------------------------------------------------------------------
    // TEST: is_markdown_file()
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_markdown_file_with_md_extension() {
        // Test lowercase .md
        assert!(is_markdown_file(Path::new("README.md")));

        // Test uppercase .MD
        assert!(is_markdown_file(Path::new("README.MD")));

        // Test mixed case .Md
        assert!(is_markdown_file(Path::new("README.Md")));
    }

    #[test]
    fn test_is_markdown_file_with_markdown_extension() {
        // Test .markdown extension
        assert!(is_markdown_file(Path::new("docs/index.markdown")));
        assert!(is_markdown_file(Path::new("docs/index.MARKDOWN")));
    }

    #[test]
    fn test_is_markdown_file_with_invalid_extension() {
        // Test .txt extension
        assert!(!is_markdown_file(Path::new("data.txt")));

        // Test .rs extension
        assert!(!is_markdown_file(Path::new("main.rs")));

        // Test no extension
        assert!(!is_markdown_file(Path::new("README")));
    }

    #[test]
    fn test_is_markdown_file_with_nested_paths() {
        // Test nested directory paths
        assert!(is_markdown_file(Path::new("docs/guides/tutorial.md")));
        assert!(is_markdown_file(Path::new("a/b/c/d/e/file.md")));
    }

    // -------------------------------------------------------------------------
    // TEST: read_markdown_file()
    // -------------------------------------------------------------------------

    #[test]
    fn test_read_markdown_file_success() {
        // Create a temporary markdown file
        let content = "# Hello World\n\nThis is a test markdown file.";
        let file_path = create_temp_markdown_file(content);

        // Test reading the file
        let result = read_markdown_file(&file_path);

        // Assert that reading was successful
        assert!(result.is_ok(), "Should successfully read markdown file");

        // Assert that the content matches
        assert_eq!(result.unwrap(), content);

        // Clean up the temporary file
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_read_markdown_file_not_found() {
        // Try to read a file that doesn't exist
        let result = read_markdown_file("nonexistent_file.md");

        // Assert that it returns an error
        assert!(result.is_err(), "Should return error for non-existent file");

        // Assert that it's the correct error type
        match result {
            Err(MarkdownError::FileNotFound(_)) => (),
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_read_markdown_file_invalid_extension() {
        // Create a temporary text file (not markdown)
        let content = "This is a text file.";
        let file_path = create_temp_text_file(content);

        // Try to read it as a markdown file
        let result = read_markdown_file(&file_path);

        // Assert that it returns an error
        assert!(result.is_err(), "Should return error for invalid extension");

        // Assert that it's the correct error type
        match result {
            Err(MarkdownError::InvalidExtension(_, ext)) => {
                assert_eq!(ext, "txt");
            }
            _ => panic!("Expected InvalidExtension error"),
        }

        // Clean up
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_read_markdown_file_empty_file() {
        // Create an empty markdown file
        let file_path = create_temp_markdown_file("");

        // Test reading the empty file
        let result = read_markdown_file(&file_path);

        // Assert that reading was successful
        assert!(
            result.is_ok(),
            "Should successfully read empty markdown file"
        );

        // Assert that the content is empty
        assert_eq!(result.unwrap(), "");

        // Clean up
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_read_markdown_file_with_unicode() {
        // Create a markdown file with unicode content
        let content = "# 测试\n\nThis has émojis 🦀 and üñíçödé";
        let file_path = create_temp_markdown_file(content);

        // Test reading the file
        let result = read_markdown_file(&file_path);

        // Assert that reading was successful
        assert!(
            result.is_ok(),
            "Should successfully read unicode markdown file"
        );

        // Assert that the content matches (unicode preserved)
        assert_eq!(result.unwrap(), content);

        // Clean up
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_read_markdown_file_with_multiline() {
        // Create a markdown file with multiple lines
        let content = "# Title\n\n## Subtitle\n\n- Item 1\n- Item 2\n- Item 3";
        let file_path = create_temp_markdown_file(content);

        // Test reading the file
        let result = read_markdown_file(&file_path);

        // Assert that reading was successful
        assert!(
            result.is_ok(),
            "Should successfully read multiline markdown file"
        );

        // Assert that the content matches
        assert_eq!(result.unwrap(), content);

        // Clean up
        fs::remove_file(&file_path).ok();
    }

    #[test]
    fn test_read_markdown_file_directory() {
        // Create a temporary directory
        let temp_dir = std::env::temp_dir();
        let dir_path = temp_dir.join(format!(
            "test_dir_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir_path).expect("Failed to create temp directory");

        // Try to read the directory as a file
        let result = read_markdown_file(dir_path.to_str().unwrap());

        // Assert that it returns an error
        assert!(result.is_err(), "Should return error for directory");

        // Assert that it's the correct error type
        match result {
            Err(MarkdownError::NotAFile(_)) => (),
            _ => panic!("Expected NotAFile error"),
        }

        // Clean up
        fs::remove_dir(&dir_path).ok();
    }
}
