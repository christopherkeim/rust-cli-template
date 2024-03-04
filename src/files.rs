use std::fmt;
use std::fs::File;
use std::io::Write;

/// Errors that occur during the process of writing a markdown file
pub enum MarkdownFileError {
    /// Empty markdown string
    EmptyMarkdown,
    /// Empty output file name string
    EmptyFileName,
    /// Error creating file
    Create(std::io::Error),
    /// Error writing markdown to file
    Write(std::io::Error),
}

impl fmt::Display for MarkdownFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MarkdownFileError::EmptyMarkdown => {
                write!(f, "Empty string passed as `markdown`.")
            }
            MarkdownFileError::EmptyFileName => {
                write!(f, "Empty string passed as `file_name`.")
            }
            MarkdownFileError::Create(ref err) => err.fmt(f),
            MarkdownFileError::Write(ref err) => err.fmt(f),
        }
    }
}

/// Writes a markdown file to local disk.
///
/// # Arguments
/// `markdown: String` a string holding markdown
/// `file_name: String` name of output file
pub fn write_markdown_file(
    markdown: String,
    file_name: &mut String,
) -> Result<(), MarkdownFileError> {
    if markdown.is_empty() {
        return Err(MarkdownFileError::EmptyMarkdown);
    }

    if file_name.is_empty() {
        return Err(MarkdownFileError::EmptyFileName);
    }

    if !file_name.ends_with(".md") {
        file_name.extend(".md".chars());
    }

    let mut md_file: File = match File::create(file_name) {
        Ok(md_file) => md_file,
        Err(file_creation_error) => {
            return Err(MarkdownFileError::Create(file_creation_error));
        }
    };

    match md_file.write_all(markdown.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(file_write_error) => {
            return Err(MarkdownFileError::Write(file_write_error));
        }
    };
}

/// `files` module unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use std::path::Path;
    use tempfile::tempdir;

    /// Empty markdown string
    #[test]
    fn empty_markdown_returns_early_with_error() {
        let result: String = write_markdown_file("".to_string(), &mut "testing.md".to_string())
            .unwrap_err()
            .to_string();

        let expected_error: String = "Empty string passed as `markdown`.".to_string();

        assert_eq!(result, expected_error);
    }

    #[test]
    /// Empty file name string
    fn invalid_file_name_returns_early_with_error() {
        let result: String =
            write_markdown_file("# Hello testing\n".to_string(), &mut "".to_string())
                .unwrap_err()
                .to_string();

        let expected_error = "Empty string passed as `file_name`.".to_string();

        assert_eq!(result, expected_error);
    }

    #[test]
    fn valid_markdown_and_file_name_successfully_write_to_file() {
        let test_directory: tempfile::TempDir =
            tempdir().expect("Failed to create test directory!");

        let mut file_name: String = test_directory
            .path()
            .to_str()
            .expect("Failed to extract test directory path!")
            .to_string();

        file_name.extend("/test_md_file.md".chars());

        let markdown: String = "# Hello from this file!".to_string();

        let result: String = match write_markdown_file(markdown, &mut file_name) {
            Ok(_) => "Success!".to_string(),
            Err(file_error) => format!("Error: {}", file_error.to_string()),
        };

        assert_eq!(result, "Success!".to_string());

        // Technically we're testing read here, but I like to know
        let md_file_contents = match read_to_string(Path::new(&file_name)) {
            Ok(contents) => contents,
            Err(read_error) => format!("Error: {}", read_error),
        };

        assert_eq!(md_file_contents, "# Hello from this file!".to_string());
    }
}
