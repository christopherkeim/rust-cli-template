use std::fmt;
use std::fs::File;
use std::io::Write;

/// Errors that occur during the process of writing a markdown file
pub enum MarkdownFileError {
    /// Empty markdown string
    Empty,
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
            MarkdownFileError::Empty => {
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

/// Writes a markdown file to local storage.
///
/// # Arguments
/// `markdown: String` a string holding markdown
/// `file_name: String` name of output file
pub fn write_markdown_file(markdown: String, file_name: &str) -> Result<(), MarkdownFileError> {
    if markdown.is_empty() {
        return Err(MarkdownFileError::Empty);
    }

    if file_name.is_empty() {
        return Err(MarkdownFileError::EmptyFileName);
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
