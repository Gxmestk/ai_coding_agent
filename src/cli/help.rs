//! # Help Display Module
//!
//! This module provides functionality to display help information to users.
//! It includes formatted help text that explains how to use the program,
//! available options, and usage examples.

/// Displays the main help information for the markdown reader CLI
///
/// This function prints a formatted help message to stdout that includes:
/// - Program name and description
/// - Usage syntax
/// - Arguments description
/// - Examples of common usage
///
/// # Example
///
/// ```no_run
/// use ai_coding_agent::cli::help::display_help;
///
/// fn main() {
///     display_help();
/// }
/// ```
///
/// This will output:
/// ```text
/// AI Coding Agent - Markdown Reader v0.1.0
///
/// USAGE:
///     ai_coding_agent <markdown_file>
///
/// ARGUMENTS:
///     <markdown_file>    Path to the markdown file to read
///                        Must have a .md or .markdown extension
///
/// OPTIONS:
///     -h, --help         Display this help message
///
/// EXAMPLES:
///     Read a markdown file:
///         $ ai_coding_agent README.md
///
///     Read a file in a subdirectory:
///         $ ai_coding_agent docs/guide.md
///
///     Show help:
///         $ ai_coding_agent --help
/// ```
pub fn display_help() {
    println!(
        r#"AI Coding Agent - Markdown Reader v0.1.0

USAGE:
    ai_coding_agent <markdown_file>

ARGUMENTS:
    <markdown_file>    Path to the markdown file to read
                       Must have a .md or .markdown extension

OPTIONS:
    -h, --help         Display this help message

EXAMPLES:
    Read a markdown file:
        $ ai_coding_agent README.md

    Read a file in a subdirectory:
        $ ai_coding_agent docs/guide.md

    Show help:
        $ ai_coding_agent --help
"#
    );
}

/// Displays an error message for invalid usage
///
/// This function is called when the user provides incorrect arguments.
/// It shows the error message and then displays the help information.
///
/// # Arguments
///
/// * `message` - A string slice that contains the error message to display
///
/// # Example
///
/// ```no_run
/// use ai_coding_agent::cli::help::display_usage_error;
///
/// fn main() {
///     display_usage_error("No file path provided");
/// }
/// ```
///
/// This will output:
/// ```text
/// Error: No file path provided
///
/// Use 'ai_coding_agent --help' for more information.
/// ```
pub fn display_usage_error(message: &str) {
    eprintln!("Error: {}", message);
    eprintln!();
    eprintln!("Use 'ai_coding_agent --help' for more information.");
}
