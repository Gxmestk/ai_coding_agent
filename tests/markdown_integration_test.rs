//! # Integration Tests for Markdown Reader
//!
//! This test file contains integration tests that verify the end-to-end
//! functionality of the AI Coding Agent's markdown reader feature.
//!
//! These tests run the compiled binary and verify:
//! - CLI argument parsing
//! - File reading and validation
//! - Error handling and user messages
//! - Help text display

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// Helper function to get the path to the ai_coding_agent binary
fn get_binary_path() -> PathBuf {
    // The binary is located in target/debug/ai_coding_agent
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove the test binary name
    if path.ends_with("deps") {
        path.pop(); // Remove deps
    }
    path.push("ai_coding_agent");
    path
}

/// Helper function to create a temporary markdown file
/// Returns the path to the created file
fn create_temp_markdown_file(name: &str, content: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_{}_{}.md",
        name,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    let mut file = fs::File::create(&file_path).expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");

    file_path
}

/// Helper function to create a temporary non-markdown file
fn create_temp_text_file(name: &str, content: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_{}_{}.txt",
        name,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    let mut file = fs::File::create(&file_path).expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");

    file_path
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_read_valid_markdown_file() {
    // Create a test markdown file
    let content = r#"# Test File

This is a test markdown file.

## Section 1

Some content here.

## Section 2

More content with **bold** and *italic* text.
"#;

    let file_path = create_temp_markdown_file("valid", content);

    // Run the binary with the markdown file
    let output = Command::new(get_binary_path())
        .arg(&file_path)
        .output()
        .expect("Failed to execute process");

    // Clean up
    fs::remove_file(&file_path).ok();

    // Verify the command succeeded
    assert!(
        output.status.success(),
        "Command failed with status: {:?}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify the output contains the file content
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Test File"));
    assert!(stdout.contains("This is a test markdown file."));
    assert!(stdout.contains("**bold**"));
}

#[test]
fn test_read_markdown_file_with_unicode() {
    // Create a markdown file with unicode content
    let content = "# 测试\n\nThis has émojis 🦀 and üñíçödé characters.";
    let file_path = create_temp_markdown_file("unicode", content);

    // Run the binary
    let output = Command::new(get_binary_path())
        .arg(&file_path)
        .output()
        .expect("Failed to execute process");

    // Clean up
    fs::remove_file(&file_path).ok();

    // Verify the command succeeded
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify unicode is preserved
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("测试"));
    assert!(stdout.contains("🦀"));
    assert!(stdout.contains("üñíçödé"));
}

#[test]
fn test_read_empty_markdown_file() {
    // Create an empty markdown file
    let file_path = create_temp_markdown_file("empty", "");

    // Run the binary
    let output = Command::new(get_binary_path())
        .arg(&file_path)
        .output()
        .expect("Failed to execute process");

    // Clean up
    fs::remove_file(&file_path).ok();

    // Verify the command succeeded
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify output is empty (or just whitespace)
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.trim().is_empty(),
        "Expected empty output, got: {}",
        stdout
    );
}

#[test]
fn test_file_not_found_error() {
    // Try to read a non-existent file
    let output = Command::new(get_binary_path())
        .arg("nonexistent_file_12345.md")
        .output()
        .expect("Failed to execute process");

    // Verify the command failed
    assert!(!output.status.success(), "Command should have failed");

    // Verify error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("File not found"));
    assert!(stderr.contains("nonexistent_file_12345.md"));

    // Verify helpful hint is provided
    assert!(
        stderr.contains("Hint:"),
        "Expected helpful hint in error message"
    );
    assert!(
        stderr.contains("Make sure the file path is correct"),
        "Expected file path hint"
    );
}

#[test]
fn test_invalid_extension_error() {
    // Create a non-markdown file
    let content = "This is a plain text file.";
    let file_path = create_temp_text_file("invalid_ext", content);

    // Try to read it as a markdown file
    let output = Command::new(get_binary_path())
        .arg(&file_path)
        .output()
        .expect("Failed to execute process");

    // Clean up
    fs::remove_file(&file_path).ok();

    // Verify the command failed
    assert!(!output.status.success(), "Command should have failed");

    // Verify error message about invalid extension
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid extension"));
    assert!(stderr.contains("txt"));

    // Verify helpful hint about .md extension
    assert!(
        stderr.contains("Hint:"),
        "Expected helpful hint in error message"
    );
    assert!(
        stderr.contains(".md or .markdown"),
        "Expected hint about .md extension"
    );
}

#[test]
fn test_no_arguments_provided() {
    // Run the binary with no arguments
    let output = Command::new(get_binary_path())
        .output()
        .expect("Failed to execute process");

    // Verify the command failed
    assert!(!output.status.success(), "Command should have failed");

    // Verify error message about no arguments
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No arguments provided") || stderr.contains("Invalid arguments"));

    // Verify hint about using --help
    assert!(
        stderr.contains("--help"),
        "Expected hint about using --help"
    );
}

#[test]
fn test_help_flag() {
    // Test --help flag
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute process");

    // Verify the command succeeded
    assert!(
        output.status.success(),
        "Help command should succeed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify help text is displayed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("ai_coding_agent"));
    assert!(stdout.contains("<markdown_file>"));
    assert!(stdout.contains("--help"));
    assert!(stdout.contains("EXAMPLES:"));
}

#[test]
fn test_short_help_flag() {
    // Test -h flag
    let output = Command::new(get_binary_path())
        .arg("-h")
        .output()
        .expect("Failed to execute process");

    // Verify the command succeeded
    assert!(
        output.status.success(),
        "Help command should succeed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify help text is displayed
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("ai_coding_agent"));
}

#[test]
fn test_read_markdown_file_with_multiline_content() {
    // Create a markdown file with complex multiline content
    let content = r#"# Complex Document

## Introduction

This document has multiple sections and complex formatting.

## Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```

## List

- Item 1
- Item 2
  - Nested item
- Item 3

## Table

| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |

End of document.
"#;

    let file_path = create_temp_markdown_file("multiline", content);

    // Run the binary
    let output = Command::new(get_binary_path())
        .arg(&file_path)
        .output()
        .expect("Failed to execute process");

    // Clean up
    fs::remove_file(&file_path).ok();

    // Verify the command succeeded
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify all content is preserved
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Complex Document"));
    assert!(stdout.contains("fn main() {"));
    assert!(stdout.contains("- Nested item"));
    assert!(stdout.contains("| Header 1 |"));
}

#[test]
fn test_read_markdown_with_markdown_extension() {
    // Test with .markdown extension (alternative to .md)
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "test_{}.markdown",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    let content = "# Test\n\nContent here.";
    let mut file = fs::File::create(&file_path).expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");

    // Run the binary
    let output = Command::new(get_binary_path())
        .arg(&file_path)
        .output()
        .expect("Failed to execute process");

    // Clean up
    fs::remove_file(&file_path).ok();

    // Verify the command succeeded
    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify content is read
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Test"));
}
